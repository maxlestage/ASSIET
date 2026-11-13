//! La page d'accueil, rendue par le serveur : tout le texte est dans le HTML (lisible
//! sans WebAssembly, et par les moteurs de recherche) ; les îlots et le mouvement
//! arrivent ensuite. Le style est plat, en 2D, et très coloré.

use paquets::prelude::*;

use super::share::{SHARE_TEXT, current_year, display_url, networks, qr_svg};
use crate::islands::{frigo, frigo_dessin};

const REPO: &str = "https://github.com/maxlestage/ASSIET";
const ISSUES: &str = "https://github.com/maxlestage/ASSIET/issues";
const README: &str = "https://github.com/maxlestage/ASSIET#readme";
const LICENSE: &str = "https://github.com/maxlestage/ASSIET/blob/master/LICENSE";
const PAQUETS: &str = "https://github.com/maxlestage/Paquets";
const MEALDB: &str = "https://www.themealdb.com/";
const TITLE: &str = "Frigo vide — la recette la plus rapide avec ce qu'il reste dans le frigo";
const DESCRIPTION: &str = "Frigo vide lit vos ingrédients, interroge TheMealDB et trouve la meilleure recette à préparer. En Rust (CLI et TUI) et sur iPhone (SwiftUI).";

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
                .child(div().class("progress").attr("aria-hidden", "true"))
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
            .child(demo())
            .child(terminal())
            .child(tui())
            .child(ios())
            .child(architecture())
            .child(install())
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
                .child(a().href("#demo").text("Démo"))
                .child(a().href("#terminal").text("CLI"))
                .child(a().href("#ios").text("iPhone"))
                .child(a().href("#installer").text("Installer"))
                .child(a().href("#partager").text("Partager")),
        )
        .child(
            a().class("btn btn--ink btn--small")
                .href(REPO)
                .attr("rel", "noopener")
                .text("GitHub"),
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
                .child(p().class("kicker").child(span().text("Rust")).child(span().text("Paquets")).child(span().text("SwiftUI")))
                .child(
                    h1().class("hero__title")
                        .child(span().class("hero__line").text("Le frigo est vide\u{a0}?"))
                        .child(span().class("hero__line hero__line--mark").child(span().text("Pas l'assiette."))),
                )
                .child(p().class("lead").text(
                    "Dites ce qu'il reste dans votre frigo. Frigo vide interroge TheMealDB, garde les recettes \
                     que vous pouvez faire avec, et vous donne la meilleure, la plus rapide en cas d'égalité.",
                ))
                .child(
                    div()
                        .class("ctas")
                        .child(a().class("btn btn--tomato btn--big").href("#demo").text("Essayer maintenant →"))
                        .child(a().class("btn btn--white btn--big").href("#installer").text("cargo install")),
                )
                .child(
                    ul().class("stats")
                        .child(li().class("stat stat--sun").child(strong().text("0")).text(" clé d'API"))
                        .child(li().class("stat stat--mint").child(strong().text("100\u{a0}%")).text(" Rust au terminal"))
                        .child(li().class("stat stat--sky").child(strong().text("1")).text(" cœur partagé")),
                ),
        )
        .child(div().class("hero__art").child(island("frigo-dessin", "", frigo_dessin)))
}

