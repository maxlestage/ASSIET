//! Les données de TheMealDB, telles que l'API les envoie, et sous une forme propre.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Un ingrédient d'une recette, avec sa quantité (« 2 cups », « 1 pinch »…).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub measure: String,
}

/// Une recette complète (réponse de `lookup.php` ou `search.php`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meal {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub area: Option<String>,
    pub instructions: String,
    pub thumbnail: Option<String>,
    pub youtube: Option<String>,
    pub source: Option<String>,
    pub tags: Vec<String>,
    pub ingredients: Vec<Ingredient>,
}

/// Le résumé d'une recette (réponse de `filter.php`) : seulement l'identifiant, le nom
/// et la photo.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MealSummary {
    #[serde(rename = "idMeal")]
    pub id: String,
    #[serde(rename = "strMeal")]
    pub name: String,
    #[serde(rename = "strMealThumb", default)]
    pub thumbnail: Option<String>,
}

/// L'enveloppe de toutes les réponses : `{"meals": [...]}`, ou `{"meals": null}` quand
/// rien ne correspond (et parfois `"meals": "no data found"`).
#[derive(Debug, Deserialize)]
#[serde(bound = "T: serde::de::DeserializeOwned")]
struct Envelope<T> {
    #[serde(default = "Vec::new", deserialize_with = "lenient_list")]
    meals: Vec<T>,
}

fn lenient_list<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    match Value::deserialize(deserializer)? {
        Value::Array(items) => items
            .into_iter()
            .map(|v| serde_json::from_value(v).map_err(serde::de::Error::custom))
            .collect(),
        _ => Ok(Vec::new()),
    }
}

/// Une recette brute : les champs `strIngredient1` … `strIngredient20` et
/// `strMeasure1` … `strMeasure20` sont lus dynamiquement.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct RawMeal(Map<String, Value>);

impl RawMeal {
    fn text(&self, key: &str) -> Option<String> {
        self.0
            .get(key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
    }

    fn into_meal(self) -> Option<Meal> {
        let id = self.text("idMeal")?;
        let name = self.text("strMeal")?;
        let mut ingredients = Vec::new();
        // L'API en envoie 20, mais rien n'empêche d'en lire plus si elle évolue.
        for i in 1.. {
            let key = format!("strIngredient{i}");
            if !self.0.contains_key(&key) {
                break;
            }
            if let Some(ingredient) = self.text(&key) {
                ingredients.push(Ingredient {
                    name: ingredient,
                    measure: self.text(&format!("strMeasure{i}")).unwrap_or_default(),
                });
            }
        }
        Some(Meal {
            id,
            name,
            category: self.text("strCategory"),
            area: self.text("strArea").or_else(|| self.text("strCountry")),
            instructions: self
                .text("strInstructions")
                .unwrap_or_default()
                .replace("\r\n", "\n"),
            thumbnail: self.text("strMealThumb"),
            youtube: self.text("strYoutube"),
            source: self.text("strSource"),
            tags: self
                .text("strTags")
                .map(|t| {
                    t.split(',')
                        .map(str::trim)
                        .filter(|t| !t.is_empty())
                        .map(str::to_owned)
                        .collect()
                })
                .unwrap_or_default(),
            ingredients,
        })
    }
}

/// Lit une réponse de `lookup.php` / `search.php`.
pub fn parse_meals(json: &str) -> Result<Vec<Meal>, serde_json::Error> {
    let envelope: Envelope<RawMeal> = serde_json::from_str(json)?;
    Ok(envelope
        .meals
        .into_iter()
        .filter_map(RawMeal::into_meal)
        .collect())
}

/// Lit une réponse de `filter.php`.
pub fn parse_summaries(json: &str) -> Result<Vec<MealSummary>, serde_json::Error> {
    let envelope: Envelope<MealSummary> = serde_json::from_str(json)?;
    Ok(envelope.meals)
}
