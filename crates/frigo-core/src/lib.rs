//! # frigo-core
//!
//! Le cœur de **Frigo vide** : que cuisiner avec ce qu'il reste dans le frigo ?
//!
//! ```no_run
//! # async fn demo() -> Result<(), frigo_core::Error> {
//! use frigo_core::{Criteria, MealDb};
//!
//! let fridge = vec!["poulet".to_string(), "riz".into(), "oignon".into()];
//! let matches = MealDb::default().suggest(&fridge, &Criteria::default()).await?;
//! if let Some(best) = matches.first() {
//!     println!("{} — {} min, il manque {:?}", best.meal.name, best.minutes, best.missing);
//! }
//! # Ok(()) }
//! ```
//!
//! Sans la feature `http`, il ne reste que la logique pure (pas de réseau) :
//! [`normalize`], [`same_ingredient`], [`rank`], [`estimate_minutes`].

mod matching;
mod model;
mod normalize;
mod time;

#[cfg(feature = "http")]
mod client;

pub use matching::{Criteria, Match, evaluate, rank};
pub use model::{Ingredient, Meal, MealSummary, parse_meals, parse_summaries};
pub use normalize::{
    PANTRY, api_name, canonical, is_pantry, normalize, parse_list, same_ingredient,
};
pub use time::estimate_minutes;

#[cfg(feature = "http")]
pub use client::{DEFAULT_BASE_URL, Error, MealDb};
