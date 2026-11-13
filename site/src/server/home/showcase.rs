//! Les sections de la vitrine de l'app iPhone : tout se passe dans l'app, le site la
//! montre. Rien n'est interactif ici (des dessins, des cartes, des animations CSS), sauf
//! les questions fréquentes, des `<details>` du navigateur.

use paquets::prelude::*;

use super::{REPO, head};

/// Un petit iPhone dessiné, avec son titre de navigation.
fn screen(title: &str, tone: &str, body: Element) -> Element {
    div()
        .class(format!("screen screen--{tone}"))
        .child(span().class("screen__island").attr("aria-hidden", "true"))
        .child(p().class("screen__time").text("9:41"))
        .child(p().class("screen__title").text(title.to_owned()))
        .child(body)
}

/// Les trois écrans de l'app, comme dans l'app : le frigo, les recettes, la recette.
pub fn screens() -> Element {
    let fridge = div()
        .class("screen__body")
        .child(div().class("ui-chips").children(
            ["🍗 poulet", "🍚 riz", "🧅 oignon", "🧄 ail", "🍅 tomate"].map(|c| span().text(c)),
        ))
        .child(div().class("ui-field").text("Ajouter un ingrédient…"))
        .child(
            div()
                .class("ui-suggest")
                .children(["+ œufs", "+ pâtes", "+ citron"].map(|c| span().text(c))),
        )
        .child(
            div()
                .class("ui-toggle is-on")
                .child(span().text("Sans courses"))
                .child(span().class("ui-switch")),
        )
        .child(
            div()
                .class("ui-toggle is-on")
                .child(span().text("Sel, huile, eau au placard"))
                .child(span().class("ui-switch")),
        )
        .child(div().class("ui-button").text("🔎 Trouver une recette"));

    let card = |name: &str,
                meta: &str,
                minutes: &str,
                missing: &str,
                cover: u8,
                tone: &str,
                best: bool| {
        let mut el = div().class(format!("ui-card ui-card--{tone}"));
        if best {
            el = el.child(span().class("ui-best").text("⭐ Meilleur choix"));
        }
        el.child(p().class("ui-card__name").text(name.to_owned()))
            .child(p().class("ui-card__meta").text(meta.to_owned()))
            .child(
                div()
                    .class("ui-tags")
                    .child(
                        span()
                            .class("ui-tag ui-tag--grape")
                            .text(format!("⏱ ≈ {minutes} min")),
                    )
                    .child(
                        span()
                            .class("ui-tag ui-tag--peach")
                            .text(format!("🛒 Manque {missing}")),
                    ),
            )
            .child(
                div()
                    .class("ui-gauge")
                    .style(format!("--p:{cover}%"))
                    .child(span())
                    .child(em().text(format!("Frigo {cover}\u{a0}%"))),
            )
    };
    let results = div()
        .class("screen__body")
        .child(card(
            "Chicken Congee",
            "Chicken · Chinese",
            "94",
            "3",
            67,
            "sun",
            true,
        ))
        .child(card(
            "Kung Pao Chicken",
            "Chicken · Chinese",
            "35",
            "4",
            50,
            "mint",
            false,
        ))
        .child(card(
            "Leek and Rice Bake",
            "Vegetarian · Albanian",
            "90",
            "3",
            40,
            "sky",
            false,
        ));

    let recipe = div()
        .class("screen__body")
        .child(p().class("ui-section").text("Ingrédients"))
        .child(
            ul().class("ui-list")
                .child(li().class("ok").text("✓ Chicken · 8 oz"))
                .child(li().class("ok").text("✓ Rice · 1/2 cup"))
                .child(li().class("ok").text("✓ Spring Onions · 1 tbs"))
                .child(li().class("miss").text("✗ Ginger · 1 tsp")),
        )
        .child(p().class("ui-section").text("Étapes"))
        .child(
            ol().class("ui-steps")
                .child(
                    li().class("is-done")
                        .child(span().text("1"))
                        .text("Rincer le riz"),
                )
                .child(
                    li().class("is-done")
                        .child(span().text("2"))
                        .text("Porter l'eau à ébullition"),
                )
                .child(li().child(span().text("3")).text("Ajouter le poulet")),
        )
        .child(
            div()
                .class("ui-button ui-button--tomato")
                .text("▶ Voir la vidéo"),
        );

    let items = [
        (
            "Mon frigo",
            "sun",
            fridge,
            "1. Remplissez le frigo",
            "Tapez vos ingrédients ou touchez une suggestion. Le frigo reste enregistré.",
        ),
        (
            "Recettes",
            "mint",
            results,
            "2. Comparez",
            "La meilleure en premier\u{a0}: ce qui manque, le temps, la part de votre frigo utilisée.",
        ),
        (
            "Chicken Congee",
            "sky",
            recipe,
            "3. Cuisinez",
            "Les ingrédients cochés, les étapes à barrer une à une, et la vidéo quand il y en a une.",
        ),
    ];
    section()
        .class("section section--band section--sky")
        .id("ecrans")
        .child(head(
            "L'app",
            "Trois écrans, un repas",
            Some("Frigo vide est une app iPhone. Tout se passe dedans\u{a0}: voici à quoi elle ressemble."),
        ))
        .child(div().class("screens").children(items.into_iter().enumerate().map(
            |(i, (title, tone, body, caption, text))| {
                figure()
                    .class("screens__item")
                    .attr("data-reveal", "")
                    .style(format!("--d:{}ms;--i:{i}", i * 140))
                    .child(
                        div()
                            .attr("role", "img")
                            .attr("aria-label", format!("Écran «\u{a0}{title}\u{a0}» de l'app Frigo vide"))
                            .child(screen(title, tone, body)),
                    )
                    .child(figcaption().child(strong().text(caption)).child(span().text(text)))
            },
        )))
}

