//! La page d'accueil, rendue par le serveur : tout le texte est dans le HTML (lisible
//! sans WebAssembly, et par les moteurs de recherche) ; les îlots et le mouvement
//! arrivent ensuite. Le style est plat, en 2D, et très coloré.

use paquets::prelude::*;

use super::share::{SHARE_TEXT, current_year, display_url, networks, qr_svg};
use crate::islands::frigo_dessin;

mod showcase;

const REPO: &str = "https://github.com/maxlestage/ASSIET";
const ISSUES: &str = "https://github.com/maxlestage/ASSIET/issues";
const README: &str = "https://github.com/maxlestage/ASSIET#readme";
const LICENSE: &str = "https://github.com/maxlestage/ASSIET/blob/master/LICENSE";
const PAQUETS: &str = "https://github.com/maxlestage/Paquets";
const MEALDB: &str = "https://www.themealdb.com/";
const TITLE: &str = "Frigo vide — la recette la plus rapide avec ce qu'il reste dans le frigo";
const DESCRIPTION: &str = "Frigo vide, l'app iPhone qui lit vos ingrédients, interroge TheMealDB et trouve la meilleure recette à préparer. Bientôt sur l'App Store : gratuite, sans compte, sans publicité.";

/// Ce que chaque page doit savoir : la version de la feuille de style et l'adresse
/// publique du site (pour les liens de partage, le QR code et les aperçus).
pub struct Site {
    css_version: String,
    url: String,
}

impl Site {
    pub fn new(css_version: &str, url: String) -> Self {
        Self {
            css_version: css_version.to_owned(),
            url,
        }
    }
}

fn layout(site: &Site, title: &str, content: Element) -> Page {
    let head = concat!(
        r##"<meta name="color-scheme" content="light"/>"##,
        r##"<link rel="icon" href="/assets/logo.svg" type="image/svg+xml"/>"##,
        r##"<link rel="icon" href="/assets/favicon-32.png" type="image/png" sizes="32x32"/>"##,
        r##"<link rel="apple-touch-icon" href="/assets/apple-touch-icon.png"/>"##,
        r##"<meta name="mobile-web-app-capable" content="yes"/>"##,
        r##"<meta name="apple-mobile-web-app-capable" content="yes"/>"##,
        r##"<meta name="apple-mobile-web-app-title" content="Frigo vide"/>"##,
        r##"<meta name="apple-mobile-web-app-status-bar-style" content="default"/>"##,
        r##"<meta name="application-name" content="Frigo vide"/>"##,
    );
    Page::new(title)
        .lang("fr")
        .description(DESCRIPTION)
        .seo(
            Seo::new()
                .site_name("Frigo vide")
                .og_type("website")
                .locale("fr_FR")
                .image(format!("{}assets/og.png", site.url))
                .meta_property("og:image:width", "2400")
                .meta_property("og:image:height", "1260")
                .meta_property(
                    "og:image:alt",
                    "Frigo vide : le frigo est vide ? Pas l'assiette.",
                )
                .canonical(site.url.clone()),
        )
        .plain()
        .stylesheet(format!("/site.css?v={}", site.css_version))
        .head(head)
        .body(
            div()
                .class("site")
                .child(a().class("skip").href("#main").text("Aller au contenu"))
                .child(navbar())
                .child(content.id("main"))
                .child(site_footer(&site.url)),
        )
}

pub fn page(site: &Site) -> Page {
    layout(
        site,
        TITLE,
        html::main()
            .child(hero())
            .child(marquee())
            .child(steps())
            .child(showcase::screens())
            .child(showcase::features())
            .child(showcase::numbers())
            .child(ios())
            .child(showcase::pantry())
            .child(showcase::week())
            .child(dishes_band())
            .child(showcase::leftovers())
            .child(showcase::cuisines())
            .child(showcase::privacy())
            .child(tech_band())
            .child(architecture())
            .child(showcase::faq())
            .child(showcase::app_store())
            .child(share(&site.url)),
    )
}

