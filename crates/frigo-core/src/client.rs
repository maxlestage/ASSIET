//! Le client de TheMealDB (gratuit, clé publique `1`).
//!
//! L'API ne cherche que par *un* ingrédient à la fois (`filter.php?i=`) et ne renvoie
//! alors que des résumés. On cherche donc chaque ingrédient du frigo, on garde les
//! recettes trouvées le plus souvent, on lit leur détail (`lookup.php?i=`) en parallèle,
//! puis on les classe localement.

use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use futures::stream::{self, StreamExt, TryStreamExt};

use crate::matching::{Criteria, Match, rank};
use crate::model::{Meal, MealSummary, parse_meals, parse_summaries};
use crate::normalize::api_name;

/// L'adresse publique de l'API.
pub const DEFAULT_BASE_URL: &str = "https://www.themealdb.com/api/json/v1/1";

/// Une erreur du client.
#[derive(Debug)]
pub enum Error {
    /// Le réseau, ou un statut HTTP d'erreur.
    Http(reqwest::Error),
    /// Une réponse qui n'est pas le JSON attendu.
    Json(serde_json::Error),
    /// Le frigo est vide.
    EmptyFridge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "TheMealDB est injoignable : {e}"),
            Error::Json(e) => write!(f, "réponse inattendue de TheMealDB : {e}"),
            Error::EmptyFridge => f.write_str("le frigo est vide : donnez au moins un ingrédient"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::EmptyFridge => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Http(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

/// Le client.
#[derive(Clone, Debug)]
pub struct MealDb {
    http: reqwest::Client,
    base_url: String,
    /// Le nombre de recettes dont on lit le détail.
    pub max_details: usize,
    /// Le nombre de requêtes en même temps.
    pub concurrency: usize,
}

impl Default for MealDb {
    fn default() -> MealDb {
        MealDb::new(DEFAULT_BASE_URL)
    }
}

impl MealDb {
    /// Un client pour l'API à cette adresse (sans `/` final).
    pub fn new(base_url: impl Into<String>) -> MealDb {
        let http = reqwest::Client::builder()
            .user_agent(concat!("frigo-vide/", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_default();
        MealDb {
            http,
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            max_details: 30,
            concurrency: 8,
        }
    }

    async fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<String, Error> {
        let url = format!("{}/{path}", self.base_url);
        let response = self.http.get(url).query(query).send().await?;
        Ok(response.error_for_status()?.text().await?)
    }

    /// Les recettes qui contiennent cet ingrédient (en anglais ou en français).
    pub async fn by_ingredient(&self, ingredient: &str) -> Result<Vec<MealSummary>, Error> {
        let body = self
            .get("filter.php", &[("i", &api_name(ingredient))])
            .await?;
        Ok(parse_summaries(&body)?)
    }

    /// Le détail d'une recette.
    pub async fn lookup(&self, id: &str) -> Result<Option<Meal>, Error> {
        let body = self.get("lookup.php", &[("i", id)]).await?;
        Ok(parse_meals(&body)?.into_iter().next())
    }

    /// Les recettes dont le nom contient ce texte.
    pub async fn search(&self, name: &str) -> Result<Vec<Meal>, Error> {
        let body = self.get("search.php", &[("s", name)]).await?;
        Ok(parse_meals(&body)?)
    }

    /// Une recette au hasard.
    pub async fn random(&self) -> Result<Option<Meal>, Error> {
        let body = self.get("random.php", &[]).await?;
        Ok(parse_meals(&body)?.into_iter().next())
    }

    /// Les recettes possibles avec ce frigo, de la meilleure à la moins bonne.
    pub async fn suggest(
        &self,
        fridge: &[String],
        criteria: &Criteria,
    ) -> Result<Vec<Match>, Error> {
        if fridge.is_empty() {
            return Err(Error::EmptyFridge);
        }
        // 1. Chaque ingrédient, en parallèle : combien d'ingrédients du frigo trouvent
        //    chaque recette.
        let lists: Vec<Vec<MealSummary>> = stream::iter(fridge.to_vec())
            .map(|item| {
                let db = self.clone();
                async move { db.by_ingredient(&item).await }
            })
            .buffered(self.concurrency.max(1))
            .try_collect()
            .await?;
        let mut hits: HashMap<String, (usize, usize)> = HashMap::new();
        let mut order = 0;
        for list in lists {
            for summary in list {
                order += 1;
                let entry = hits.entry(summary.id).or_insert((0, order));
                entry.0 += 1;
            }
        }
        let mut candidates: Vec<(String, (usize, usize))> = hits.into_iter().collect();
        candidates.sort_by(|(_, (a, oa)), (_, (b, ob))| b.cmp(a).then(oa.cmp(ob)));
        candidates.truncate(self.max_details);

        // 2. Le détail des meilleures, en parallèle.
        let meals: Vec<Meal> = stream::iter(candidates.into_iter().map(|(id, _)| id))
            .map(|id| {
                let db = self.clone();
                async move { db.lookup(&id).await }
            })
            .buffer_unordered(self.concurrency.max(1))
            .try_filter_map(futures::future::ok)
            .try_collect()
            .await?;

        // 3. Le classement, en local.
        Ok(rank(&meals, fridge, criteria))
    }
}
