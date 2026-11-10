use clap::Parser;
use frigo_core::{Criteria, DEFAULT_BASE_URL};

/// Frigo vide : la recette la plus rapide avec ce qu'il reste dans le frigo.
///
/// Les ingrédients s'écrivent en français ou en anglais (« poulet », « chicken »).
/// Les recettes viennent de TheMealDB (gratuit, sans clé).
#[derive(Debug, Parser)]
#[command(name = "frigo", version, about, long_about)]
pub struct Cli {
    /// Les ingrédients du frigo (séparés par des espaces ou des virgules).
    pub ingredients: Vec<String>,

    /// Un fichier d'ingrédients (un par ligne ou séparés par des virgules ; `-` pour
    /// l'entrée standard). Sans ingrédients, `frigo.txt` est lu s'il existe.
    #[arg(short = 'f', long = "fichier", value_name = "FICHIER")]
    pub file: Option<String>,

    /// Seulement les recettes faisables avec ce que j'ai, sans rien acheter.
    #[arg(short, long)]
    pub strict: bool,

    /// Le sel, le poivre, l'huile, l'eau et le sucre ne sont pas considérés comme acquis.
    #[arg(long = "sans-placard")]
    pub no_pantry: bool,

    /// Au plus N ingrédients manquants.
    #[arg(short = 'm', long = "manque", value_name = "N")]
    pub max_missing: Option<usize>,

    /// Au plus N minutes (estimées à partir des instructions).
    #[arg(short = 't', long = "temps", value_name = "MINUTES")]
    pub max_minutes: Option<u32>,

    /// Une catégorie de TheMealDB : Chicken, Beef, Vegetarian, Dessert…
    #[arg(short = 'c', long = "categorie")]
    pub category: Option<String>,

    /// Une cuisine : French, Italian, Japanese…
    #[arg(short = 'a', long = "cuisine")]
    pub area: Option<String>,

    /// Le nombre de recettes affichées.
    #[arg(short = 'n', long = "nombre", default_value_t = 5)]
    pub count: usize,

    /// Le nombre de recettes dont le détail est téléchargé et comparé.
    #[arg(long = "details", default_value_t = 30)]
    pub details: usize,

    /// Le résultat en JSON.
    #[arg(long, conflicts_with = "tui")]
    pub json: bool,

    /// L'interface plein écran (lancée aussi sans ingrédients, dans un terminal).
    #[arg(long)]
    pub tui: bool,

    /// Sans couleurs.
    #[arg(long = "sans-couleur", env = "NO_COLOR", value_parser = clap::builder::FalseyValueParser::new())]
    pub no_color: bool,

    /// L'adresse de l'API.
    #[arg(long, env = "FRIGO_API", default_value = DEFAULT_BASE_URL, hide_default_value = true)]
    pub api: String,
}

impl Cli {
    pub fn criteria(&self) -> Criteria {
        Criteria {
            strict: self.strict,
            pantry: !self.no_pantry,
            max_missing: self.max_missing,
            max_minutes: self.max_minutes,
            category: self.category.clone(),
            area: self.area.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn the_command_line_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn options_become_criteria() {
        let cli =
            Cli::try_parse_from(["frigo", "poulet", "riz", "-s", "--sans-placard", "-t", "30"])
                .unwrap();
        assert_eq!(cli.ingredients, ["poulet", "riz"]);
        let c = cli.criteria();
        assert!(c.strict && !c.pantry);
        assert_eq!(c.max_minutes, Some(30));
    }
}
