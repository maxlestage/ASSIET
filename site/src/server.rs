//! Le serveur : la page d'accueil, l'API de la démo (`/api/recettes`, qui utilise
//! `frigo-core`, le même cœur que la CLI) et les vignettes de TheMealDB servies par le
//! site (la page n'accepte que ses propres images).

mod api;
pub mod export;
mod home;
pub mod share;

use std::path::{Path, PathBuf};

use paquets::prelude::*;

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
    let v = css_version.clone();
    app.get("/", move |req: Request| {
        let site = home::Site::new(&v, share::public_url(&req));
        async move {
            home::page(&site)
                .into_response()
                .header("cache-control", "no-cache")
        }
    });
    api::mount(&mut app);
    app.get("/robots.txt", |_req: Request| async {
        Response::new()
            .header("content-type", "text/plain; charset=utf-8")
            .send("User-agent: *\nAllow: /\nDisallow: /api/\n")
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
        let site = home::Site::new(&v, share::public_url(&req));
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