/// Un bandeau d'ingrédients qui défile (deux copies pour une boucle sans couture).
fn marquee() -> Element {
    let words = [
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
    ];
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
    div()
        .class("marquee")
        .child(track(false))
        .child(track(true))
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
            "Tapez vos ingrédients, en français ou en anglais\u{a0}: «\u{a0}poulet\u{a0}», «\u{a0}œufs\u{a0}», «\u{a0}pommes de terre\u{a0}».",
            "sun",
        ),
        (
            "🔎",
            "TheMealDB cherche",
            "Une requête par ingrédient, en parallèle, puis le détail des recettes trouvées le plus souvent.",
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
            Some("Le même cœur Rust, frigo-core, sert la CLI, la TUI et l'API de ce site."),
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

fn demo() -> Element {
    section()
        .class("section section--band section--sky")
        .id("demo")
        .child(head(
            "Démo en direct",
            "Qu'est-ce qu'on mange\u{a0}?",
            Some("Touchez une suggestion ou tapez un ingrédient, puis lancez la recherche. Les recettes viennent vraiment de TheMealDB."),
        ))
        .child(div().class("demo-wrap").attr("data-reveal", "").child(island("frigo", "poulet,riz,oignon", frigo)))
}

fn window_bar(title: &str) -> Element {
    div()
        .class("window__bar")
        .child(span().class("dot dot--r"))
        .child(span().class("dot dot--y"))
        .child(span().class("dot dot--g"))
        .child(span().class("window__title").text(title.to_owned()))
}

fn terminal() -> Element {
    // La vraie sortie de `frigo poulet riz oignon ail` (étapes et dernières lignes coupées).
    let lines: [(&str, &str); 16] = [
        ("cmd", "frigo poulet riz oignon ail"),
        ("dim", "🧊 Frigo\u{a0}: poulet, riz, oignon, ail"),
        ("", ""),
        ("title", "🍽  Chicken Congee"),
        (
            "dim",
            "   ≈ 94 min · Chicken · Chinese · 67\u{a0}% des ingrédients",
        ),
        ("ok", "   ✓ Chicken (8 oz)"),
        ("ok", "   ✓ Rice (1/2 cup)"),
        ("ok", "   ✓ Spring Onions (1 tbs)"),
        ("ok", "   ✓ Salt · Pepper · Water"),
        ("miss", "   ✗ Ginger (1 tsp)"),
        ("miss", "   ✗ Coriander (2 oz)"),
        (
            "miss",
            "   À acheter\u{a0}: Ginger Cordial, Ginger, Coriander",
        ),
        ("", ""),
        ("title", "Aussi possible\u{a0}:"),
        (
            "dim",
            "   • Poulet Roti a l'Algerienne ≈ 105 min · manque 3",
        ),
        (
            "dim",
            "   • Presh me Oriz Leek and Rice Bake ≈ 90 min · manque 3",
        ),
    ];
    section()
        .class("section split")
        .id("terminal")
        .child(
            div()
                .child(head(
                    "En ligne de commande",
                    "Un binaire, aucun JavaScript",
                    Some("frigo est un seul binaire, compilé par cargo\u{a0}: tokio pour l'asynchrone, reqwest pour HTTP, serde pour le JSON, clap pour les options."),
                ))
                .child(
                    ul().class("flags")
                        .child(li().child(code().text("--strict")).text(" rien à acheter"))
                        .child(li().child(code().text("-t 30")).text(" 30 minutes au plus"))
                        .child(li().child(code().text("-f frigo.txt")).text(" la liste d'un fichier"))
                        .child(li().child(code().text("--json")).text(" pour vos scripts")),
                ),
        )
        .child(
            div()
                .class("window window--term")
                .attr("data-reveal", "")
                .child(window_bar("~/cuisine — frigo"))
                .child(div().class("term").attr("role", "img").attr("aria-label", "Exemple de sortie de la commande frigo").children(
                    lines.iter().enumerate().map(|(i, (kind, text))| {
                        let kind = if kind.is_empty() { "blank" } else { kind };
                        let line = div().class(format!("term__line term__line--{kind}")).style(format!("--i:{i}"));
                        if kind == "cmd" {
                            line.child(span().class("term__prompt").text("$ "))
                                .child(span().class("term__typed").style(format!("--n:{}", text.chars().count())).text(*text))
                        } else {
                            line.text(*text)
                        }
                    }),
                )),
        )
}

fn tui() -> Element {
    let recipes = [
        ("-3", "Zemiakové Placky", "84 min"),
        ("-3", "Salt cod tortilla", "93 min"),
        ("-3", "Yemeni Lahsa", "25 min"),
        ("-4", "Spiced tortilla", "30 min"),
    ];
    section()
        .class("section split split--reverse")
        .id("tui")
        .child(
            div()
                .child(head(
                    "Interface plein écran",
                    "La TUI, au clavier",
                    Some("frigo --tui ouvre une interface façon curses (ratatui et crossterm)\u{a0}: le frigo en haut, les recettes à gauche, la recette à droite."),
                ))
                .child(
                    ul().class("keys")
                        .child(li().child(kbd().text("↑ ↓")).text(" choisir"))
                        .child(li().child(kbd().text("Tab")).text(" changer de zone"))
                        .child(li().child(kbd().text("s")).text(" strict"))
                        .child(li().child(kbd().text("p")).text(" placard"))
                        .child(li().child(kbd().text("q")).text(" quitter")),
                ),
        )
        .child(
            div()
                .class("window window--tui")
                .attr("data-reveal", "")
                .attr("role", "img")
                .attr("aria-label", "Aperçu de l'interface plein écran de frigo")
                .child(window_bar("frigo --tui"))
                .child(
                    div()
                        .class("tui")
                        .child(
                            div()
                                .class("tui__box tui__box--active")
                                .child(span().class("tui__label").text("🧊 Mon frigo"))
                                .child(span().text("œufs, pomme de terre, oignon"))
                                .child(span().class("tui__caret")),
                        )
                        .child(
                            div()
                                .class("tui__body")
                                .child(div().class("tui__box").child(span().class("tui__label").text("🍽 Recettes (4)")).children(
                                    recipes.iter().enumerate().map(|(i, (badge, name, time))| {
                                        div().class(if i == 0 { "tui__row is-selected" } else { "tui__row" })
                                            .child(span().class("tui__badge").text(*badge))
                                            .child(span().class("tui__name").text(*name))
                                            .child(span().class("tui__time").text(*time))
                                    }),
                                ))
                                .child(
                                    div()
                                        .class("tui__box")
                                        .child(span().class("tui__label").text("Zemiakové Placky"))
                                        .child(div().class("tui__gauge").style("--p:67%").child(span()).child(em().text("67\u{a0}% · ≈ 84 min")))
                                        .child(p().class("ok").text("✓ Potatoes  ✓ Onion  ✓ Egg"))
                                        .child(p().class("ok").text("✓ Salt  ✓ Black Pepper  ✓ Oil"))
                                        .child(p().class("miss").text("✗ Garlic  ✗ All purpose flour"))
                                        .child(p().class("miss").text("✗ Marjoram")),
                                ),
                        )
                        .child(
                            div()
                                .class("tui__help")
                                .child(span().class("tui__flags").text(" strict:non placard:oui "))
                                .child(span().text(" ↑↓ choisir · Tab zone · q quitter")),
                        ),
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
                    "Sur iPhone",
                    "La même idée, en SwiftUI",
                    Some("L'app iOS est écrite en Swift 6\u{a0}: SwiftUI, async/await et actors, SwiftData pour garder le frigo, et FrigoKit, le même algorithme que frigo-core, testé avec Swift Testing."),
                ))
                .child(
                    ul().class("features")
                        .child(li().text("📦 Le frigo enregistré sur l'appareil"))
                        .child(li().text("⚡ Recherche en parallèle (TaskGroup)"))
                        .child(li().text("📳 Retours haptiques et animations"))
                        .child(li().text("🧩 Aucune dépendance externe")),
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
            "Arguments, fichier, entrée standard, TUI, formulaire du site ou app iOS",
            "sun",
        ),
        (
            "🦀",
            "frigo-core",
            "Normalisation (français → anglais, pluriels), client TheMealDB asynchrone",
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
            "Console, ratatui, JSON, cartes WebAssembly ou SwiftUI",
            "grape",
        ),
    ];
    section()
        .class("section")
        .id("architecture")
        .child(head("Architecture", "Un cœur, trois visages", None))
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

fn install() -> Element {
    let commands = [
        (
            "La CLI et la TUI",
            "cargo install --git https://github.com/maxlestage/ASSIET frigo --locked",
            "sun",
        ),
        ("Une recette", "frigo poulet riz \"pomme de terre\"", "mint"),
        ("Sans courses", "frigo -f frigo.txt --strict -t 30", "sky"),
        ("Le site", "cd site && cargo run --release", "peach"),
        (
            "L'app iOS",
            "cd ios && xcodegen && open FrigoVide.xcodeproj",
            "grape",
        ),
    ];
    section()
        .class("section section--center")
        .id("installer")
        .child(head(
            "Installer",
            "À table en une commande",
            Some("Rust 1.95 ou plus récent. Aucune clé d'API\u{a0}: TheMealDB est gratuit."),
        ))
        .child(
            div()
                .class("commands")
                .children(commands.iter().enumerate().map(|(i, (label, cmd, tone))| {
                    figure()
                        .class(format!("command command--{tone}"))
                        .attr("data-reveal", "")
                        .style(format!("--d:{}ms", i * 80))
                        .child(figcaption().text(*label))
                        .child(pre().child(code().text(*cmd)))
                })),
        )
        .child(
            div()
                .class("ctas ctas--center")
                .attr("data-reveal", "")
                .child(
                    a().class("btn btn--tomato btn--big")
                        .href(REPO)
                        .attr("rel", "noopener")
                        .text("Le code sur GitHub"),
                )
                .child(
                    a().class("btn btn--white btn--big")
                        .href("#demo")
                        .text("Rejouer la démo"),
                ),
        )
}

/// Les boutons d'installation : cachés tant que le navigateur ne propose pas
/// l'installation (le bundle les montre), et l'astuce pour Safari sur iPhone.
fn install_buttons(class: &str) -> Element {
    div()
        .class(format!("install {class}"))
        .child(
            button()
                .class("btn btn--sun")
                .attr("type", "button")
                .attr("data-install", "")
                .attr("hidden", "")
                .child(span().attr("aria-hidden", "true").text("⬇"))
                .child(span().text("Installer l'app")),
        )
        .child(
            p().class("install__ios")
                .attr("data-install-ios", "")
                .attr("hidden", "")
                .text("Sur iPhone\u{a0}: touchez ")
                .child(strong().text("Partager"))
                .text(" puis ")
                .child(strong().text("Sur l'écran d'accueil"))
                .text("."),
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
                                .child(span().class("pill pill--tomato").text("Hors ligne")),
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
            div()
                .class("share-actions__more")
                .child(install_buttons("install--share"))
                .child(
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
            Some("Montrez la carte à un ami, envoyez le lien, ou installez Frigo vide sur votre écran d'accueil\u{a0}: l'app s'ouvre comme une vraie application, même sans réseau."),
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
                            "La meilleure recette avec ce qu'il reste dans le frigo. \
                             Gratuit, sans compte, sans publicité.",
                        ))
                        .child(install_buttons("install--footer"))
                        .child(div().class("footer__socials").children(quick)),
                )
                .child(footer_column(
                    "Produit",
                    "Le produit",
                    &[
                        ("Démo en direct", "/#demo", false),
                        ("Ligne de commande", "/#terminal", false),
                        ("Interface plein écran", "/#tui", false),
                        ("App iPhone", "/#ios", false),
                        ("Installer", "/#installer", false),
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