pub fn not_found(site: &Site) -> Page {
    layout(
        site,
        "Page introuvable — Frigo vide",
        html::main().child(
            section()
                .class("section section--center lost")
                .child(
                    span()
                        .class("lost__emoji")
                        .attr("aria-hidden", "true")
                        .text("🥡"),
                )
                .child(h1().class("title").text("Ce placard est vide"))
                .child(
                    p().class("lead")
                        .text("La page demandée n'existe pas (ou quelqu'un l'a mangée)."),
                )
                .child(
                    a().class("btn btn--tomato btn--big")
                        .href("/")
                        .text("Retour à la cuisine"),
                ),
        ),
    )
}

fn navbar() -> Element {
    header()
        .class("nav")
        // La barre de progression, accrochée sous la navigation : toujours visible, même
        // sur iPhone où le haut de l'écran passe sous la barre d'état.
        .child(div().class("progress").attr("aria-hidden", "true"))
        .child(
            a().class("nav__brand")
                .href("/")
                .attr("aria-label", "Frigo vide, accueil")
                .child(span().class("logo").attr("aria-hidden", "true").text("🧊"))
                .child(span().text("Frigo vide")),
        )
        .child(
            nav()
                .class("nav__links")
                .attr("aria-label", "Sections")
                .child(a().href("#ecrans").text("L'app"))
                .child(a().href("#fonctions").text("Fonctions"))
                .child(a().href("#faq").text("Questions"))
                .child(a().href("#partager").text("Partager")),
        )
        .child(
            a().class("btn btn--ink btn--small")
                .href("#telecharger")
                .text("Bientôt"),
        )
}

/// Des autocollants qui flottent autour du titre.
fn stickers() -> Element {
    let items = [
        ("🍋", "sun"),
        ("🥑", "mint"),
        ("🍓", "tomato"),
        ("🫐", "sky"),
        ("🥐", "peach"),
        ("🍆", "grape"),
    ];
    div()
        .class("stickers")
        .attr("aria-hidden", "true")
        .children(items.iter().enumerate().map(|(i, (e, tone))| {
            span()
                .class(format!("sticker sticker--{tone} sticker--{i}"))
                .style(format!("--i:{i}"))
                .text(*e)
        }))
}

fn hero() -> Element {
    section()
        .class("hero")
        .child(stickers())
        .child(
            div()
                .class("hero__text")
                .child(p().class("kicker").child(span().text("iPhone")).child(span().text("Gratuit")).child(span().text("Sans compte")))
                .child(
                    h1().class("hero__title")
                        .child(span().class("hero__line").text("Le frigo est vide\u{a0}?"))
                        .child(span().class("hero__line hero__line--mark").child(span().text("Pas l'assiette."))),
                )
                .child(p().class("lead").text(
                    "Frigo vide est une app iPhone\u{a0}: dites-lui ce qu'il reste dans votre frigo, elle \
                     cherche parmi les recettes du monde entier et vous donne la meilleure, la plus rapide en cas d'égalité.",
                ))
                .child(
                    div()
                        .class("ctas")
                        .child(a().class("btn btn--tomato btn--big").href("#ecrans").text("Découvrir l'app →"))
                        .child(a().class("btn btn--white btn--big").href("#telecharger").text("Bientôt sur l'App Store")),
                )
                .child(
                    ul().class("stats")
                        .child(li().class("stat stat--sun").child(strong().text("0")).text(" clé d'API"))
                        .child(li().class("stat stat--mint").child(strong().text("0")).text(" compte à créer"))
                        .child(li().class("stat stat--sky").child(strong().text("100\u{a0}%")).text(" gratuit")),
                ),
        )
        .child(div().class("hero__art").child(island("frigo-dessin", "", frigo_dessin)))
}

