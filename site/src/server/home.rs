//! La page d'accueil, rendue par le serveur : tout le texte est dans le HTML (lisible
//! sans WebAssembly, et par les moteurs de recherche) ; les îlots et le mouvement
//! arrivent ensuite.

use paquets::prelude::*;

use crate::islands::{assiette, frigo};

const REPO: &str = "https://github.com/maxlestage/ASSIET";
const TITLE: &str = "Frigo vide — la recette la plus rapide avec ce qu'il reste dans le frigo";
const DESCRIPTION: &str = "Frigo vide lit vos ingrédients, interroge TheMealDB et trouve la recette la plus rapide à préparer. En Rust (CLI et TUI) et sur iPhone (SwiftUI).";

fn layout(css_version: &str, title: &str, content: Element) -> Page {
    let head = r##"<meta name="color-scheme" content="dark"/><link rel="icon" href="/assets/logo.svg" type="image/svg+xml"/><link rel="apple-touch-icon" href="/assets/logo.svg"/>"##;
    Page::new(title)
        .lang("fr")
        .description(DESCRIPTION)
        .seo(
            Seo::new()
                .site_name("Frigo vide")
                .og_type("website")
                .locale("fr_FR")
                .image("/assets/og.svg")
                .canonical("/"),
        )
        .plain()
        .stylesheet(format!("/site.css?v={css_version}"))
        .head(head)
        .body(
            div()
                .class("site")
                .child(a().class("skip").href("#main").text("Aller au contenu"))
                .child(div().class("progress").attr("aria-hidden", "true"))
                .child(
                    div()
                        .class("aurora")
                        .attr("aria-hidden", "true")
                        .children((0..3).map(|i| span().style(format!("--i:{i}")))),
                )
                .child(navbar())
                .child(content.id("main"))
                .child(site_footer()),
        )
}

pub fn page(css_version: &str) -> Page {
    layout(
        css_version,
        TITLE,
        html::main()
            .child(hero())
            .child(steps())
            .child(demo())
            .child(terminal())
            .child(tui())
            .child(ios())
            .child(architecture())
            .child(install()),
    )
}

