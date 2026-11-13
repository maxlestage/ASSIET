//! Les sections de la vitrine de l'app iPhone : tout se passe dans l'app, le site la
//! montre. Rien n'est interactif ici (des dessins, des cartes, des animations CSS), sauf
//! les questions fréquentes, des `<details>` du navigateur.

use paquets::prelude::*;

use super::{REPO, head};
use crate::i18n::Lang;

/// Un petit iPhone dessiné, avec son titre de navigation.
fn screen(title: &str, tone: &str, body: Element) -> Element {
    div()
        .class(format!("screen screen--{tone}"))
        .child(span().class("screen__island").attr("aria-hidden", "true"))
        .child(p().class("screen__time").text("9:41"))
        .child(p().class("screen__title").text(title.to_owned()))
        .child(body)
}

/// Une recette des écrans dessinés : nom, catégorie, minutes, manquants, part du frigo.
struct Demo {
    name: &'static str,
    meta: &'static str,
    minutes: u32,
    missing: u32,
    cover: u32,
    tone: &'static str,
    best: bool,
}

fn demo_card(l: Lang, d: &Demo) -> Element {
    let mut el = div().class(format!("ui-card ui-card--{}", d.tone));
    if d.best {
        el = el.child(span().class("ui-best").text(l.t(
            "⭐ Meilleur choix",
            "⭐ Best match",
            "⭐ Mejor opción",
        )));
    }
    let missing = match l {
        Lang::Fr => format!("🛒 Manque {}", d.missing),
        Lang::En => format!("🛒 {} missing", d.missing),
        Lang::Es => format!("🛒 Faltan {}", d.missing),
    };
    let fridge = match l {
        Lang::Fr => format!("Frigo {}\u{a0}%", d.cover),
        Lang::En => format!("Fridge {}%", d.cover),
        Lang::Es => format!("Nevera {}\u{a0}%", d.cover),
    };
    el.child(p().class("ui-card__name").text(d.name))
        .child(p().class("ui-card__meta").text(d.meta))
        .child(
            div()
                .class("ui-tags")
                .child(
                    span()
                        .class("ui-tag ui-tag--grape")
                        .text(format!("⏱ ≈ {} min", d.minutes)),
                )
                .child(span().class("ui-tag ui-tag--peach").text(missing)),
        )
        .child(
            div()
                .class("ui-gauge")
                .style(format!("--p:{}%", d.cover))
                .child(span())
                .child(em().text(fridge)),
        )
}

