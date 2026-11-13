//! `frigo-site export <dossier>` : le site en fichiers statiques, pour GitHub Pages.
//!
//! Chaque page vient du serveur lui-même, par le client de test de Paquets (`app.test()`),
//! donc les deux sont identiques. Ce qu'un hébergement statique ne sait pas faire est
//! remplacé : la page est marquée `data-mode="static"`, et la démo interroge alors
//! TheMealDB directement depuis le navigateur (au lieu de `/api/recettes`).

use std::io;
use std::path::{Path, PathBuf};

use super::{app, share, site_dir};

/// Écrit le site dans `out`, avec les liens derrière `base` (`""` ou `/ASSIET`).
/// Renvoie le nombre de fichiers écrits.
pub async fn export(out: &Path, base: &str) -> io::Result<usize> {
    let base = normalize_base(base);
    let out = std::path::absolute(out)?;
    // Paquets trouve `pkg/` (le bundle du navigateur) dans le dossier courant.
    let dir = std::path::absolute(site_dir())?;
    std::env::set_current_dir(&dir)?;
    // Les liens de partage et le QR code visent l'adresse publique du site : `SITE_URL`
    // (le workflow de Pages la donne), sinon celle de GitHub Pages.
    let url = std::env::var("SITE_URL")
        .ok()
        .and_then(|u| share::normalize_url(&u))
        .unwrap_or_else(|| share::DEFAULT_URL.to_owned());
    let _ = share::PUBLIC_URL.set(url);
    if !dir.join("pkg").is_dir() {
        eprintln!("attention : site/pkg manque, la page ne sera pas interactive (voir le README)");
    }
    let mut app = app();
    // Des fichiers statiques n'ont pas de serveur à protéger, et l'export demande toutes
    // les pages d'un coup (la limite de débit l'arrêterait).
    app.security(None);
    let client = app.test().await;

    let mut written = 0;
    let mut write = |path: &str, bytes: &[u8]| -> io::Result<()> {
        let file = out.join(path.trim_start_matches('/'));
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(file, bytes)?;
        written += 1;
        Ok(())
    };
    let get = |path: &'static str, expected: u16| {
        let client = &client;
        async move {
            let res = client.get(path).await;
            if res.status().as_u16() != expected {
                return Err(io::Error::other(format!(
                    "GET {path} : {} au lieu de {expected}",
                    res.status()
                )));
            }
            Ok(res.bytes().to_vec())
        }
    };

    let home = String::from_utf8_lossy(&get("/", 200).await?).into_owned();
    write("index.html", static_page(&home, &base).as_bytes())?;
    for (path, file) in [("/en/", "en/index.html"), ("/es/", "es/index.html")] {
        let page = String::from_utf8_lossy(&get(path, 200).await?).into_owned();
        write(file, static_page(&page, &base).as_bytes())?;
    }
    let missing = String::from_utf8_lossy(&get("/introuvable", 404).await?).into_owned();
    write("404.html", static_page(&missing, &base).as_bytes())?;

    // La PWA : le manifeste et le service worker listent des adresses absolues.
    for file in ["/manifest.webmanifest", "/sw.js"] {
        let body = String::from_utf8_lossy(&get(file, 200).await?).into_owned();
        write(file, with_base_everywhere(&body, &base).as_bytes())?;
    }
    write(
        "robots.txt",
        format!("User-agent: *\nAllow: {base}/\n").as_bytes(),
    )?;

    for folder in ["public", "pkg"] {
        let root = dir.join(folder);
        let prefix = if folder == "pkg" { "pkg/" } else { "" };
        for file in files(&root) {
            let relative = file.strip_prefix(&root).map_err(io::Error::other)?;
            let url = format!("{prefix}{}", relative.to_string_lossy().replace('\\', "/"));
            // Les fichiers de wasm-bindgen inutiles pour faire tourner le bundle.
            if url.ends_with(".gitignore") || url.ends_with("package.json") {
                continue;
            }
            write(&url, &std::fs::read(&file)?)?;
        }
    }
    // Les fichiers sont servis tels quels : pas de Jekyll sur GitHub Pages.
    write(".nojekyll", b"")?;
    Ok(written)
}

/// `"ASSIET/"` → `"/ASSIET"`, `"/"` → `""`.
pub fn normalize_base(base: &str) -> String {
    let base = base.trim().trim_matches('/');
    if base.is_empty() {
        String::new()
    } else {
        format!("/{base}")
    }
}

/// Une page du serveur, pour l'hébergement statique : marquée `data-mode="static"`, et
/// ses adresses absolues placées derrière `base`. Seuls les attributs qui commencent
/// par une adresse connue changent (pas les `"/>` des balises).
pub fn static_page(html: &str, base: &str) -> String {
    let mut html = html.replacen("class=\"site\"", "class=\"site\" data-mode=\"static\"", 1);
    if !base.is_empty() {
        for prefix in [
            "\"/\"",
            "\"/#",
            "\"/en/",
            "\"/es/",
            "\"/assets/",
            "\"/pkg/",
            "\"/paquets/",
            "\"/site.css",
            "\"/manifest.webmanifest\"",
            "\"/sw.js\"",
        ] {
            html = html.replace(prefix, &format!("\"{base}{}", &prefix[1..]));
        }
    }
    html
}

/// Toutes les adresses absolues (`"/…"`) d'un fichier JSON ou JavaScript, derrière `base`.
pub fn with_base_everywhere(text: &str, base: &str) -> String {
    if base.is_empty() {
        return text.to_owned();
    }
    text.replace("\"/", &format!("\"{base}/"))
}

/// Tous les fichiers sous `dir`.
fn files(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(files(&path));
        } else {
            found.push(path);
        }
    }
    found.sort();
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bases_are_normalized() {
        assert_eq!(normalize_base(""), "");
        assert_eq!(normalize_base("/"), "");
        assert_eq!(normalize_base("ASSIET/"), "/ASSIET");
        assert_eq!(normalize_base("/ASSIET"), "/ASSIET");
    }

    #[test]
    fn only_known_urls_move_behind_the_base() {
        let html = r#"<div class="site"><a href="/">x</a><a href="/#demo">d</a><img src="/assets/a.svg"/><link href="/site.css?v=1"/><script src="/pkg/x.js"></script></div>"#;
        let out = static_page(html, "/ASSIET");
        assert!(out.contains(r#"class="site" data-mode="static""#));
        assert!(out.contains(r#"href="/ASSIET/""#));
        assert!(out.contains(r#"href="/ASSIET/#demo""#));
        assert!(out.contains(r#"src="/ASSIET/assets/a.svg"/>"#));
        assert!(out.contains(r#"href="/ASSIET/site.css?v=1"/>"#));
        assert!(out.contains(r#"src="/ASSIET/pkg/x.js""#));
    }
}