pub fn not_found(css_version: &str) -> Page {
    layout(
        css_version,
        "Page introuvable — Frigo vide",
        html::main().child(
            section()
                .class("section section--center lost")
                .child(
                    div()
                        .class("lost__fridge")
                        .attr("aria-hidden", "true")
                        .child(span())
                        .child(span()),
                )
                .child(h1().class("title").text("Ce placard est vide"))
                .child(
                    p().class("lead")
                        .text("La page demandée n'existe pas (ou quelqu'un l'a mangée)."),
                )
                .child(
                    a().class("button button--primary")
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
                .child(
                    span()
                        .class("logo")
                        .attr("aria-hidden", "true")
                        .child(span())
                        .child(span()),
                )
                .child(span().text("Frigo vide")),
        )
        .child(
            nav()
                .class("nav__links")
                .attr("aria-label", "Sections")
                .child(a().href("#demo").text("Démo"))
                .child(a().href("#terminal").text("CLI"))
                .child(a().href("#ios").text("iPhone"))
                .child(a().href("#installer").text("Installer")),
        )
        .child(
            a().class("button button--small")
                .href(REPO)
                .attr("rel", "noopener")
                .text("GitHub"),
        )
}

/// Un titre dont chaque lettre arrive en 3D, l'une après l'autre.
fn letters(text: &str) -> Vec<Element> {
    text.split(' ')
        .enumerate()
        .map(|(w, word)| {
            span()
                .class("word")
                .children(word.chars().enumerate().map(move |(i, c)| {
                    span()
                        .class("char")
                        .style(format!("--i:{}", w * 6 + i))
                        .text(c.to_string())
                }))
        })
        .collect()
}

fn hero() -> Element {
    section()
        .class("hero")
        .child(
            div()
                .class("hero__text")
                .child(p().class("eyebrow").text("Rust · Paquets · SwiftUI · TheMealDB"))
                .child(
                    h1().class("hero__title")
                        .attr("aria-label", "Le frigo est vide\u{a0}? Pas l'assiette.")
                        .child(span().class("line").attr("aria-hidden", "true").children(letters("Le frigo est vide\u{a0}?")))
                        .child(span().class("line line--accent").attr("aria-hidden", "true").children(letters("Pas l'assiette."))),
                )
                .child(p().class("lead").text(
                    "Dites ce qu'il reste dans votre frigo. Frigo vide interroge TheMealDB, garde les recettes \
                     que vous pouvez faire avec, et vous donne la plus rapide à préparer.",
                ))
                .child(
                    div()
                        .class("hero__ctas")
                        .child(a().class("button button--primary").href("#demo").text("Essayer maintenant"))
                        .child(a().class("button button--ghost").href("#installer").text("cargo install")),
                )
                .child(
                    ul().class("stats")
                        .child(li().child(strong().text("0")).text(" clé d'API"))
                        .child(li().child(strong().text("100 %")).text(" Rust côté terminal"))
                        .child(li().child(strong().text("1")).text(" cœur partagé")),
                ),
        )
        .child(
            div()
                .class("hero__stage")
                .child(div().class("hero__halo").attr("aria-hidden", "true"))
                .child(island("assiette", "", assiette)),
        )
}

fn steps() -> Element {
    let items = [
        (
            "🧊",
            "Videz le frigo",
            "Tapez vos ingrédients, en français ou en anglais : « poulet », « œufs », « pommes de terre ».",
        ),
        (
            "🔎",
            "TheMealDB cherche",
            "Une requête par ingrédient, en parallèle, puis le détail des recettes trouvées le plus souvent.",
        ),
        (
            "⚖️",
            "On compare",
            "Ce que vous avez, ce qui manque : le sel, l'huile et l'eau sont au placard.",
        ),
        (
            "⏱️",
            "Le plus rapide",
            "Le temps est lu dans les instructions (« bake 35 minutes »). À égalité, la plus rapide gagne.",
        ),
    ];
    section()
        .class("section")
        .id("comment")
        .child(
            div()
                .class("section__head")
                .attr("data-reveal", "")
                .child(cube())
                .child(p().class("eyebrow").text("Comment ça marche"))
                .child(h2().class("title").text("Quatre étapes, zéro courses"))
                .child(p().class("lead").text(
                    "Le même cœur Rust, frigo-core, sert la CLI, la TUI et l'API de ce site.",
                )),
        )
        .child(
            ol().class("steps-grid")
                .children(items.iter().enumerate().map(|(i, (icon, title, text))| {
                    li().class("step")
                        .attr("data-reveal", "")
                        .attr("data-tilt", "")
                        .style(format!("--d:{}ms", i * 110))
                        .child(span().class("step__num").text(format!("0{}", i + 1)))
                        .child(
                            span()
                                .class("step__icon")
                                .attr("aria-hidden", "true")
                                .text(*icon),
                        )
                        .child(h3().text(*title))
                        .child(p().text(*text))
                })),
        )
}

/// Un cube d'ingrédients en CSS 3D, qui tourne sur lui-même.
fn cube() -> Element {
    let faces = ["🍅", "🥕", "🧀", "🥚", "🍄", "🧅"];
    div().class("cube").attr("aria-hidden", "true").child(
        div().class("cube__inner").children(
            faces
                .iter()
                .enumerate()
                .map(|(i, f)| span().class(format!("cube__face cube__face--{i}")).text(*f)),
        ),
    )
}

fn demo() -> Element {
    section()
        .class("section")
        .id("demo")
        .child(
            div()
                .class("section__head")
                .attr("data-reveal", "")
                .child(p().class("eyebrow").text("Démo en direct"))
                .child(h2().class("title").text("Ouvrez le frigo"))
                .child(p().class("lead").text(
                    "Touchez une suggestion ou tapez un ingrédient, puis ouvrez le frigo. Touchez une carte pour la retourner.",
                )),
        )
        .child(div().class("demo").attr("data-reveal", "").child(island("frigo", "poulet,riz,oignon", frigo)))
}

fn terminal() -> Element {
    // La vraie sortie de `frigo poulet riz oignon ail` (étapes et dernières lignes coupées).
    let lines: [(&str, &str); 16] = [
        ("cmd", "frigo poulet riz oignon ail"),
        ("dim", "🧊 Frigo : poulet, riz, oignon, ail"),
        ("", ""),
        ("title", "🍽  Chicken Congee"),
        (
            "dim",
            "   ≈ 94 min · Chicken · Chinese · 67 % des ingrédients",
        ),
        ("ok", "   ✓ Chicken (8 oz)"),
        ("ok", "   ✓ Rice (1/2 cup)"),
        ("ok", "   ✓ Spring Onions (1 tbs)"),
        ("ok", "   ✓ Salt · Pepper · Water"),
        ("miss", "   ✗ Ginger (1 tsp)"),
        ("miss", "   ✗ Coriander (2 oz)"),
        ("miss", "   À acheter : Ginger Cordial, Ginger, Coriander"),
        ("", ""),
        ("title", "Aussi possible :"),
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
        .class("section section--split")
        .id("terminal")
        .child(
            div()
                .class("section__head")
                .attr("data-reveal", "")
                .child(p().class("eyebrow").text("En ligne de commande"))
                .child(h2().class("title").text("Un binaire, aucun JavaScript"))
                .child(p().class("lead").text(
                    "frigo est compilé avec cargo build --release : tokio pour l'asynchrone, reqwest pour HTTP, \
                     serde pour le JSON, clap pour les options. Sortie en couleurs ou en JSON pour vos scripts.",
                ))
                .child(
                    ul().class("flags")
                        .child(li().child(code().text("--strict")).text(" rien à acheter"))
                        .child(li().child(code().text("-t 30")).text(" 30 minutes au plus"))
                        .child(li().child(code().text("-f frigo.txt")).text(" la liste d'un fichier"))
                        .child(li().child(code().text("--json")).text(" pour jq et compagnie")),
                ),
        )
        .child(
            div()
                .class("window window--term")
                .attr("data-depth", "")
                .attr("data-reveal", "")
                .child(window_bar("~/cuisine — frigo"))
                .child(div().class("term").attr("role", "img").attr("aria-label", "Exemple de sortie de la commande frigo").children(
                    lines.iter().enumerate().map(|(i, (kind, text))| {
                        let line = div().class(format!("term__line term__line--{}", if kind.is_empty() { "blank" } else { kind })).style(format!("--i:{i}"));
                        if *kind == "cmd" {
                            line.child(span().class("term__prompt").text("$ ")).child(span().class("term__typed").style(format!("--n:{}", text.chars().count())).text(*text))
                        } else {
                            line.text(*text)
                        }
                    }),
                )),
        )
}

fn window_bar(title: &str) -> Element {
    div()
        .class("window__bar")
        .child(span().class("dot dot--r"))
        .child(span().class("dot dot--y"))
        .child(span().class("dot dot--g"))
        .child(span().class("window__title").text(title.to_owned()))
}

fn tui() -> Element {
    let recipes = [
        ("-3", "Zemiakové Placky", "84 min", false),
        ("-3", "Salt cod tortilla", "93 min", false),
        ("-3", "Yemeni Lahsa", "25 min", false),
        ("-4", "Spiced tortilla", "30 min", false),
    ];
    section()
        .class("section section--split section--reverse")
        .id("tui")
        .child(
            div()
                .class("section__head")
                .attr("data-reveal", "")
                .child(p().class("eyebrow").text("Interface plein écran"))
                .child(h2().class("title").text("La TUI, au clavier"))
                .child(p().class("lead").text(
                    "frigo --tui ouvre une interface façon curses (ratatui et crossterm) : le frigo en haut, \
                     les recettes à gauche, la recette à droite. Elle s'empile sur un terminal étroit.",
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
                .attr("data-depth", "")
                .attr("data-reveal", "")
                .attr("role", "img")
                .attr("aria-label", "Aperçu de l'interface plein écran de frigo")
                .child(window_bar("frigo --tui"))
                .child(
                    div()
                        .class("tui")
                        .child(div().class("tui__box tui__box--active tui__fridge").child(span().class("tui__label").text("🧊 Mon frigo")).child(span().text("œufs, pomme de terre, oignon")).child(span().class("tui__caret")))
                        .child(
                            div()
                                .class("tui__body")
                                .child(div().class("tui__box tui__list").child(span().class("tui__label").text("🍽 Recettes (4)")).children(recipes.iter().enumerate().map(|(i, (badge, name, time, ok))| {
                                    div().class("tui__row")
                                        .class_if("is-selected", move || i == 0)
                                        .child(span().class(if *ok { "tui__badge tui__badge--ok" } else { "tui__badge" }).text(*badge))
                                        .child(span().class("tui__name").text(*name))
                                        .child(span().class("tui__time").text(*time))
                                })))
                                .child(
                                    div()
                                        .class("tui__box tui__recipe")
                                        .child(span().class("tui__label").text("Zemiakové Placky"))
                                        .child(div().class("tui__gauge").style("--p:67%").child(span()).child(em().text("67 % · ≈ 84 min")))
                                        .child(p().class("ok").text("✓ Potatoes  ✓ Onion  ✓ Egg"))
                                        .child(p().class("ok").text("✓ Salt  ✓ Black Pepper  ✓ Oil"))
                                        .child(p().class("miss").text("✗ Garlic  ✗ All purpose flour"))
                                        .child(p().class("miss").text("✗ Marjoram"))
                                        .child(p().child(strong().text(" 1. ")).text("Start by peeling and finely grating the potatoes…")),
                                ),
                        )
                        .child(div().class("tui__help").child(span().class("tui__flags").text(" strict:non placard:oui ")).child(span().text(" ↑↓ choisir · Tab zone · q quitter"))),
                ),
        )
}

fn ios() -> Element {
    section()
        .class("section section--split")
        .id("ios")
        .child(
            div()
                .class("section__head")
                .attr("data-reveal", "")
                .child(p().class("eyebrow").text("Sur iPhone"))
                .child(h2().class("title").text("La même idée, en SwiftUI"))
                .child(p().class("lead").text(
                    "L'app iOS est écrite en Swift 6 : SwiftUI, la concurrence structurée (async/await, \
                     actors), SwiftData pour garder le frigo, et FrigoKit, le même algorithme que frigo-core, \
                     testé avec Swift Testing.",
                ))
                .child(
                    ul().class("features")
                        .child(li().text("Le frigo enregistré sur l'appareil"))
                        .child(li().text("Recherche en parallèle avec un TaskGroup"))
                        .child(li().text("Cartes animées, retour haptique, mode sombre"))
                        .child(li().text("Aucune dépendance externe")),
                ),
        )
        .child(
            div()
                .class("phone-stage")
                .attr("data-depth", "")
                .attr("data-reveal", "")
                .child(
                    div()
                        .class("phone")
                        .attr("role", "img")
                        .attr("aria-label", "L'application Frigo vide sur iPhone")
                        .child(div().class("phone__side phone__side--l"))
                        .child(div().class("phone__side phone__side--r"))
                        .child(
                            div()
                                .class("phone__screen")
                                .child(div().class("phone__island"))
                                .child(p().class("phone__time").text("9:41"))
                                .child(h3().class("phone__title").text("Mon frigo"))
                                .child(div().class("phone__chips").children(["🍗 poulet", "🍚 riz", "🧅 oignon", "🧄 ail"].map(|c| span().text(c))))
                                .child(div().class("phone__cta").text("Trouver une recette"))
                                .child(div().class("phone__cards").children([
                                    ("Chicken Congee", "94 min", "-3", "a"),
                                    ("Poulet Roti a l'Algerienne", "105 min", "-3", "b"),
                                    ("Leek and Rice Bake", "90 min", "-3", "c"),
                                ].map(|(name, time, miss, tone)| {
                                    div().class(format!("phone__card phone__card--{tone}"))
                                        .child(span().class("phone__thumb"))
                                        .child(span().class("phone__name").text(name))
                                        .child(span().class("phone__meta").text(format!("{time} · {miss}")))
                                }))),
                        ),
                )
                .child(div().class("phone-shadow").attr("aria-hidden", "true")),
        )
}

fn architecture() -> Element {
    let blocks = [
        (
            "Entrée",
            "Arguments, fichier, entrée standard, TUI, formulaire du site ou app iOS",
        ),
        (
            "frigo-core",
            "Normalisation (français → anglais, pluriels), client TheMealDB asynchrone",
        ),
        (
            "TheMealDB",
            "filter.php?i= pour chaque ingrédient, lookup.php?i= pour le détail",
        ),
        (
            "Classement",
            "Ingrédients manquants, puis frigo le mieux utilisé, puis temps estimé",
        ),
        (
            "Affichage",
            "Console, ratatui, JSON, cartes WebAssembly ou SwiftUI",
        ),
    ];
    section()
        .class("section")
        .id("architecture")
        .child(
            div()
                .class("section__head")
                .attr("data-reveal", "")
                .child(p().class("eyebrow").text("Architecture"))
                .child(h2().class("title").text("Un cœur, trois visages")),
        )
        .child(
            ol().class("flow")
                .children(blocks.iter().enumerate().map(|(i, (title, text))| {
                    li().class("flow__item")
                        .attr("data-reveal", "")
                        .style(format!("--d:{}ms", i * 90))
                        .child(span().class("flow__dot").attr("aria-hidden", "true"))
                        .child(h3().text(*title))
                        .child(p().text(*text))
                })),
        )
}

fn install() -> Element {
    let commands = [
        (
            "La CLI et la TUI",
            "cargo install --git https://github.com/maxlestage/ASSIET frigo --locked",
        ),
        ("Une recette", "frigo poulet riz \"pomme de terre\""),
        ("Sans courses", "frigo -f frigo.txt --strict -t 30"),
        ("Le site", "cd site && cargo run --release"),
        (
            "L'app iOS",
            "cd ios && xcodegen && open FrigoVide.xcodeproj",
        ),
    ];
    section()
        .class("section section--center")
        .id("installer")
        .child(
            div()
                .class("section__head")
                .attr("data-reveal", "")
                .child(p().class("eyebrow").text("Installer"))
                .child(h2().class("title").text("À table en une commande"))
                .child(p().class("lead").text(
                    "Rust 1.95 ou plus récent. Aucune clé d'API\u{a0}: TheMealDB est gratuit.",
                )),
        )
        .child(
            div()
                .class("commands")
                .children(commands.iter().enumerate().map(|(i, (label, cmd))| {
                    figure()
                        .class("command")
                        .attr("data-reveal", "")
                        .style(format!("--d:{}ms", i * 80))
                        .child(figcaption().text(*label))
                        .child(pre().child(code().text(*cmd)))
                })),
        )
        .child(
            div()
                .class("hero__ctas hero__ctas--center")
                .attr("data-reveal", "")
                .child(
                    a().class("button button--primary")
                        .href(REPO)
                        .attr("rel", "noopener")
                        .text("Le code sur GitHub"),
                )
                .child(
                    a().class("button button--ghost")
                        .href("#demo")
                        .text("Rejouer la démo"),
                ),
        )
}

fn site_footer() -> Element {
    footer()
        .class("footer")
        .child(p().text("Frigo vide · écrit en Rust avec Paquets, et en Swift pour l'iPhone."))
        .child(
            p().class("muted")
                .child(
                    a().href("https://www.themealdb.com/")
                        .attr("rel", "noopener")
                        .text("Recettes\u{a0}: TheMealDB"),
                )
                .text(" · Licence MIT"),
        )
}
