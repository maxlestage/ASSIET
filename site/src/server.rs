//! Le serveur : la vitrine de l'app iPhone, une page rendue d'avance. Le site ne cherche
//! rien lui-même : tout se passe dans l'app.

pub mod export;
mod home;
pub mod share;

use std::path::{Path, PathBuf};

use paquets::prelude::*;

use crate::i18n::Lang;

/// Le dossier `site/` : celui qui contient `public/` (le dossier courant, sinon celui de
/// la crate).
pub fn site_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("SITE_DIR") {
        return PathBuf::from(dir);
    }
    if Path::new("public").is_dir() {
        return PathBuf::from(".");
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// FNV-1a : assez pour distinguer deux versions d'un fichier (`?v=` des liens).
pub fn version(bytes: &[u8]) -> String {
    let h = bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3)
    });
    format!("{h:016x}")[..10].to_owned()
}

/// L'application, prête à tourner ou à tester.
pub fn app() -> App {
    let css = std::fs::read(site_dir().join("public/site.css")).unwrap_or_default();
    let css_version = version(&css);
    let mut app = App::new();
    // La page d'accueil en français à la racine, en anglais sous /en/, en espagnol sous /es/.
    for lang in Lang::ALL {
        let v = css_version.clone();
        app.get(lang.home(), move |req: Request| {
            let site = home::Site::new(&v, share::public_url(&req), lang);
            async move {
                home::page(&site)
                    .into_response()
                    .header("cache-control", "no-cache")
            }
        });
    }
    app.get("/robots.txt", |_req: Request| async {
        Response::new()
            .header("content-type", "text/plain; charset=utf-8")
            .send("User-agent: *\nAllow: /\n")
    });
    app.pwa(
        Pwa::new("Frigo vide")
            .short_name("Frigo vide")
            .description("La recette la plus rapide avec ce qu'il reste dans le frigo.")
            .id("/")
            .start_url("/")
            .scope("/")
            .display("standalone")
            .lang("fr")
            .theme_color("#fff8ec")
            .background_color("#fff8ec")
            .categories(["food", "lifestyle", "utilities"])
            .icon("/assets/icon-192.png", "192x192", "image/png")
            .icon("/assets/icon-512.png", "512x512", "image/png")
            .maskable_icon("/assets/icon-maskable-512.png", "512x512", "image/png")
            .icon("/assets/logo.svg", "any", "image/svg+xml"),
    );
    let v = css_version;
    app.fallback(move |req: Request| {
        let lang = Lang::from_path(req.path());
        let site = home::Site::new(&v, share::public_url(&req), lang);
        async move { home::not_found(&site).into_response().status(404) }
    });
    app
}

/// `cargo run` : le site, sur `PORT` ou 3000.
pub fn run() -> std::io::Result<()> {
    let dir = site_dir();
    if dir != Path::new(".") {
        std::env::set_current_dir(&dir)?;
    }
    app().run()
}
