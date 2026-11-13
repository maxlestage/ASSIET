//! Partager le site : son adresse publique, le QR code de la carte de partage et les
//! liens vers les messageries et les réseaux (de simples liens, sans script ni traceur).

use std::sync::OnceLock;

use paquets::prelude::*;
use qrcodegen::{QrCode, QrCodeEcc};

/// L'adresse du site publié sur GitHub Pages, quand aucune autre n'est connue.
pub const DEFAULT_URL: &str = "https://maxlestage.github.io/ASSIET/";

/// L'adresse publique imposée (l'export statique la fixe avant de rendre les pages).
pub static PUBLIC_URL: OnceLock<String> = OnceLock::new();

/// Le texte qui accompagne le lien partagé.
pub const SHARE_TEXT: &str = "Le frigo est vide ? Pas l'assiette. Frigo vide trouve la meilleure recette avec ce qu'il te reste :";

/// L'adresse publique du site, avec un `/` final : celle imposée par l'export, sinon
/// `SITE_URL`, sinon celle que le visiteur a demandée (`Host`, derrière le proxy de Heroku
/// `X-Forwarded-Host` et `X-Forwarded-Proto`), sinon [`DEFAULT_URL`].
pub fn public_url(req: &Request) -> String {
    if let Some(url) = PUBLIC_URL.get() {
        return url.clone();
    }
    if let Some(url) = std::env::var("SITE_URL")
        .ok()
        .and_then(|u| normalize_url(&u))
    {
        return url;
    }
    let host = req
        .header("x-forwarded-host")
        .or_else(|| req.header("host"))
        .map(|h| h.split(',').next().unwrap_or("").trim())
        .filter(|h| valid_host(h));
    let Some(host) = host else {
        return DEFAULT_URL.to_owned();
    };
    let local = host.starts_with("localhost") || host.starts_with("127.0.0.1");
    let proto = match req.header("x-forwarded-proto") {
        Some(p) if p.trim().eq_ignore_ascii_case("http") => "http",
        Some(_) => "https",
        None if local => "http",
        None => "https",
    };
    format!("{proto}://{host}/")
}

/// `https://exemple.fr/app` → `https://exemple.fr/app/` ; `None` si ce n'est pas une
/// adresse http(s).
pub fn normalize_url(url: &str) -> Option<String> {
    let url = url.trim();
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let host = rest.split('/').next().unwrap_or("");
    if !valid_host(host)
        || url
            .chars()
            .any(|c| c.is_whitespace() || c == '"' || c == '<')
    {
        return None;
    }
    Some(format!("{}/", url.trim_end_matches('/')))
}

/// Un nom d'hôte (et son port) : rien qui puisse sortir d'un attribut HTML.
fn valid_host(host: &str) -> bool {
    !host.is_empty()
        && host.len() <= 253
        && host
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b':'))
}

/// L'adresse sans `https://` ni `/` final, pour l'afficher.
pub fn display_url(url: &str) -> &str {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
}

/// Encode une valeur pour une adresse (RFC 3986 : seuls les caractères non réservés
/// restent tels quels).
pub fn encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 3);
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Un réseau où partager : son nom, son adresse de partage, une marque courte, sa couleur.
pub struct Network {
    pub name: &'static str,
    pub href: String,
    pub badge: &'static str,
    pub tone: &'static str,
}