/// Les fonctions de l'app, en tuiles colorées.
pub fn features() -> Element {
    let items = [
        (
            "🧊",
            "Votre frigo, gardé",
            "Vos ingrédients restent dans l'app d'une fois sur l'autre. Un geste pour en retirer, un bouton pour tout vider.",
            "sun",
        ),
        (
            "🇫🇷",
            "En français",
            "«\u{a0}Poulet\u{a0}», «\u{a0}œufs\u{a0}», «\u{a0}pommes de terre\u{a0}»\u{a0}: l'app traduit pour TheMealDB, pluriels et accents compris.",
            "sky",
        ),
        (
            "🛒",
            "Sans courses",
            "Le mode strict ne garde que les recettes faisables avec ce que vous avez déjà.",
            "mint",
        ),
        (
            "🧂",
            "Le placard",
            "Sel, poivre, huile, eau et sucre sont comptés comme acquis. Un interrupteur les retire.",
            "peach",
        ),
        (
            "⏱️",
            "Le temps estimé",
            "Lu dans les instructions («\u{a0}bake 35 minutes\u{a0}»), plus le temps de préparer chaque ingrédient.",
            "grape",
        ),
        (
            "🏆",
            "La meilleure d'abord",
            "Le moins d'ingrédients manquants, puis le frigo le mieux utilisé, puis la plus rapide.",
            "tomato",
        ),
        (
            "✅",
            "Étapes à cocher",
            "Touchez une étape pour la barrer\u{a0}: l'iPhone vibre doucement, vous savez où vous en êtes.",
            "mint",
        ),
        (
            "▶️",
            "La vidéo",
            "Quand TheMealDB a une vidéo de la recette, elle est à un toucher.",
            "sun",
        ),
    ];
    section()
        .class("section")
        .id("fonctions")
        .child(head(
            "Fonctionnalités",
            "Tout ce qu'il faut, rien de plus",
            Some("Pas de compte, pas de publicité, pas d'abonnement. Juste la bonne recette."),
        ))
        .child(
            ul().class("tiles").children(items.iter().enumerate().map(
                |(i, (icon, title, text, tone))| {
                    li().class(format!("tile tile--{tone}"))
                        .attr("data-reveal", "")
                        .style(format!("--d:{}ms", (i % 4) * 90))
                        .child(
                            span()
                                .class("tile__icon")
                                .attr("aria-hidden", "true")
                                .text(*icon),
                        )
                        .child(h3().text(*title))
                        .child(p().text(*text))
                },
            )),
        )
}

