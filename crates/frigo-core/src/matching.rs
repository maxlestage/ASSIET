//! Le filtrage et le classement : ce que j'ai, ce qui manque, combien de temps.

use serde::{Deserialize, Serialize};

use crate::model::Meal;
use crate::normalize::{is_pantry, same_ingredient};
use crate::time::estimate_minutes;

/// Comment choisir les recettes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Criteria {
    /// Seulement les recettes faisables avec ce que j'ai (rien ne manque).
    pub strict: bool,
    /// Le sel, le poivre, l'huile, l'eau et le sucre sont considérés comme disponibles.
    pub pantry: bool,
    /// Au plus ce nombre d'ingrédients manquants (ignoré avec `strict`).
    pub max_missing: Option<usize>,
    /// Au plus ce nombre de minutes (estimées).
    pub max_minutes: Option<u32>,
    /// Une catégorie (« Chicken », « Vegetarian »…), sans tenir compte de la casse.
    pub category: Option<String>,
    /// Une cuisine (« French », « Italian »…), sans tenir compte de la casse.
    pub area: Option<String>,
}

impl Default for Criteria {
    fn default() -> Criteria {
        Criteria {
            strict: false,
            pantry: true,
            max_missing: None,
            max_minutes: None,
            category: None,
            area: None,
        }
    }
}

/// Une recette, comparée au contenu du frigo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub meal: Meal,
    /// Les ingrédients de la recette que j'ai (ou qui sont au placard).
    pub have: Vec<String>,
    /// Les ingrédients de la recette qui me manquent.
    pub missing: Vec<String>,
    /// Les ingrédients de mon frigo que la recette utilise.
    pub used: Vec<String>,
    /// Le temps estimé, en minutes (TheMealDB ne le donne pas : il est lu dans les
    /// instructions).
    pub minutes: u32,
}

impl Match {
    /// La part des ingrédients de la recette que j'ai, de 0 à 1.
    pub fn coverage(&self) -> f32 {
        let total = self.have.len() + self.missing.len();
        if total == 0 {
            0.0
        } else {
            self.have.len() as f32 / total as f32
        }
    }

    /// Faisable tout de suite : rien ne manque.
    pub fn complete(&self) -> bool {
        self.missing.is_empty()
    }
}

/// Compare une recette au frigo.
pub fn evaluate(meal: &Meal, fridge: &[String], criteria: &Criteria) -> Match {
    let mut have = Vec::new();
    let mut missing = Vec::new();
    let mut used = Vec::new();
    for ingredient in &meal.ingredients {
        let mine = fridge.iter().find(|f| same_ingredient(f, &ingredient.name));
        match mine {
            Some(f) => {
                have.push(ingredient.name.clone());
                if !used.contains(f) {
                    used.push(f.clone());
                }
            }
            None if criteria.pantry && is_pantry(&ingredient.name) => {
                have.push(ingredient.name.clone())
            }
            None => missing.push(ingredient.name.clone()),
        }
    }
    Match {
        minutes: estimate_minutes(&meal.instructions, meal.ingredients.len()),
        meal: meal.clone(),
        have,
        missing,
        used,
    }
}

fn same_text(a: &Option<String>, b: &Option<String>) -> bool {
    match (a, b) {
        (_, None) => true,
        (Some(a), Some(b)) => a.eq_ignore_ascii_case(b.trim()),
        (None, Some(_)) => false,
    }
}

/// Garde les recettes qui utilisent au moins un ingrédient du frigo et qui respectent
/// les critères, de la meilleure à la moins bonne : d'abord celles où il manque le moins
/// de choses, puis celles qui utilisent le plus le frigo, puis les plus rapides.
pub fn rank(meals: &[Meal], fridge: &[String], criteria: &Criteria) -> Vec<Match> {
    let mut matches: Vec<Match> = meals
        .iter()
        .filter(|m| same_text(&m.category, &criteria.category))
        .filter(|m| same_text(&m.area, &criteria.area))
        .map(|m| evaluate(m, fridge, criteria))
        .filter(|m| !m.used.is_empty())
        .filter(|m| !criteria.strict || m.complete())
        .filter(|m| {
            criteria.strict
                || criteria
                    .max_missing
                    .is_none_or(|max| m.missing.len() <= max)
        })
        .filter(|m| criteria.max_minutes.is_none_or(|max| m.minutes <= max))
        .collect();
    matches.sort_by(|a, b| {
        a.missing
            .len()
            .cmp(&b.missing.len())
            .then(b.used.len().cmp(&a.used.len()))
            .then(a.minutes.cmp(&b.minutes))
            .then(a.meal.name.cmp(&b.meal.name))
    });
    // Une recette peut arriver deux fois (deux ingrédients du frigo la trouvent).
    matches.dedup_by(|a, b| a.meal.id == b.meal.id);
    matches
}