/// Les trois écrans de l'app, comme dans l'app : le frigo, les recettes, la recette.
pub fn screens(l: Lang) -> Element {
    let chips: [&'static str; 5] = match l {
        Lang::Fr => ["🍗 poulet", "🍚 riz", "🧅 oignon", "🧄 ail", "🍅 tomate"],
        Lang::En => [
            "🍗 chicken",
            "🍚 rice",
            "🧅 onion",
            "🧄 garlic",
            "🍅 tomato",
        ],
        Lang::Es => ["🍗 pollo", "🍚 arroz", "🧅 cebolla", "🧄 ajo", "🍅 tomate"],
    };
    let suggest: [&'static str; 3] = match l {
        Lang::Fr => ["+ œufs", "+ pâtes", "+ citron"],
        Lang::En => ["+ eggs", "+ pasta", "+ lemon"],
        Lang::Es => ["+ huevos", "+ pasta", "+ limón"],
    };
    let toggle = |text: &'static str| {
        div()
            .class("ui-toggle is-on")
            .child(span().text(text))
            .child(span().class("ui-switch"))
    };
    let fridge = div()
        .class("screen__body")
        .child(
            div()
                .class("ui-chips")
                .children(chips.map(|c| span().text(c))),
        )
        .child(div().class("ui-field").text(l.t(
            "Ajouter un ingrédient…",
            "Add an ingredient…",
            "Añadir un ingrediente…",
        )))
        .child(
            div()
                .class("ui-suggest")
                .children(suggest.map(|c| span().text(c))),
        )
        .child(toggle(l.t("Sans courses", "No shopping", "Sin compras")))
        .child(toggle(l.t(
            "Sel, huile, eau au placard",
            "Salt, oil, water in the cupboard",
            "Sal, aceite y agua en la despensa",
        )))
        .child(div().class("ui-button").text(l.t(
            "🔎 Trouver une recette",
            "🔎 Find a recipe",
            "🔎 Buscar una receta",
        )));

    let demos = [
        Demo {
            name: "Chicken Congee",
            meta: "Chicken · Chinese",
            minutes: 94,
            missing: 3,
            cover: 67,
            tone: "sun",
            best: true,
        },
        Demo {
            name: "Kung Pao Chicken",
            meta: "Chicken · Chinese",
            minutes: 35,
            missing: 4,
            cover: 50,
            tone: "mint",
            best: false,
        },
        Demo {
            name: "Leek and Rice Bake",
            meta: "Vegetarian · Albanian",
            minutes: 90,
            missing: 3,
            cover: 40,
            tone: "sky",
            best: false,
        },
    ];
    let results = div()
        .class("screen__body")
        .children(demos.iter().map(|d| demo_card(l, d)));

    let steps: [&'static str; 3] = match l {
        Lang::Fr => [
            "Rincer le riz",
            "Porter l'eau à ébullition",
            "Ajouter le poulet",
        ],
        Lang::En => [
            "Rinse the rice",
            "Bring the water to a boil",
            "Add the chicken",
        ],
        Lang::Es => [
            "Lavar el arroz",
            "Llevar el agua a ebullición",
            "Añadir el pollo",
        ],
    };
    let recipe = div()
        .class("screen__body")
        .child(
            p().class("ui-section")
                .text(l.t("Ingrédients", "Ingredients", "Ingredientes")),
        )
        .child(
            ul().class("ui-list")
                .child(li().class("ok").text("✓ Chicken · 8 oz"))
                .child(li().class("ok").text("✓ Rice · 1/2 cup"))
                .child(li().class("ok").text("✓ Spring Onions · 1 tbs"))
                .child(li().class("miss").text("✗ Ginger · 1 tsp")),
        )
        .child(
            p().class("ui-section")
                .text(l.t("Étapes", "Steps", "Pasos")),
        )
        .child(
            ol().class("ui-steps")
                .children(steps.iter().enumerate().map(|(i, step)| {
                    let item = li().child(span().text(format!("{}", i + 1))).text(*step);
                    if i < 2 { item.class("is-done") } else { item }
                })),
        )
        .child(div().class("ui-button ui-button--tomato").text(l.t(
            "▶ Voir la vidéo",
            "▶ Watch the video",
            "▶ Ver el vídeo",
        )));

    let items = [
        (
            l.t("Mon frigo", "My fridge", "Mi nevera"),
            "sun",
            fridge,
            l.t("1. Remplissez le frigo", "1. Fill the fridge", "1. Llena la nevera"),
            l.t(
                "Tapez vos ingrédients ou touchez une suggestion. Le frigo reste enregistré.",
                "Type your ingredients or tap a suggestion. Your fridge is saved.",
                "Escribe tus ingredientes o toca una sugerencia. Tu nevera queda guardada.",
            ),
        ),
        (
            l.t("Recettes", "Recipes", "Recetas"),
            "mint",
            results,
            l.t("2. Comparez", "2. Compare", "2. Compara"),
            l.t(
                "La meilleure en premier\u{a0}: ce qui manque, le temps, la part de votre frigo utilisée.",
                "The best one first: what's missing, the time, how much of your fridge it uses.",
                "La mejor primero: lo que falta, el tiempo y cuánto de tu nevera aprovecha.",
            ),
        ),
        (
            "Chicken Congee",
            "sky",
            recipe,
            l.t("3. Cuisinez", "3. Cook", "3. Cocina"),
            l.t(
                "Les ingrédients cochés, les étapes à barrer une à une, et la vidéo quand il y en a une.",
                "Ingredients ticked, steps to cross off one by one, and the video when there is one.",
                "Los ingredientes marcados, los pasos para tachar uno a uno y el vídeo cuando lo hay.",
            ),
        ),
    ];
    let alt = |title: &str| match l {
        Lang::Fr => format!("Écran «\u{a0}{title}\u{a0}» de l'app Frigo vide"),
        Lang::En => format!("The “{title}” screen of the Frigo vide app"),
        Lang::Es => format!("Pantalla «{title}» de la app Frigo vide"),
    };
    section()
        .class("section section--band section--sky")
        .id("ecrans")
        .child(head(
            l.t("L'app", "The app", "La app"),
            l.t("Trois écrans, un repas", "Three screens, one meal", "Tres pantallas, una comida"),
            Some(l.t(
                "Frigo vide est une app iPhone. Tout se passe dedans\u{a0}: voici à quoi elle ressemble.",
                "Frigo vide is an iPhone app. Everything happens inside it: here's what it looks like.",
                "Frigo vide es una app para iPhone. Todo pasa dentro: así es como se ve.",
            )),
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
                            .attr("aria-label", alt(title))
                            .child(screen(title, tone, body)),
                    )
                    .child(figcaption().child(strong().text(caption)).child(span().text(text)))
            },
        )))
}