/// Quelques chiffres, en grand.
pub fn numbers() -> Element {
    let items = [
        (
            "24",
            "recettes comparées, au plus, à chaque recherche",
            "sun",
        ),
        ("20", "ingrédients lus par recette", "mint"),
        ("12", "suggestions à un toucher", "sky"),
        ("0", "compte, pub ou abonnement", "tomato"),
    ];
    section()
        .class("numbers")
        .attr("aria-label", "Frigo vide en chiffres")
        .child(
            ul().class("numbers__list")
                .children(items.iter().enumerate().map(|(i, (n, label, tone))| {
                    li().class(format!("number number--{tone}"))
                        .attr("data-reveal", "")
                        .style(format!("--d:{}ms", i * 110))
                        .child(strong().text(*n))
                        .child(span().text(*label))
                })),
        )
}

/// Le placard : les bocaux toujours là.
pub fn pantry() -> Element {
    let jars = [
        ("🧂", "Sel", "sky"),
        ("🌶️", "Poivre", "tomato"),
        ("🫒", "Huile", "sun"),
        ("💧", "Eau", "mint"),
        ("🍬", "Sucre", "grape"),
    ];
    section()
        .class("section split")
        .id("placard")
        .child(
            div()
                .child(head(
                    "Le placard",
                    "Ce qu'on a toujours",
                    Some("Personne ne note «\u{a0}sel\u{a0}» sur sa liste. Frigo vide non plus\u{a0}: ces cinq-là ne comptent jamais comme manquants. Si votre placard est vraiment vide, un interrupteur de l'app les retire."),
                )),
        )
        .child(
            div()
                .class("shelf")
                .attr("data-reveal", "")
                .attr("role", "img")
                .attr("aria-label", "Une étagère avec du sel, du poivre, de l'huile, de l'eau et du sucre")
                .child(div().class("shelf__jars").children(jars.iter().enumerate().map(
                    |(i, (icon, name, tone))| {
                        div()
                            .class(format!("jar jar--{tone}"))
                            .style(format!("--i:{i}"))
                            .child(span().class("jar__lid"))
                            .child(span().class("jar__icon").text(*icon))
                            .child(span().class("jar__label").text(*name))
                    },
                )))
                .child(div().class("shelf__board")),
        )
}

/// Une semaine de repas, pour l'exemple.
pub fn week() -> Element {
    let days = [
        (
            "Lun",
            "poulet · riz · oignon",
            "🍲",
            "Congee au poulet",
            "sun",
        ),
        ("Mar", "œufs · pommes de terre", "🥔", "Tortilla", "peach"),
        ("Mer", "lait · œufs · farine", "🥞", "Crêpes", "sky"),
        ("Jeu", "pâtes · œufs · fromage", "🍝", "Carbonara", "mint"),
        (
            "Ven",
            "tomates · œufs · poivron",
            "🍳",
            "Chakchouka",
            "tomato",
        ),
        ("Sam", "riz · légumes · œufs", "🍚", "Riz sauté", "grape"),
        ("Dim", "carottes · poireaux", "🥣", "Soupe maison", "sun"),
    ];
    section()
        .class("section section--band section--peach")
        .id("semaine")
        .child(head(
            "Une semaine",
            "Sept soirs, zéro courses",
            Some("Un exemple de semaine avec ce qui traîne au frigo. Chaque soir, l'app propose, vous cuisinez."),
        ))
        .child(ol().class("week").children(days.iter().enumerate().map(
            |(i, (day, fridge, icon, dish, tone))| {
                li().class(format!("day day--{tone}"))
                    .attr("data-reveal", "")
                    .style(format!("--d:{}ms;--i:{i}", i * 70))
                    .child(span().class("day__name").text(*day))
                    .child(span().class("day__icon").attr("aria-hidden", "true").text(*icon))
                    .child(strong().class("day__dish").text(*dish))
                    .child(span().class("day__fridge").text(*fridge))
            },
        )))
}

