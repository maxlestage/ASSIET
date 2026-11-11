//! Une recette de `frigo-core` devient une carte de la démo. Partagé par le serveur
//! (`/api/recettes`) et par le navigateur (le site statique de GitHub Pages, qui
//! interroge TheMealDB directement).

use frigo_core::Match;

use crate::islands::Card;

/// Le nombre de recettes montrées par la démo.
pub const CARDS: usize = 6;

/// La carte d'une recette ; `thumb` est l'adresse de sa photo (relayée ou directe).
pub fn card(m: &Match, thumb: Option<String>) -> Card {
    Card {
        id: m.meal.id.clone(),
        name: m.meal.name.clone(),
        thumb,
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