/// Les liens de partage de `url`, du plus courant au moins courant.
pub fn networks(url: &str) -> Vec<Network> {
    let u = encode(url);
    let t = encode(SHARE_TEXT);
    let both = encode(&format!("{SHARE_TEXT} {url}"));
    let subject = encode("Une idée pour ce soir : Frigo vide");
    vec![
        Network {
            name: "WhatsApp",
            href: format!("https://wa.me/?text={both}"),
            badge: "W",
            tone: "mint",
        },
        Network {
            name: "Telegram",
            href: format!("https://t.me/share/url?url={u}&text={t}"),
            badge: "T",
            tone: "sky",
        },
        Network {
            name: "X",
            href: format!("https://x.com/intent/post?text={t}&url={u}"),
            badge: "X",
            tone: "ink",
        },
        Network {
            name: "Facebook",
            href: format!("https://www.facebook.com/sharer/sharer.php?u={u}"),
            badge: "f",
            tone: "grape",
        },
        Network {
            name: "LinkedIn",
            href: format!("https://www.linkedin.com/sharing/share-offsite/?url={u}"),
            badge: "in",
            tone: "sky",
        },
        Network {
            name: "E-mail",
            href: format!("mailto:?subject={subject}&body={both}"),
            badge: "@",
            tone: "sun",
        },
        Network {
            name: "SMS",
            href: format!("sms:?&body={both}"),
            badge: "✉",
            tone: "peach",
        },
    ]
}

/// Le QR code de `url`, en SVG : un seul chemin, net à toutes les tailles.
pub fn qr_svg(url: &str) -> String {
    let Ok(qr) = QrCode::encode_text(url, QrCodeEcc::Medium) else {
        return String::new();
    };
    let border = 2;
    let size = qr.size();
    let full = size + border * 2;
    let mut path = String::with_capacity((size * size) as usize * 4);
    for y in 0..size {
        let mut x = 0;
        while x < size {
            if qr.get_module(x, y) {
                // Les modules noirs voisins d'une même ligne forment un seul rectangle.
                let start = x;
                while x < size && qr.get_module(x, y) {
                    x += 1;
                }
                path.push_str(&format!(
                    "M{},{}h{}v1h-{}z",
                    start + border,
                    y + border,
                    x - start,
                    x - start
                ));
            } else {
                x += 1;
            }
        }
    }
    format!(
        r##"<svg class="qr" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {full} {full}" shape-rendering="crispEdges" role="img" aria-label="QR code vers {label}"><rect width="{full}" height="{full}" fill="#fff"/><path fill="#1f1a3d" d="{path}"/></svg>"##,
        label = display_url(url),
    )
}

/// L'année en cours (UTC), pour le copyright du pied de page.
pub fn current_year() -> i64 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    year_of_day(i64::try_from(secs / 86_400).unwrap_or(0))
}

/// L'année du jour `days` depuis le 1er janvier 1970 (l'algorithme « civil from days »
/// de Howard Hinnant).
fn year_of_day(days: i64) -> i64 {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    yoe + era * 400 + i64::from(month <= 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_are_normalized() {
        assert_eq!(
            normalize_url("https://exemple.fr/app").as_deref(),
            Some("https://exemple.fr/app/")
        );
        assert_eq!(
            normalize_url(" http://localhost:3000/ ").as_deref(),
            Some("http://localhost:3000/")
        );
        assert_eq!(normalize_url("javascript:alert(1)"), None);
        assert_eq!(normalize_url("https://a\"b.fr/"), None);
        assert_eq!(display_url(DEFAULT_URL), "maxlestage.github.io/ASSIET");
    }

    #[test]
    fn share_links_are_encoded() {
        assert_eq!(encode("a b&c/é"), "a%20b%26c%2F%C3%A9");
        let links = networks("https://exemple.fr/");
        assert!(links.iter().all(|n| !n.href.contains(' ')));
        assert!(links.iter().any(|n| n.href
            == "https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fexemple.fr%2F"));
    }

    #[test]
    fn the_qr_code_is_an_svg() {
        let svg = qr_svg(DEFAULT_URL);
        assert!(svg.starts_with("<svg class=\"qr\""));
        assert!(svg.contains("<path fill=\"#1f1a3d\" d=\"M"));
        assert!(svg.contains("maxlestage.github.io/ASSIET"));
    }

    #[test]
    fn years_are_computed() {
        assert_eq!(year_of_day(0), 1970);
        assert_eq!(year_of_day(20_405), 2025); // 13 novembre 2025
        assert_eq!(year_of_day(20_770), 2026); // 13 novembre 2026
        assert!(current_year() >= 2026);
    }
}