/// Anti-gaspi : les restes ont du talent.
pub fn leftovers() -> Element {
    let items = [
        ("🍚", "Le riz d'hier", "🍳", "Riz sauté", "sun"),
        ("🥖", "Le pain rassis", "🍞", "Pain perdu", "peach"),
        ("🍌", "Les bananes trop mûres", "🍰", "Banana bread", "mint"),
        ("🥕", "Les légumes fatigués", "🥣", "Soupe", "sky"),
        ("🧀", "Les bouts de fromage", "🥘", "Gratin", "grape"),
        ("🌿", "Les herbes flétries", "🫙", "Pesto", "tomato"),
    ];
    section()
        .class("section")
        .id("anti-gaspi")
        .child(head(
            "Anti-gaspi",
            "Les restes ont du talent",
            Some("Ce qui allait finir à la poubelle fait souvent le meilleur repas. Mettez-le dans le frigo de l'app et voyez ce qui en sort."),
        ))
        .child(ul().class("swaps").children(items.iter().enumerate().map(
            |(i, (from_icon, from, to_icon, to, tone))| {
                li().class(format!("swap swap--{tone}"))
                    .attr("data-reveal", "")
                    .style(format!("--d:{}ms", (i % 3) * 100))
                    .child(
                        span()
                            .class("swap__from")
                            .child(span().class("swap__icon").attr("aria-hidden", "true").text(*from_icon))
                            .child(span().text(*from)),
                    )
                    .child(span().class("swap__arrow").attr("aria-hidden", "true").text("→"))
                    .child(
                        span()
                            .class("swap__to")
                            .child(span().class("swap__icon").attr("aria-hidden", "true").text(*to_icon))
                            .child(strong().text(*to)),
                    )
            },
        )))
}

/// Les cuisines du monde de TheMealDB.
pub fn cuisines() -> Element {
    let items = [
        ("🇫🇷", "Française"),
        ("🇮🇹", "Italienne"),
        ("🇯🇵", "Japonaise"),
        ("🇲🇽", "Mexicaine"),
        ("🇮🇳", "Indienne"),
        ("🇲🇦", "Marocaine"),
        ("🇨🇳", "Chinoise"),
        ("🇬🇷", "Grecque"),
        ("🇹🇭", "Thaïe"),
        ("🇯🇲", "Jamaïcaine"),
        ("🇹🇷", "Turque"),
        ("🇻🇳", "Vietnamienne"),
        ("🇪🇸", "Espagnole"),
        ("🇵🇹", "Portugaise"),
        ("🇹🇳", "Tunisienne"),
        ("🇬🇧", "Britannique"),
    ];
    let tones = ["sun", "mint", "sky", "peach", "grape", "tomato"];
    section()
        .class("section section--band section--mint section--center")
        .id("cuisines")
        .child(head(
            "Le tour du monde",
            "Un tour du monde en cuisine",
            Some("Les recettes viennent de TheMealDB, une base de recettes du monde entier. Ce soir, ce sera peut-être marocain."),
        ))
        .child(ul().class("globe").children(items.iter().enumerate().map(|(i, (flag, name))| {
            li().class(format!("globe__item globe__item--{}", tones[i % tones.len()]))
                .style(format!("--i:{i}"))
                .child(span().attr("aria-hidden", "true").text(*flag))
                .child(span().text(*name))
        })))
}

/// La vie privée : le frigo reste sur l'iPhone.
pub fn privacy() -> Element {
    let items = [
        (
            "📱",
            "Sur votre iPhone",
            "Votre frigo est enregistré dans l'app, sur l'appareil. Aucun serveur à nous ne le reçoit.",
        ),
        (
            "🔎",
            "Juste la recherche",
            "Seuls les noms des ingrédients partent vers TheMealDB, le temps de chercher les recettes.",
        ),
        (
            "🙅",
            "Ni compte ni pub",
            "Pas d'inscription, pas de publicité, pas de traçage. L'app ne sait pas qui vous êtes.",
        ),
    ];
    section()
        .class("section split split--reverse")
        .id("confidentialite")
        .child(
            div()
                .child(head(
                    "Confidentialité",
                    "Votre frigo ne regarde que vous",
                    None,
                ))
                .child(
                    ul().class("promises")
                        .children(items.iter().enumerate().map(|(i, (icon, title, text))| {
                            li().attr("data-reveal", "")
                                .style(format!("--d:{}ms", i * 100))
                                .child(
                                    span()
                                        .class("promises__icon")
                                        .attr("aria-hidden", "true")
                                        .text(*icon),
                                )
                                .child(div().child(h3().text(*title)).child(p().text(*text)))
                        })),
                ),
        )
        .child(
            div()
                .class("vault")
                .attr("data-reveal", "")
                .attr("aria-hidden", "true")
                .child(span().class("vault__ring"))
                .child(span().class("vault__ring vault__ring--b"))
                .child(
                    div()
                        .class("vault__lock")
                        .child(span().class("vault__shackle"))
                        .child(span().class("vault__body").text("🧊")),
                ),
        )
}