/// Les fonctions de l'app, en tuiles colorées.
pub fn features(l: Lang) -> Element {
    let items = [
        (
            "🧊",
            l.t("Votre frigo, gardé", "Your fridge, saved", "Tu nevera, guardada"),
            l.t(
                "Vos ingrédients restent dans l'app d'une fois sur l'autre. Un geste pour en retirer, un bouton pour tout vider.",
                "Your ingredients stay in the app from one time to the next. Swipe to remove one, one button to clear them all.",
                "Tus ingredientes se quedan en la app de una vez para otra. Un gesto para quitar uno, un botón para vaciarlo todo.",
            ),
            "sun",
        ),
        (
            "🌍",
            l.t("Dans votre langue", "In your language", "En tu idioma"),
            l.t(
                "Français, anglais ou espagnol\u{a0}: «\u{a0}poulet\u{a0}», «\u{a0}pollo\u{a0}» ou «\u{a0}chicken\u{a0}», l'app traduit pour TheMealDB, pluriels et accents compris.",
                "English, French or Spanish: “chicken”, “poulet” or “pollo”, the app translates for TheMealDB, plurals and accents included.",
                "Español, inglés o francés: «pollo», «chicken» o «poulet», la app traduce para TheMealDB, con plurales y acentos.",
            ),
            "sky",
        ),
        (
            "🛒",
            l.t("Sans courses", "No shopping", "Sin compras"),
            l.t(
                "Le mode strict ne garde que les recettes faisables avec ce que vous avez déjà.",
                "Strict mode only keeps the recipes you can make with what you already have.",
                "El modo estricto solo deja las recetas que puedes hacer con lo que ya tienes.",
            ),
            "mint",
        ),
        (
            "🧂",
            l.t("Le placard", "The cupboard", "La despensa"),
            l.t(
                "Sel, poivre, huile, eau et sucre sont comptés comme acquis. Un interrupteur les retire.",
                "Salt, pepper, oil, water and sugar count as already there. One switch takes them out.",
                "Sal, pimienta, aceite, agua y azúcar cuentan como disponibles. Un interruptor los quita.",
            ),
            "peach",
        ),
        (
            "⏱️",
            l.t("Le temps estimé", "Estimated time", "Tiempo estimado"),
            l.t(
                "Lu dans les instructions («\u{a0}bake 35 minutes\u{a0}»), plus le temps de préparer chaque ingrédient.",
                "Read from the instructions (“bake 35 minutes”), plus the time to prepare each ingredient.",
                "Leído en las instrucciones («bake 35 minutes»), más el tiempo de preparar cada ingrediente.",
            ),
            "grape",
        ),
        (
            "🏆",
            l.t("La meilleure d'abord", "The best first", "La mejor primero"),
            l.t(
                "Le moins d'ingrédients manquants, puis le frigo le mieux utilisé, puis la plus rapide.",
                "The fewest missing ingredients, then the best use of your fridge, then the quickest.",
                "La que menos ingredientes necesita, luego la que más aprovecha la nevera, luego la más rápida.",
            ),
            "tomato",
        ),
        (
            "✅",
            l.t("Étapes à cocher", "Steps to tick", "Pasos para marcar"),
            l.t(
                "Touchez une étape pour la barrer\u{a0}: l'iPhone vibre doucement, vous savez où vous en êtes.",
                "Tap a step to cross it off: your iPhone gives a gentle buzz, so you know where you are.",
                "Toca un paso para tacharlo: el iPhone vibra suavemente y sabes por dónde vas.",
            ),
            "mint",
        ),
        (
            "▶️",
            l.t("La vidéo", "The video", "El vídeo"),
            l.t(
                "Quand TheMealDB a une vidéo de la recette, elle est à un toucher.",
                "When TheMealDB has a video of the recipe, it's one tap away.",
                "Cuando TheMealDB tiene un vídeo de la receta, está a un toque.",
            ),
            "sun",
        ),
    ];
    section()
        .class("section")
        .id("fonctions")
        .child(head(
            l.t("Fonctionnalités", "Features", "Funciones"),
            l.t(
                "Tout ce qu'il faut, rien de plus",
                "Everything you need, nothing more",
                "Todo lo que hace falta, nada más",
            ),
            Some(l.t(
                "Pas de compte à créer, pas de liste de courses. Juste la bonne recette.",
                "No account to create, no shopping list. Just the right recipe.",
                "Sin cuenta que crear, sin lista de la compra. Solo la receta adecuada.",
            )),
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
pub fn numbers(l: Lang) -> Element {
    let items = [
        (
            "24",
            l.t(
                "recettes comparées, au plus, à chaque recherche",
                "recipes compared, at most, per search",
                "recetas comparadas, como máximo, en cada búsqueda",
            ),
            "sun",
        ),
        (
            "20",
            l.t(
                "ingrédients lus par recette",
                "ingredients read per recipe",
                "ingredientes leídos por receta",
            ),
            "mint",
        ),
        (
            "12",
            l.t(
                "suggestions à un toucher",
                "suggestions one tap away",
                "sugerencias a un toque",
            ),
            "sky",
        ),
        (
            "3",
            l.t(
                "langues\u{a0}: français, anglais, espagnol",
                "languages: English, French, Spanish",
                "idiomas: español, inglés, francés",
            ),
            "tomato",
        ),
    ];
    section()
        .class("numbers")
        .attr(
            "aria-label",
            l.t(
                "Frigo vide en chiffres",
                "Frigo vide in numbers",
                "Frigo vide en cifras",
            ),
        )
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
pub fn pantry(l: Lang) -> Element {
    let jars = [
        ("🧂", l.t("Sel", "Salt", "Sal"), "sky"),
        ("🌶️", l.t("Poivre", "Pepper", "Pimienta"), "tomato"),
        ("🫒", l.t("Huile", "Oil", "Aceite"), "sun"),
        ("💧", l.t("Eau", "Water", "Agua"), "mint"),
        ("🍬", l.t("Sucre", "Sugar", "Azúcar"), "grape"),
    ];
    section()
        .class("section split")
        .id("placard")
        .child(div().child(head(
            l.t("Le placard", "The cupboard", "La despensa"),
            l.t("Ce qu'on a toujours", "What you always have", "Lo que siempre tienes"),
            Some(l.t(
                "Personne ne note «\u{a0}sel\u{a0}» sur sa liste. Frigo vide non plus\u{a0}: ces cinq-là ne comptent jamais comme manquants. Si votre placard est vraiment vide, un interrupteur de l'app les retire.",
                "Nobody writes “salt” on their shopping list. Neither does Frigo vide: these five never count as missing. If your cupboard really is bare, a switch in the app takes them out.",
                "Nadie apunta «sal» en la lista de la compra. Frigo vide tampoco: estos cinco nunca cuentan como faltantes. Si tu despensa está de verdad vacía, un interruptor de la app los quita.",
            )),
        )))
        .child(
            div()
                .class("shelf")
                .attr("data-reveal", "")
                .attr("role", "img")
                .attr(
                    "aria-label",
                    l.t(
                        "Une étagère avec du sel, du poivre, de l'huile, de l'eau et du sucre",
                        "A shelf with salt, pepper, oil, water and sugar",
                        "Una estantería con sal, pimienta, aceite, agua y azúcar",
                    ),
                )
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
pub fn week(l: Lang) -> Element {
    let days: [(&str, &str, &str, &str, &str); 7] = match l {
        Lang::Fr => [
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
        ],
        Lang::En => [
            (
                "Mon",
                "chicken · rice · onion",
                "🍲",
                "Chicken congee",
                "sun",
            ),
            ("Tue", "eggs · potatoes", "🥔", "Tortilla", "peach"),
            ("Wed", "milk · eggs · flour", "🥞", "Pancakes", "sky"),
            ("Thu", "pasta · eggs · cheese", "🍝", "Carbonara", "mint"),
            (
                "Fri",
                "tomatoes · eggs · pepper",
                "🍳",
                "Shakshuka",
                "tomato",
            ),
            ("Sat", "rice · veg · eggs", "🍚", "Fried rice", "grape"),
            ("Sun", "carrots · leeks", "🥣", "Homemade soup", "sun"),
        ],
        Lang::Es => [
            (
                "Lun",
                "pollo · arroz · cebolla",
                "🍲",
                "Congee de pollo",
                "sun",
            ),
            ("Mar", "huevos · patatas", "🥔", "Tortilla", "peach"),
            ("Mié", "leche · huevos · harina", "🥞", "Crepes", "sky"),
            ("Jue", "pasta · huevos · queso", "🍝", "Carbonara", "mint"),
            (
                "Vie",
                "tomates · huevos · pimiento",
                "🍳",
                "Shakshuka",
                "tomato",
            ),
            (
                "Sáb",
                "arroz · verduras · huevos",
                "🍚",
                "Arroz frito",
                "grape",
            ),
            ("Dom", "zanahorias · puerros", "🥣", "Sopa casera", "sun"),
        ],
    };
    section()
        .class("section section--band section--peach")
        .id("semaine")
        .child(head(
            l.t("Une semaine", "One week", "Una semana"),
            l.t(
                "Sept soirs, zéro courses",
                "Seven nights, zero shopping",
                "Siete noches, cero compras",
            ),
            Some(l.t(
                "Un exemple de semaine avec ce qui traîne au frigo. Chaque soir, l'app propose, vous cuisinez.",
                "An example week with whatever's lying around in the fridge. Every night, the app suggests, you cook.",
                "Un ejemplo de semana con lo que anda por la nevera. Cada noche, la app propone y tú cocinas.",
            )),
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
pub fn leftovers(l: Lang) -> Element {
    let items = [
        (
            "🍚",
            l.t("Le riz d'hier", "Yesterday's rice", "El arroz de ayer"),
            "🍳",
            l.t("Riz sauté", "Fried rice", "Arroz frito"),
            "sun",
        ),
        (
            "🥖",
            l.t("Le pain rassis", "Stale bread", "El pan duro"),
            "🍞",
            l.t("Pain perdu", "French toast", "Torrijas"),
            "peach",
        ),
        (
            "🍌",
            l.t(
                "Les bananes trop mûres",
                "Overripe bananas",
                "Plátanos demasiado maduros",
            ),
            "🍰",
            "Banana bread",
            "mint",
        ),
        (
            "🥕",
            l.t(
                "Les légumes fatigués",
                "Tired vegetables",
                "Verduras cansadas",
            ),
            "🥣",
            l.t("Soupe", "Soup", "Sopa"),
            "sky",
        ),
        (
            "🧀",
            l.t("Les bouts de fromage", "Cheese ends", "Restos de queso"),
            "🥘",
            l.t("Gratin", "Gratin", "Gratinado"),
            "grape",
        ),
        (
            "🌿",
            l.t("Les herbes flétries", "Wilted herbs", "Hierbas mustias"),
            "🫙",
            "Pesto",
            "tomato",
        ),
    ];
    section()
        .class("section")
        .id("anti-gaspi")
        .child(head(
            l.t("Anti-gaspi", "Zero waste", "Antidesperdicio"),
            l.t(
                "Les restes ont du talent",
                "Leftovers have talent",
                "Las sobras tienen talento",
            ),
            Some(l.t(
                "Ce qui allait finir à la poubelle fait souvent le meilleur repas. Mettez-le dans le frigo de l'app et voyez ce qui en sort.",
                "What was about to end up in the bin often makes the best meal. Put it in the app's fridge and see what comes out.",
                "Lo que iba a acabar en la basura suele dar la mejor comida. Ponlo en la nevera de la app y mira lo que sale.",
            )),
        ))
        .child(ul().class("swaps").children(items.iter().enumerate().map(
            |(i, (from_icon, from, to_icon, to, tone))| {
                li().class(format!("swap swap--{tone}"))
                    .attr("data-reveal", "")
                    .style(format!("--d:{}ms", (i % 3) * 100))
                    .child(
                        span()
                            .class("swap__from")
                            .child(
                                span()
                                    .class("swap__icon")
                                    .attr("aria-hidden", "true")
                                    .text(*from_icon),
                            )
                            .child(span().text(*from)),
                    )
                    .child(span().class("swap__arrow").attr("aria-hidden", "true").text("→"))
                    .child(
                        span()
                            .class("swap__to")
                            .child(
                                span()
                                    .class("swap__icon")
                                    .attr("aria-hidden", "true")
                                    .text(*to_icon),
                            )
                            .child(strong().text(*to)),
                    )
            },
        )))
}

/// Les cuisines du monde de TheMealDB.
pub fn cuisines(l: Lang) -> Element {
    let items = [
        ("🇫🇷", l.t("Française", "French", "Francesa")),
        ("🇮🇹", l.t("Italienne", "Italian", "Italiana")),
        ("🇯🇵", l.t("Japonaise", "Japanese", "Japonesa")),
        ("🇲🇽", l.t("Mexicaine", "Mexican", "Mexicana")),
        ("🇮🇳", l.t("Indienne", "Indian", "India")),
        ("🇲🇦", l.t("Marocaine", "Moroccan", "Marroquí")),
        ("🇨🇳", l.t("Chinoise", "Chinese", "China")),
        ("🇬🇷", l.t("Grecque", "Greek", "Griega")),
        ("🇹🇭", l.t("Thaïe", "Thai", "Tailandesa")),
        ("🇯🇲", l.t("Jamaïcaine", "Jamaican", "Jamaicana")),
        ("🇹🇷", l.t("Turque", "Turkish", "Turca")),
        ("🇻🇳", l.t("Vietnamienne", "Vietnamese", "Vietnamita")),
        ("🇪🇸", l.t("Espagnole", "Spanish", "Española")),
        ("🇵🇹", l.t("Portugaise", "Portuguese", "Portuguesa")),
        ("🇹🇳", l.t("Tunisienne", "Tunisian", "Tunecina")),
        ("🇬🇧", l.t("Britannique", "British", "Británica")),
    ];
    let tones = ["sun", "mint", "sky", "peach", "grape", "tomato"];
    section()
        .class("section section--band section--mint section--center")
        .id("cuisines")
        .child(head(
            l.t("Le tour du monde", "Around the world", "La vuelta al mundo"),
            l.t(
                "Un tour du monde en cuisine",
                "A world tour in your kitchen",
                "La vuelta al mundo en la cocina",
            ),
            Some(l.t(
                "Les recettes viennent de TheMealDB, une base de recettes du monde entier. Ce soir, ce sera peut-être marocain.",
                "The recipes come from TheMealDB, a database of recipes from all over the world. Tonight might be Moroccan.",
                "Las recetas vienen de TheMealDB, una base de recetas de todo el mundo. Esta noche, quizá toque marroquí.",
            )),
        ))
        .child(ul().class("globe").children(items.iter().enumerate().map(|(i, (flag, name))| {
            li().class(format!("globe__item globe__item--{}", tones[i % tones.len()]))
                .style(format!("--i:{i}"))
                .child(span().attr("aria-hidden", "true").text(*flag))
                .child(span().text(*name))
        })))
}

/// La vie privée : le frigo reste sur l'iPhone.
pub fn privacy(l: Lang) -> Element {
    let items = [
        (
            "📱",
            l.t("Sur votre iPhone", "On your iPhone", "En tu iPhone"),
            l.t(
                "Votre frigo est enregistré dans l'app, sur l'appareil. Aucun serveur à nous ne le reçoit.",
                "Your fridge is saved in the app, on the device. No server of ours ever receives it.",
                "Tu nevera se guarda en la app, en el dispositivo. Ningún servidor nuestro la recibe.",
            ),
        ),
        (
            "🔎",
            l.t("Juste la recherche", "Just the search", "Solo la búsqueda"),
            l.t(
                "Seuls les noms des ingrédients partent vers TheMealDB, le temps de chercher les recettes.",
                "Only ingredient names go to TheMealDB, just long enough to look up recipes.",
                "Solo los nombres de los ingredientes van a TheMealDB, el tiempo de buscar las recetas.",
            ),
        ),
        (
            "🙅",
            l.t("Ni compte ni traçage", "No account, no tracking", "Ni cuenta ni rastreo"),
            l.t(
                "Pas d'inscription, pas de traçage. L'app ne sait pas qui vous êtes.",
                "No sign-up, no tracking. The app doesn't know who you are.",
                "Sin registro, sin rastreo. La app no sabe quién eres.",
            ),
        ),
    ];
    section()
        .class("section split split--reverse")
        .id("confidentialite")
        .child(
            div()
                .child(head(
                    l.t("Confidentialité", "Privacy", "Privacidad"),
                    l.t(
                        "Votre frigo ne regarde que vous",
                        "Your fridge is your business",
                        "Tu nevera es cosa tuya",
                    ),
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
pub fn faq(l: Lang) -> Element {
    let items = [
        (
            l.t("L'app est-elle gratuite\u{a0}?", "Is the app free?", "¿La app es gratis?"),
            l.t(
                "Elle est gratuite à télécharger, avec des fonctions premium en option. Son code est ouvert, sous licence MIT.",
                "It's free to download, with optional premium features. Its code is open source, under the MIT license.",
                "Es gratis para descargar, con funciones premium opcionales. Su código es abierto, con licencia MIT.",
            ),
        ),
        (
            l.t(
                "Quand sera-t-elle sur l'App Store\u{a0}?",
                "When will it be on the App Store?",
                "¿Cuándo estará en el App Store?",
            ),
            l.t(
                "Bientôt. En attendant, le code est sur GitHub, et l'app se compile avec Xcode pour iOS 26.",
                "Soon. In the meantime, the code is on GitHub, and the app builds with Xcode for iOS 26.",
                "Pronto. Mientras tanto, el código está en GitHub y la app se compila con Xcode para iOS 26.",
            ),
        ),
        (
            l.t("Et sur Android\u{a0}?", "What about Android?", "¿Y en Android?"),
            l.t(
                "Pas pour l'instant\u{a0}: Frigo vide est écrite en SwiftUI, pour l'iPhone.",
                "Not for now: Frigo vide is written in SwiftUI, for iPhone.",
                "Por ahora no: Frigo vide está escrita en SwiftUI, para iPhone.",
            ),
        ),
        (
            l.t("Dans quelles langues\u{a0}?", "Which languages?", "¿En qué idiomas?"),
            l.t(
                "L'app et ce site parlent français, anglais et espagnol, et l'app comprend les ingrédients tapés dans ces trois langues.",
                "The app and this site speak English, French and Spanish, and the app understands ingredients typed in all three.",
                "La app y esta web hablan español, inglés y francés, y la app entiende los ingredientes escritos en los tres idiomas.",
            ),
        ),
        (
            l.t(
                "D'où viennent les recettes\u{a0}?",
                "Where do the recipes come from?",
                "¿De dónde vienen las recetas?",
            ),
            l.t(
                "De TheMealDB, une base gratuite de recettes du monde entier. Elles sont en anglais, mais vous tapez vos ingrédients dans votre langue.",
                "From TheMealDB, a free database of recipes from all over the world. The recipes are in English.",
                "De TheMealDB, una base gratuita de recetas de todo el mundo. Están en inglés, pero escribes tus ingredientes en tu idioma.",
            ),
        ),
        (
            l.t("Faut-il du réseau\u{a0}?", "Do I need a connection?", "¿Hace falta conexión?"),
            l.t(
                "Pour chercher des recettes, oui. Votre frigo, lui, reste enregistré dans l'app.",
                "To search for recipes, yes. Your fridge itself stays saved in the app.",
                "Para buscar recetas, sí. Tu nevera, en cambio, queda guardada en la app.",
            ),
        ),
        (
            l.t(
                "Comment le temps est-il calculé\u{a0}?",
                "How is the time worked out?",
                "¿Cómo se calcula el tiempo?",
            ),
            l.t(
                "TheMealDB ne donne pas de durée\u{a0}: l'app la lit dans les instructions («\u{a0}simmer 20 minutes\u{a0}») et ajoute un peu de temps par ingrédient. C'est une estimation.",
                "TheMealDB doesn't give a duration: the app reads it from the instructions (“simmer 20 minutes”) and adds a little time per ingredient. It's an estimate.",
                "TheMealDB no da la duración: la app la lee en las instrucciones («simmer 20 minutes») y añade un poco de tiempo por ingrediente. Es una estimación.",
            ),
        ),
        (
            l.t(
                "Pourquoi cette recette plutôt qu'une autre\u{a0}?",
                "Why this recipe rather than another?",
                "¿Por qué esta receta y no otra?",
            ),
            l.t(
                "L'app classe d'abord par ingrédients manquants, puis par part de votre frigo utilisée, puis par temps. La première est le «\u{a0}Meilleur choix\u{a0}».",
                "The app ranks by missing ingredients first, then by how much of your fridge is used, then by time. The top one is the “Best match”.",
                "La app ordena primero por ingredientes que faltan, luego por cuánto aprovecha tu nevera y luego por tiempo. La primera es la «Mejor opción».",
            ),
        ),
    ];
    section()
        .class("section")
        .id("faq")
        .child(head(
            l.t("Questions", "Questions", "Preguntas"),
            l.t("On vous répond", "We've got answers", "Te respondemos"),
            None,
        ))
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
pub fn app_store(l: Lang) -> Element {
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
                        .child(p().class("eyebrow").text(l.t("Bientôt", "Coming soon", "Muy pronto")))
                        .child(h2().class("title").text(l.t(
                            "Frigo vide arrive sur l'App Store",
                            "Frigo vide is coming to the App Store",
                            "Frigo vide llega al App Store",
                        )))
                        .child(p().class("lead").text(l.t(
                            "Gratuite à télécharger, sans compte. En attendant, suivez le projet sur GitHub, ou partagez-le à ceux qui ont toujours un frigo à moitié vide.",
                            "Free to download, no account. In the meantime, follow the project on GitHub, or share it with anyone whose fridge is always half empty.",
                            "Gratis para descargar, sin cuenta. Mientras tanto, sigue el proyecto en GitHub o compártelo con quien siempre tiene la nevera medio vacía.",
                        )))
                        .child(
                            div()
                                .class("ctas")
                                .child(
                                    span()
                                        .class("btn btn--ink btn--big btn--soon")
                                        .attr("aria-disabled", "true")
                                        .child(span().attr("aria-hidden", "true").text("📲"))
                                        .child(span().text(l.t(
                                            "Bientôt sur l'App Store",
                                            "Coming soon to the App Store",
                                            "Muy pronto en el App Store",
                                        ))),
                                )
                                .child(
                                    a().class("btn btn--white btn--big")
                                        .href(REPO)
                                        .attr("rel", "noopener")
                                        .text(l.t(
                                            "Suivre sur GitHub",
                                            "Follow on GitHub",
                                            "Seguir en GitHub",
                                        )),
                                )
                                .child(
                                    a().class("btn btn--sun btn--big")
                                        .href("#partager")
                                        .text(l.t("Partager", "Share", "Compartir")),
                                ),
                        ),
                ),
        )
}

/// Une ligne d'offre : une coche, le texte, et « bientôt » pour ce qui n'existe pas encore.
fn perk(l: Lang, text: &'static str, detail: Option<&'static str>, soon: bool) -> Element {
    let mut item = li().class("plan__perk").child(
        span()
            .class("plan__check")
            .attr("aria-hidden", "true")
            .text("✓"),
    );
    let mut body = span().class("plan__text").child(strong().text(text));
    if let Some(detail) = detail {
        body = body.child(span().text(detail));
    }
    item = item.child(body);
    if soon {
        item = item.child(
            span()
                .class("plan__soon")
                .text(l.t("bientôt", "soon", "pronto")),
        );
    }
    item
}

/// Les tarifs : Gratuit (tout ce que fait l'app aujourd'hui) et Premium (en préparation).
pub fn pricing(l: Lang) -> Element {
    let free = div()
        .class("plan plan--free")
        .attr("data-reveal", "")
        .child(
            p().class("plan__name")
                .text(l.t("Gratuit", "Free", "Gratis")),
        )
        .child(
            p().class("plan__price")
                .child(strong().text(l.t("0\u{a0}€", "$0", "0\u{a0}€")))
                .child(span().text(l.t(
                    "pour cuisiner tous les soirs",
                    "to cook every night",
                    "para cocinar cada noche",
                ))),
        )
        .child(
            ul().class("plan__perks")
                .child(perk(
                    l,
                    l.t(
                        "Le frigo enregistré",
                        "Your saved fridge",
                        "La nevera guardada",
                    ),
                    Some(l.t(
                        "et 12 suggestions à un toucher",
                        "and 12 one-tap suggestions",
                        "y 12 sugerencias a un toque",
                    )),
                    false,
                ))
                .child(perk(
                    l,
                    l.t(
                        "La recherche de recettes",
                        "Recipe search",
                        "La búsqueda de recetas",
                    ),
                    Some(l.t(
                        "la meilleure en premier",
                        "the best one first",
                        "la mejor primero",
                    )),
                    false,
                ))
                .child(perk(
                    l,
                    l.t(
                        "Sans courses et le placard",
                        "No shopping and the cupboard",
                        "Sin compras y la despensa",
                    ),
                    None,
                    false,
                ))
                .child(perk(
                    l,
                    l.t("Le temps estimé", "Estimated time", "El tiempo estimado"),
                    None,
                    false,
                ))
                .child(perk(
                    l,
                    l.t(
                        "Les étapes à cocher et la vidéo",
                        "Steps to tick and the video",
                        "Los pasos para marcar y el vídeo",
                    ),
                    None,
                    false,
                ))
                .child(perk(
                    l,
                    l.t(
                        "En français, anglais et espagnol",
                        "In English, French and Spanish",
                        "En español, inglés y francés",
                    ),
                    None,
                    false,
                )),
        )
        .child(
            a().class("btn btn--white btn--big plan__cta")
                .href("#telecharger")
                .text(l.t(
                    "Bientôt sur l'App Store",
                    "Coming soon to the App Store",
                    "Muy pronto en el App Store",
                )),
        );

    let premium = div()
        .class("plan plan--premium")
        .attr("data-reveal", "")
        .style("--d:120ms")
        .child(span().class("plan__badge").text(l.t(
            "En préparation",
            "In the works",
            "En preparación",
        )))
        .child(p().class("plan__name").text("Premium"))
        .child(
            p().class("plan__price plan__price--tbd")
                .child(strong().text(l.t(
                    "Prix annoncé à la sortie",
                    "Price announced at launch",
                    "Precio anunciado en el lanzamiento",
                )))
                .child(span().text(l.t(
                    "pour aller plus loin",
                    "to go further",
                    "para ir más lejos",
                ))),
        )
        .child(
            ul().class("plan__perks")
                .child(perk(
                    l,
                    l.t(
                        "Tout ce qui est gratuit",
                        "Everything in Free",
                        "Todo lo gratuito",
                    ),
                    None,
                    false,
                ))
                .child(perk(
                    l,
                    l.t(
                        "La liste de courses",
                        "Shopping list",
                        "La lista de la compra",
                    ),
                    Some(l.t(
                        "les ingrédients manquants ajoutés d'un toucher",
                        "missing ingredients added in one tap",
                        "los ingredientes que faltan, añadidos de un toque",
                    )),
                    true,
                ))
                .child(perk(
                    l,
                    l.t(
                        "Le planning de la semaine",
                        "Weekly meal plan",
                        "La planificación de la semana",
                    ),
                    Some(l.t(
                        "sept soirs prévus avec ce qu'il y a au frigo",
                        "seven nights planned with what's in the fridge",
                        "siete noches planificadas con lo que hay en la nevera",
                    )),
                    true,
                ))
                .child(perk(
                    l,
                    l.t(
                        "Les filtres avancés",
                        "Advanced filters",
                        "Filtros avanzados",
                    ),
                    Some(l.t(
                        "temps maximum, catégorie, cuisine du monde",
                        "maximum time, category, world cuisine",
                        "tiempo máximo, categoría, cocina del mundo",
                    )),
                    true,
                )),
        )
        .child(
            span()
                .class("btn btn--ink btn--big btn--soon plan__cta")
                .attr("aria-disabled", "true")
                .text(l.t("Bientôt", "Coming soon", "Muy pronto")),
        );

    section()
        .class("section section--center")
        .id("tarifs")
        .child(head(
            l.t("Tarifs", "Pricing", "Precios"),
            l.t(
                "Gratuit pour cuisiner, Premium pour s'organiser",
                "Free to cook, Premium to get organised",
                "Gratis para cocinar, Premium para organizarte",
            ),
            Some(l.t(
                "Tout ce que fait l'app aujourd'hui est gratuit. Premium ajoutera de quoi préparer la semaine.",
                "Everything the app does today is free. Premium will add what you need to plan the week.",
                "Todo lo que hace la app hoy es gratis. Premium añadirá lo necesario para preparar la semana.",
            )),
        ))
        .child(div().class("pricing").child(free).child(premium))
        .child(p().class("pricing__note").text(l.t(
            "Les fonctions Premium sont en préparation\u{a0}: elles et leur prix seront annoncés à la sortie de l'app.",
            "Premium features are in the works: they and their price will be announced when the app launches.",
            "Las funciones Premium están en preparación: ellas y su precio se anunciarán con el lanzamiento de la app.",
        )))
}
