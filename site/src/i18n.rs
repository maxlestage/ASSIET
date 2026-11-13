//! Les langues du site : le français (à la racine), l'anglais (`/en/`) et l'espagnol
//! (`/es/`). Chaque texte est écrit dans les trois langues à l'endroit où il sert :
//! `lang.t("Bonjour", "Hello", "Hola")`.

/// Une langue du site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    Fr,
    En,
    Es,
}

impl Lang {
    /// Toutes les langues, dans l'ordre du sélecteur.
    pub const ALL: [Lang; 3] = [Lang::Fr, Lang::En, Lang::Es];

    /// Le code de la langue (`fr`, `en`, `es`).
    pub fn code(self) -> &'static str {
        match self {
            Lang::Fr => "fr",
            Lang::En => "en",
            Lang::Es => "es",
        }
    }

    /// La langue d'un code (`fr` par défaut).
    pub fn from_code(code: &str) -> Self {
        match code
            .trim()
            .get(..2)
            .unwrap_or("")
            .to_ascii_lowercase()
            .as_str()
        {
            "en" => Lang::En,
            "es" => Lang::Es,
            _ => Lang::Fr,
        }
    }

    /// La langue d'un chemin : `/en/…` en anglais, `/es/…` en espagnol, le reste en
    /// français.
    pub fn from_path(path: &str) -> Self {
        let first = path.trim_start_matches('/').split('/').next().unwrap_or("");
        match first {
            "en" => Lang::En,
            "es" => Lang::Es,
            _ => Lang::Fr,
        }
    }

    /// Le préfixe des adresses : `""`, `"/en"` ou `"/es"`.
    pub fn prefix(self) -> &'static str {
        match self {
            Lang::Fr => "",
            Lang::En => "/en",
            Lang::Es => "/es",
        }
    }

    /// La page d'accueil dans cette langue : `/`, `/en/` ou `/es/`.
    pub fn home(self) -> &'static str {
        match self {
            Lang::Fr => "/",
            Lang::En => "/en/",
            Lang::Es => "/es/",
        }
    }

    /// Le nom de la langue, dans cette langue.
    pub fn name(self) -> &'static str {
        match self {
            Lang::Fr => "Français",
            Lang::En => "English",
            Lang::Es => "Español",
        }
    }

    /// La locale Open Graph.
    pub fn locale(self) -> &'static str {
        match self {
            Lang::Fr => "fr_FR",
            Lang::En => "en_US",
            Lang::Es => "es_ES",
        }
    }

    /// Le texte dans cette langue.
    pub fn t(self, fr: &'static str, en: &'static str, es: &'static str) -> &'static str {
        match self {
            Lang::Fr => fr,
            Lang::En => en,
            Lang::Es => es,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages_come_from_codes_and_paths() {
        assert_eq!(Lang::from_code("en-US"), Lang::En);
        assert_eq!(Lang::from_code("ES"), Lang::Es);
        assert_eq!(Lang::from_code(""), Lang::Fr);
        assert_eq!(Lang::from_path("/en/"), Lang::En);
        assert_eq!(Lang::from_path("/es/nulle-part"), Lang::Es);
        assert_eq!(Lang::from_path("/entree"), Lang::Fr);
        assert_eq!(Lang::En.t("a", "b", "c"), "b");
        assert_eq!(Lang::Es.home(), "/es/");
    }
}