/// Les questions fréquentes : des `<details>`, qui s'ouvrent sans script.
pub fn faq() -> Element {
    let items = [
        (
            "L'app est-elle gratuite\u{a0}?",
            "Oui, entièrement\u{a0}: pas de publicité, pas d'abonnement, pas d'achat intégré. Son code est ouvert, sous licence MIT.",
        ),
        (
            "Quand sera-t-elle sur l'App Store\u{a0}?",
            "Bientôt. En attendant, le code est sur GitHub, et l'app se compile avec Xcode pour iOS 26.",
        ),
        (
            "Et sur Android\u{a0}?",
            "Pas pour l'instant\u{a0}: Frigo vide est écrite en SwiftUI, pour l'iPhone.",
        ),
        (
            "D'où viennent les recettes\u{a0}?",
            "De TheMealDB, une base gratuite de recettes du monde entier. Elles sont en anglais, mais vous tapez vos ingrédients en français.",
        ),
        (
            "Faut-il du réseau\u{a0}?",
            "Pour chercher des recettes, oui. Votre frigo, lui, reste enregistré dans l'app.",
        ),
        (
            "Comment le temps est-il calculé\u{a0}?",
            "TheMealDB ne donne pas de durée\u{a0}: l'app la lit dans les instructions («\u{a0}simmer 20 minutes\u{a0}») et ajoute un peu de temps par ingrédient. C'est une estimation.",
        ),
        (
            "Pourquoi cette recette plutôt qu'une autre\u{a0}?",
            "L'app classe d'abord par ingrédients manquants, puis par part de votre frigo utilisée, puis par temps. La première est le «\u{a0}Meilleur choix\u{a0}».",
        ),
    ];
    section()
        .class("section")
        .id("faq")
        .child(head("Questions", "On vous répond", None))
        .child(
            div()
                .class("faq")
                .children(items.iter().enumerate().map(|(i, (q, a))| {
                    details()
                        .class("faq__item")
                        .attr("data-reveal", "")
                        .style(format!("--d:{}ms", (i % 4) * 70))
                        .child(summary().text(*q))
                        .child(p().text(*a))
                })),
        )
}

/// L'appel à l'action final : l'app arrive sur l'App Store.
pub fn app_store() -> Element {
    section()
        .class("section section--band section--tomato appstore")
        .id("telecharger")
        .child(
            div()
                .class("appstore__inner")
                .child(
                    div()
                        .class("appstore__icon")
                        .attr("data-reveal", "")
                        .attr("aria-hidden", "true")
                        .child(span().text("🧊")),
                )
                .child(
                    div()
                        .class("appstore__text")
                        .attr("data-reveal", "")
                        .child(p().class("eyebrow").text("Bientôt"))
                        .child(h2().class("title").text("Frigo vide arrive sur l'App Store"))
                        .child(p().class("lead").text(
                            "Gratuite, sans compte et sans publicité. En attendant, suivez le projet sur GitHub, ou partagez-le à ceux qui ont toujours un frigo à moitié vide.",
                        ))
                        .child(
                            div()
                                .class("ctas")
                                .child(
                                    span()
                                        .class("btn btn--ink btn--big btn--soon")
                                        .attr("aria-disabled", "true")
                                        .child(span().attr("aria-hidden", "true").text("📲"))
                                        .child(span().text("Bientôt sur l'App Store")),
                                )
                                .child(
                                    a().class("btn btn--white btn--big")
                                        .href(REPO)
                                        .attr("rel", "noopener")
                                        .text("Suivre sur GitHub"),
                                )
                                .child(
                                    a().class("btn btn--sun btn--big")
                                        .href("#partager")
                                        .text("Partager"),
                                ),
                        ),
                ),
        )
}