/// Un bandeau qui défile (plusieurs copies pour une boucle sans couture). `variant` choisit
/// la couleur, l'inclinaison et le sens : `""` (encre), `"tomato"`, `"sun"`, `"mint"`.
fn band(words: &[&'static str], variant: &str) -> Element {
    let track = |hidden: bool| {
        let t = div()
            .class("marquee__track")
            .children(words.iter().map(|w| span().class("marquee__item").text(*w)));
        if hidden {
            t.attr("aria-hidden", "true")
        } else {
            t
        }
    };
    let class = if variant.is_empty() {
        "marquee".to_owned()
    } else {
        format!("marquee marquee--{variant}")
    };
    // Quatre copies : même sur un grand écran, la boucle reste pleine (une seule est lue).
    div()
        .class(class)
        .child(track(false))
        .children((0..3).map(|_| track(true)))
}

/// Le bandeau d'ingrédients, sous le titre.
fn marquee() -> Element {
    band(
        &[
            "🍅 tomate",
            "🥚 œufs",
            "🧀 fromage",
            "🍗 poulet",
            "🥕 carotte",
            "🍚 riz",
            "🧅 oignon",
            "🍋 citron",
            "🍄 champignon",
            "🥔 pomme de terre",
        ],
        "",
    )
}

/// Le bandeau des technologies, avant l'architecture (dans l'autre sens).
fn tech_band() -> Element {
    band(
        &[
            "📱 iPhone",
            "🍎 SwiftUI",
            "🫧 Liquid Glass",
            "💾 SwiftData",
            "🔎 TheMealDB",
            "📳 Haptique",
            "🌍 Recettes du monde",
            "💚 Open source",
        ],
        "tomato",
    )
}

/// Le bandeau des plats, avant l'installation.
fn dishes_band() -> Element {
    band(
        &[
            "🍝 pâtes",
            "🥗 salade",
            "🍲 soupe",
            "🌮 tacos",
            "🍛 curry",
            "🥘 paella",
            "🍳 omelette",
            "🥞 crêpes",
            "🍜 ramen",
            "🥧 quiche",
        ],
        "sun",
    )
}

/// Le bandeau du pied de page.
fn footer_band() -> Element {
    band(
        &[
            "✨ Gratuit",
            "🔑 Sans clé d'API",
            "🙅 Sans compte",
            "🍪 Sans cookie",
            "📱 Pour iPhone",
            "🔒 Privée",
            "🇫🇷 En français",
            "💚 Open source",
        ],
        "mint",
    )
}

fn head(eyebrow: &str, title: &str, lead: Option<&str>) -> Element {
    let mut el = div()
        .class("section__head")
        .attr("data-reveal", "")
        .child(p().class("eyebrow").text(eyebrow.to_owned()))
        .child(h2().class("title").text(title.to_owned()));
    if let Some(lead) = lead {
        el = el.child(p().class("lead").text(lead.to_owned()));
    }
    el
}

fn steps() -> Element {
    let items = [
        (
            "🧊",
            "Videz le frigo",
            "Dans l'app, tapez vos ingrédients, en français ou en anglais\u{a0}: «\u{a0}poulet\u{a0}», «\u{a0}œufs\u{a0}», «\u{a0}pommes de terre\u{a0}».",
            "sun",
        ),
        (
            "🔎",
            "L'app cherche",
            "Touchez «\u{a0}Trouver une recette\u{a0}»\u{a0}: chaque ingrédient est cherché dans TheMealDB, en même temps.",
            "mint",
        ),
        (
            "⚖️",
            "On compare",
            "Ce que vous avez, ce qui manque\u{a0}: le sel, l'huile et l'eau sont au placard.",
            "sky",
        ),
        (
            "⏱️",
            "Le meilleur gagne",
            "Le temps est lu dans les instructions («\u{a0}bake 35 minutes\u{a0}»). À égalité, la plus rapide gagne.",
            "peach",
        ),
    ];
    section()
        .class("section")
        .id("comment")
        .child(head(
            "Comment ça marche",
            "Quatre étapes, zéro courses",
            Some("Quelques secondes entre «\u{a0}qu'est-ce qu'on mange\u{a0}?\u{a0}» et le premier coup de couteau."),
        ))
        .child(
            ol().class("steps-grid")
                .children(
                    items
                        .iter()
                        .enumerate()
                        .map(|(i, (icon, title, text, tone))| {
                            li().class(format!("step step--{tone}"))
                                .attr("data-reveal", "")
                                .style(format!("--d:{}ms", i * 100))
                                .child(span().class("step__num").text(format!("{}", i + 1)))
                                .child(
                                    span()
                                        .class("step__icon")
                                        .attr("aria-hidden", "true")
                                        .text(*icon),
                                )
                                .child(h3().text(*title))
                                .child(p().text(*text))
                        }),
                ),
        )
}

fn ios() -> Element {
    let cards = [
        ("Chicken Congee", "94 min · -3", "sun"),
        ("Poulet Roti a l'Algerienne", "105 min · -3", "mint"),
        ("Leek and Rice Bake", "90 min · -3", "peach"),
    ];
    section()
        .class("section section--band section--grape split")
        .id("ios")
        .child(
            div()
                .child(head(
                    "Pensée pour l'iPhone",
                    "Douce au toucher",
                    Some("Écrite en SwiftUI pour iOS 26\u{a0}: le verre liquide d'iOS, des cartes qui arrivent en relief quand on fait défiler, et une petite vibration à chaque étape cochée."),
                ))
                .child(
                    ul().class("features")
                        .child(li().text("🫧 Liquid Glass, le style d'iOS 26"))
                        .child(li().text("🃏 Des cartes qui arrivent en 3D"))
                        .child(li().text("📳 Des vibrations douces"))
                        .child(li().text("⚡ Une recherche rapide, tout en parallèle")),
                ),
        )
        .child(
            div()
                .class("phone-stage")
                .attr("data-reveal", "")
                .child(span().class("blob blob--a").attr("aria-hidden", "true"))
                .child(span().class("blob blob--b").attr("aria-hidden", "true"))
                .child(
                    div()
                        .class("phone")
                        .attr("role", "img")
                        .attr("aria-label", "L'application Frigo vide sur iPhone")
                        .child(div().class("phone__island"))
                        .child(p().class("phone__time").text("9:41"))
                        .child(h3().class("phone__title").text("Mon frigo"))
                        .child(div().class("phone__chips").children(["🍗 poulet", "🍚 riz", "🧅 oignon", "🧄 ail"].map(|c| span().text(c))))
                        .child(div().class("phone__cta").text("Trouver une recette"))
                        .child(div().class("phone__cards").children(cards.map(|(name, meta, tone)| {
                            div().class(format!("phone__card phone__card--{tone}"))
                                .child(span().class("phone__thumb"))
                                .child(span().class("phone__name").text(name))
                                .child(span().class("phone__meta").text(meta))
                        }))),
                ),
        )
}

fn architecture() -> Element {
    let blocks = [
        (
            "📝",
            "Entrée",
            "Le frigo de l'app, gardé avec SwiftData",
            "sun",
        ),
        (
            "🧠",
            "FrigoKit",
            "Le français traduit pour TheMealDB, pluriels et accents compris",
            "tomato",
        ),
        (
            "🌍",
            "TheMealDB",
            "filter.php?i= pour chaque ingrédient, lookup.php?i= pour le détail",
            "sky",
        ),
        (
            "🏆",
            "Classement",
            "Ingrédients manquants, puis frigo le mieux utilisé, puis temps estimé",
            "mint",
        ),
        (
            "🖥️",
            "Affichage",
            "Des cartes SwiftUI, la meilleure en premier",
            "grape",
        ),
    ];
    section()
        .class("section")
        .id("architecture")
        .child(head("Sous le capot", "Du frigo à l'assiette", None))
        .child(
            ol().class("flow").children(blocks.iter().enumerate().map(
                |(i, (icon, title, text, tone))| {
                    li().class(format!("flow__item flow__item--{tone}"))
                        .attr("data-reveal", "")
                        .style(format!("--d:{}ms", i * 90))
                        .child(
                            span()
                                .class("flow__icon")
                                .attr("aria-hidden", "true")
                                .text(*icon),
                        )
                        .child(h3().text(*title))
                        .child(p().text(*text))
                },
            )),
        )
}

/// La carte de partage : une carte à scanner (QR code), le partage du téléphone, la copie
/// du lien et des liens directs vers les messageries et les réseaux.
fn share(url: &str) -> Element {
    let shown = display_url(url).to_owned();
    let card = figure()
        .class("share-card")
        .attr("data-reveal", "")
        .child(
            div()
                .class("share-card__top")
                .child(
                    span()
                        .class("logo logo--white")
                        .attr("aria-hidden", "true")
                        .text("🧊"),
                )
                .child(
                    div()
                        .child(strong().class("share-card__name").text("Frigo vide"))
                        .child(
                            span()
                                .class("share-card__tag")
                                .text("La recette avec ce qu'il reste"),
                        ),
                ),
        )
        .child(
            div()
                .class("share-card__body")
                .child(div().class("share-card__qr").html(qr_svg(url)))
                .child(
                    div()
                        .class("share-card__text")
                        .child(
                            p().class("share-card__claim")
                                .text("Le frigo est vide\u{a0}? Pas l'assiette."),
                        )
                        .child(
                            p().class("share-card__hint")
                                .text("📱 Scannez avec l'appareil photo"),
                        )
                        .child(
                            div()
                                .class("share-card__chips")
                                .child(span().class("pill pill--white").text("Gratuit"))
                                .child(span().class("pill pill--mint").text("Sans compte"))
                                .child(span().class("pill pill--tomato").text("iPhone")),
                        ),
                ),
        )
        .child(
            figcaption()
                .class("share-card__url")
                .child(span().class("share-card__dot").attr("aria-hidden", "true"))
                .child(code().text(shown)),
        );

    let actions = div()
        .class("share-actions")
        .attr("data-reveal", "")
        .style("--d:120ms")
        .child(
            div()
                .class("share-actions__main")
                .child(
                    button()
                        .class("btn btn--tomato btn--big")
                        .attr("type", "button")
                        .attr("data-share", "")
                        .attr("data-url", url.to_owned())
                        .attr("data-title", "Frigo vide")
                        .attr("data-text", SHARE_TEXT)
                        .attr("hidden", "")
                        .child(span().attr("aria-hidden", "true").text("↗"))
                        .child(span().text("Partager…")),
                )
                .child(
                    button()
                        .class("btn btn--white btn--big")
                        .attr("type", "button")
                        .attr("data-copy", url.to_owned())
                        .attr("hidden", "")
                        .child(span().attr("aria-hidden", "true").text("🔗"))
                        .child(span().attr("data-copy-label", "").text("Copier le lien")),
                ),
        )
        .child(
            label()
                .class("share-link")
                .child(span().class("share-link__label").text("Le lien"))
                .child(
                    input()
                        .attr("type", "url")
                        .attr("readonly", "")
                        .attr("value", url.to_owned())
                        .attr("aria-label", "Le lien du site, à copier"),
                ),
        )
        .child(p().class("share-actions__title").text("Envoyer par"))
        .child(
            ul().class("networks")
                .children(networks(url).into_iter().map(|n| {
                    li().child(
                        a().class(format!("network network--{}", n.tone))
                            .href(n.href)
                            .attr("target", "_blank")
                            .attr("rel", "noopener noreferrer")
                            .attr(
                                "aria-label",
                                format!("Partager sur {} (nouvel onglet)", n.name),
                            )
                            .child(
                                span()
                                    .class("network__badge")
                                    .attr("aria-hidden", "true")
                                    .text(n.badge),
                            )
                            .child(span().text(n.name)),
                    )
                })),
        )
        .child(
            div().class("share-actions__more").child(
                a().class("link")
                    .href("/assets/og.png")
                    .attr("download", "frigo-vide.png")
                    .text("Télécharger l'image de partage"),
            ),
        );

    section()
        .class("section section--band section--sun share")
        .id("partager")
        .child(head(
            "Partager",
            "Faites tourner l'assiette",
            Some("Montrez la carte à un ami ou envoyez-lui le lien\u{a0}: il découvrira Frigo vide avant même sa sortie sur l'App Store."),
        ))
        .child(div().class("share__grid").child(card).child(actions))
}

fn footer_column(
    title: &str,
    label: &str,
    links: &[(&'static str, &'static str, bool)],
) -> Element {
    nav()
        .class("footer__col")
        .attr("aria-label", label.to_owned())
        .child(h2().class("footer__title").text(title.to_owned()))
        .child(ul().children(links.iter().map(|(text, href, external)| {
            let link = a().href(*href).text(*text);
            let link = if *external {
                link.attr("rel", "noopener")
            } else {
                link
            };
            li().child(link)
        })))
}

fn site_footer(url: &str) -> Element {
    let networks = networks(url);
    let quick: Vec<Element> = networks
        .iter()
        .filter(|n| {
            matches!(
                n.name,
                "WhatsApp" | "Telegram" | "X" | "LinkedIn" | "E-mail"
            )
        })
        .map(|n| {
            a().class(format!("footer__social network--{}", n.tone))
                .href(n.href.clone())
                .attr("target", "_blank")
                .attr("rel", "noopener noreferrer")
                .attr(
                    "aria-label",
                    format!("Partager sur {} (nouvel onglet)", n.name),
                )
                .attr("title", n.name)
                .text(n.badge)
        })
        .collect();
    footer()
        .class("footer")
        .child(footer_band())
        .child(
            div()
                .class("footer__inner")
                .child(
                    div()
                        .class("footer__brand")
                        .child(
                            a().class("nav__brand footer__logo")
                                .href("/")
                                .attr("aria-label", "Frigo vide, accueil")
                                .child(span().class("logo").attr("aria-hidden", "true").text("🧊"))
                                .child(span().text("Frigo vide")),
                        )
                        .child(p().class("footer__pitch").text(
                            "L'app iPhone qui trouve la meilleure recette avec ce qu'il reste dans le frigo. \
                             Gratuite, sans compte, sans publicité.",
                        ))
                        .child(div().class("footer__socials").children(quick)),
                )
                .child(footer_column(
                    "Produit",
                    "Le produit",
                    &[
                        ("Les écrans", "/#ecrans", false),
                        ("Fonctionnalités", "/#fonctions", false),
                        ("Confidentialité", "/#confidentialite", false),
                        ("Questions fréquentes", "/#faq", false),
                        ("Bientôt sur l'App Store", "/#telecharger", false),
                    ],
                ))
                .child(footer_column(
                    "Ressources",
                    "Les ressources",
                    &[
                        ("Code source", REPO, true),
                        ("Documentation", README, true),
                        ("Signaler un problème", ISSUES, true),
                        ("TheMealDB", MEALDB, true),
                        ("Paquets", PAQUETS, true),
                    ],
                ))
                .child(footer_column(
                    "Partager",
                    "Partager",
                    &[
                        ("La carte à scanner", "/#partager", false),
                        ("Image de partage", "/assets/og.png", false),
                        ("Licence MIT", LICENSE, true),
                    ],
                )),
        )
        .child(
            div()
                .class("footer__bottom")
                .child(p().text(format!("© {} Frigo vide · Licence MIT", current_year())))
                .child(
                    p().class("footer__trust")
                        .child(span().text("🍪 Aucun cookie"))
                        .child(span().text("🔒 Aucun traçage"))
                        .child(span().text("🦀 Rust & Swift")),
                )
                .child(
                    a().class("footer__top")
                        .href("#main")
                        .text("Haut de page ↑"),
                ),
        )
}
