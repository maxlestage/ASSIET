//! `/api/recettes?i=poulet,riz&strict=1` : les recettes du frigo, en JSON, pour la démo.
//! `/vignette/{fichier}` : la photo d'une recette, relayée depuis TheMealDB.

use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, Instant};

use frigo_core::{Criteria, Match, MealDb, parse_list};
use paquets::prelude::*;

use crate::islands::{Answer, Card};

/// Le nombre de recettes envoyées à la démo.
const CARDS: usize = 6;
/// Le nombre d'ingrédients au plus (l'API est interrogée une fois par ingrédient).
const MAX_ITEMS: usize = 12;
/// Combien de temps une réponse est gardée.
const TTL: Duration = Duration::from_secs(15 * 60);

type Cache = Mutex<HashMap<String, (Instant, Arc<Answer>)>>;

static CACHE: LazyLock<Cache> = LazyLock::new(|| Mutex::new(HashMap::new()));
static MEALDB: LazyLock<MealDb> = LazyLock::new(|| {
    let mut db = MealDb::new(
        std::env::var("FRIGO_API").unwrap_or_else(|_| frigo_core::DEFAULT_BASE_URL.into()),
    );
    db.max_details = 24;
    db
});
static IMAGES: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap_or_default()
});

pub fn mount(app: &mut App) {
    app.get("/api/recettes", |req: Request| async move {
        recettes(&req).await
    });
    app.get("/vignette/:file", |req: Request| async move {
        vignette(req.param("file").unwrap_or("")).await
    });
}

/// Une recette de `frigo-core`, pour la carte de la démo.
pub fn card(m: &Match) -> Card {
    Card {
        id: m.meal.id.clone(),
        name: m.meal.name.clone(),
        thumb: m
            .meal
            .thumbnail
            .as_deref()
            .and_then(thumb_file)
            .map(|f| format!("/vignette/{f}")),
        minutes: m.minutes,
        category: m.meal.category.clone(),
        area: m.meal.area.clone(),
        have: m.have.clone(),
        missing: m.missing.clone(),
        coverage: (m.coverage() * 100.0).round() as u32,
        steps: m
            .meal
            .instructions
            .lines()
            .map(str::trim)
            .filter(|l| l.len() > 12)
            .map(str::to_owned)
            .collect(),
        youtube: m.meal.youtube.clone().filter(|u| u.starts_with("https://")),
    }
}

/// `https://www.themealdb.com/images/media/meals/abc123.jpg` → `abc123.jpg`.
fn thumb_file(url: &str) -> Option<String> {
    let file = url.strip_prefix("https://www.themealdb.com/images/media/meals/")?;
    valid_file(file).then(|| file.to_owned())
}

fn valid_file(file: &str) -> bool {
    let Some((stem, ext)) = file.rsplit_once('.') else {
        return false;
    };
    !stem.is_empty()
        && stem.len() <= 64
        && stem
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        && matches!(ext, "jpg" | "jpeg" | "png" | "webp")
}

async fn recettes(req: &Request) -> Response {
    let fridge: Vec<String> = parse_list(&req.query("i").unwrap_or_default())
        .into_iter()
        .filter(|i| i.chars().count() <= 40)
        .take(MAX_ITEMS)
        .collect();
    let strict = matches!(req.query("strict").as_deref(), Some("1" | "true" | "oui"));
    if fridge.is_empty() {
        return Response::new()
            .json(Answer {
                error: Some("Le frigo est vide : ajoutez au moins un ingrédient.".into()),
                ..Answer::default()
            })
            .status(400);
    }

    let mut key: Vec<String> = fridge.iter().map(|i| frigo_core::canonical(i)).collect();
    key.sort();
    let key = format!("{}|{strict}", key.join(","));
    if let Some(answer) = cached(&key) {
        return Response::new().json(&*answer).header("x-cache", "hit");
    }

    let criteria = Criteria {
        strict,
        ..Criteria::default()
    };
    match MEALDB.suggest(&fridge, &criteria).await {
        Ok(matches) => {
            let answer = Arc::new(Answer {
                fridge,
                cards: matches.iter().take(CARDS).map(card).collect(),
                error: None,
            });
            if let Ok(mut cache) = CACHE.lock() {
                cache.retain(|_, (at, _)| at.elapsed() < TTL);
                cache.insert(key, (Instant::now(), answer.clone()));
            }
            Response::new().json(&*answer)
        }
        Err(e) => Response::new()
            .json(Answer {
                fridge,
                cards: Vec::new(),
                error: Some(format!("TheMealDB ne répond pas pour l'instant ({e}).")),
            })
            .status(502),
    }
}

fn cached(key: &str) -> Option<Arc<Answer>> {
    let cache = CACHE.lock().ok()?;
    let (at, answer) = cache.get(key)?;
    (at.elapsed() < TTL).then(|| answer.clone())
}

async fn vignette(file: &str) -> Response {
    if !valid_file(file) {
        return Response::new().status(404).send("introuvable");
    }
    let url = format!("https://www.themealdb.com/images/media/meals/{file}/small");
    let Ok(response) = IMAGES
        .get(&url)
        .send()
        .await
        .and_then(|r| r.error_for_status())
    else {
        return Response::new().status(502).send("image indisponible");
    };
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .filter(|v| v.starts_with("image/"))
        .unwrap_or("image/jpeg")
        .to_owned();
    match response.bytes().await {
        Ok(bytes) => Response::new()
            .header("content-type", content_type)
            .header("cache-control", "public, max-age=604800, immutable")
            .send(bytes.to_vec()),
        Err(_) => Response::new().status(502).send("image indisponible"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_themealdb_files_are_relayed() {
        assert_eq!(
            thumb_file("https://www.themealdb.com/images/media/meals/wvpsxx1468256321.jpg")
                .as_deref(),
            Some("wvpsxx1468256321.jpg")
        );
        assert!(thumb_file("https://evil.example/x.jpg").is_none());
        assert!(!valid_file("../../etc/passwd"));
        assert!(!valid_file("a.svg"));
    }
}
