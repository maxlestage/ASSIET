//! La page d'accueil, rendue par le serveur : tout le texte est dans le HTML (lisible
//! sans WebAssembly, et par les moteurs de recherche) ; les îlots et le mouvement
//! arrivent ensuite. Le style est plat, en 2D, et très coloré. Trois langues : le
//! français à la racine, l'anglais sous `/en/`, l'espagnol sous `/es/`.

use paquets::prelude::*;

use super::share::{current_year, display_url, networks, qr_svg, share_text};
use crate::i18n::Lang;
use crate::islands::frigo_dessin;

mod showcase;

const REPO: &str = "https://github.com/maxlestage/ASSIET";
const ISSUES: &str = "https://github.com/maxlestage/ASSIET/issues";
const README: &str = "https://github.com/maxlestage/ASSIET#readme";
const LICENSE: &str = "https://github.com/maxlestage/ASSIET/blob/master/LICENSE";
const PAQUETS: &str = "https://github.com/maxlestage/Paquets";
const MEALDB: &str = "https://www.themealdb.com/";

/// Ce que chaque page doit savoir : la version de la feuille de style, l'adresse
/// publique du site (pour les liens de partage, le QR code et les aperçus) et la langue.
pub struct Site {
    css_version: String,
    url: String,
    lang: Lang,
}

impl Site {
    pub fn new(css_version: &str, url: String, lang: Lang) -> Self {
        Self {
            css_version: css_version.to_owned(),
            url,
            lang,
        }
    }

    /// L'adresse publique de la page d'accueil dans la langue `lang`.
    fn url_in(&self, lang: Lang) -> String {
        format!("{}{}", self.url, lang.home().trim_start_matches('/'))
    }

    /// L'adresse publique de cette page.
    fn page_url(&self) -> String {
        self.url_in(self.lang)
    }
}

/// Un lien vers une section de la page d'accueil, dans la langue `l` : `/en/#faq`.
fn anchor(l: Lang, id: &str) -> String {
    format!("{}#{id}", l.home())
}

fn layout(site: &Site, title: &str, content: Element) -> Page {
    let l = site.lang;
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
    let mut seo = Seo::new()
        .site_name("Frigo vide")
        .og_type("website")
        .locale(l.locale())
        .image(format!("{}assets/{}", site.url, og_image(l)))
        .meta_property("og:image:width", "2400")
        .meta_property("og:image:height", "1260")
        .meta_property(
            "og:image:alt",
            l.t(
                "Frigo vide : le frigo est vide ? Pas l'assiette.",
                "Frigo vide: empty fridge? Full plate.",
                "Frigo vide: ¿la nevera vacía? El plato no.",
            ),
        )
        .canonical(site.page_url());
    for other in Lang::ALL {
        seo = seo.alternate(other.code(), site.url_in(other));
    }
    seo = seo.alternate("x-default", site.url_in(Lang::Fr));
    Page::new(title)
        .lang(l.code())
        .description(l.t(
            "Frigo vide, l'app iPhone qui lit vos ingrédients, interroge TheMealDB et trouve la meilleure recette à préparer. Bientôt sur l'App Store : gratuite, sans compte, sans publicité.",
            "Frigo vide, the iPhone app that reads your ingredients, searches TheMealDB and finds the best recipe to cook. Coming soon to the App Store: free, no account, no ads.",
            "Frigo vide, la app para iPhone que lee tus ingredientes, busca en TheMealDB y encuentra la mejor receta. Muy pronto en el App Store: gratis, sin cuenta, sin anuncios.",
        ))
        .seo(seo)
        .plain()
        .stylesheet(format!("/site.css?v={}", site.css_version))
        .head(head)
        .body(
            div()
                .class("site")
                .child(a().class("skip").href("#main").text(l.t(
                    "Aller au contenu",
                    "Skip to content",
                    "Ir al contenido",
                )))
                .child(navbar(l))
                .child(content.id("main"))
                .child(site_footer(&site.page_url(), l)),
        )
}

/// L'image des aperçus de partage, dans la langue de la page.
fn og_image(l: Lang) -> &'static str {
    l.t("og.png", "og-en.png", "og-es.png")
}

pub fn page(site: &Site) -> Page {
    let l = site.lang;
    layout(
        site,
        l.t(
            "Frigo vide — la recette la plus rapide avec ce qu'il reste dans le frigo",
            "Frigo vide — the quickest recipe with what's left in your fridge",
            "Frigo vide — la receta más rápida con lo que queda en la nevera",
        ),
        html::main()
            .child(hero(l))
            .child(marquee(l))
            .child(steps(l))
            .child(showcase::screens(l))
            .child(showcase::features(l))
            .child(showcase::numbers(l))
            .child(ios(l))
            .child(showcase::pantry(l))
            .child(showcase::week(l))
            .child(dishes_band(l))
            .child(showcase::leftovers(l))
            .child(showcase::cuisines(l))
            .child(showcase::privacy(l))
            .child(tech_band(l))
            .child(architecture(l))
            .child(showcase::faq(l))
            .child(showcase::app_store(l))
            .child(share(&site.page_url(), l)),
    )
}

pub fn not_found(site: &Site) -> Page {
    let l = site.lang;
    layout(
        site,
        l.t(
            "Page introuvable — Frigo vide",
            "Page not found — Frigo vide",
            "Página no encontrada — Frigo vide",
        ),
        html::main().child(
            section()
                .class("section section--center lost")
                .child(
                    span()
                        .class("lost__emoji")
                        .attr("aria-hidden", "true")
                        .text("🥡"),
                )
                .child(h1().class("title").text(l.t(
                    "Ce placard est vide",
                    "This cupboard is empty",
                    "Esta despensa está vacía",
                )))
                .child(p().class("lead").text(l.t(
                    "La page demandée n'existe pas (ou quelqu'un l'a mangée).",
                    "The page you asked for doesn't exist (or someone ate it).",
                    "La página que buscas no existe (o alguien se la comió).",
                )))
                .child(
                    a().class("btn btn--tomato btn--big")
                        .href(l.home())
                        .text(l.t(
                            "Retour à la cuisine",
                            "Back to the kitchen",
                            "Volver a la cocina",
                        )),
                ),
        ),
    )
}

/// Le sélecteur de langue : FR · EN · ES.
fn languages(l: Lang, class: &str) -> Element {
    nav()
        .class(format!("langs {class}"))
        .attr("aria-label", l.t("Langue", "Language", "Idioma"))
        .children(Lang::ALL.map(|other| {
            let link = a()
                .href(other.home())
                .attr("hreflang", other.code())
                .attr("lang", other.code())
                .attr("title", other.name())
                .text(other.code().to_uppercase());
            if other == l {
                link.class("is-current").attr("aria-current", "true")
            } else {
                link
            }
        }))
}

fn navbar(l: Lang) -> Element {
    header()
        .class("nav")
        // La barre de progression, accrochée sous la navigation : toujours visible, même
        // sur iPhone où le haut de l'écran passe sous la barre d'état.
        .child(div().class("progress").attr("aria-hidden", "true"))
        .child(
            a().class("nav__brand")
                .href(l.home())
                .attr(
                    "aria-label",
                    l.t(
                        "Frigo vide, accueil",
                        "Frigo vide, home",
                        "Frigo vide, inicio",
                    ),
                )
                .child(span().class("logo").attr("aria-hidden", "true").text("🧊"))
                .child(span().text("Frigo vide")),
        )
        .child(
            nav()
                .class("nav__links")
                .attr("aria-label", "Sections")
                .child(a().href("#ecrans").text(l.t("L'app", "The app", "La app")))
                .child(
                    a().href("#fonctions")
                        .text(l.t("Fonctions", "Features", "Funciones")),
                )
                .child(a().href("#faq").text(l.t("Questions", "FAQ", "Preguntas")))
                .child(
                    a().href("#partager")
                        .text(l.t("Partager", "Share", "Compartir")),
                ),
        )
        .child(
            div()
                .class("nav__end")
                .child(languages(l, "langs--nav"))
                .child(
                    a().class("btn btn--ink btn--small nav__soon")
                        .href("#telecharger")
                        .text(l.t("Bientôt", "Soon", "Pronto")),
                ),
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

fn hero(l: Lang) -> Element {
    section()
        .class("hero")
        .child(stickers())
        .child(
            div()
                .class("hero__text")
                .child(
                    p().class("kicker")
                        .child(span().text("iPhone"))
                        .child(span().text(l.t("Gratuit", "Free", "Gratis")))
                        .child(span().text(l.t("Sans compte", "No account", "Sin cuenta"))),
                )
                .child(
                    h1().class("hero__title")
                        .child(span().class("hero__line").text(l.t(
                            "Le frigo est vide\u{a0}?",
                            "Empty fridge?",
                            "¿La nevera vacía?",
                        )))
                        .child(
                            span()
                                .class("hero__line hero__line--mark")
                                .child(span().text(l.t(
                                    "Pas l'assiette.",
                                    "Full plate.",
                                    "El plato no.",
                                ))),
                        ),
                )
                .child(p().class("lead").text(l.t(
                    "Frigo vide est une app iPhone\u{a0}: dites-lui ce qu'il reste dans votre frigo, elle cherche parmi les recettes du monde entier et vous donne la meilleure, la plus rapide en cas d'égalité.",
                    "Frigo vide is an iPhone app: tell it what's left in your fridge, it searches recipes from all over the world and gives you the best one, the quickest when it's a tie.",
                    "Frigo vide es una app para iPhone: dile lo que queda en tu nevera, busca entre recetas de todo el mundo y te da la mejor, la más rápida en caso de empate.",
                )))
                .child(
                    div()
                        .class("ctas")
                        .child(
                            a().class("btn btn--tomato btn--big")
                                .href("#ecrans")
                                .text(l.t("Découvrir l'app →", "See the app →", "Descubre la app →")),
                        )
                        .child(
                            a().class("btn btn--white btn--big")
                                .href("#telecharger")
                                .text(l.t(
                                    "Bientôt sur l'App Store",
                                    "Coming soon to the App Store",
                                    "Muy pronto en el App Store",
                                )),
                        ),
                )
                .child(
                    ul().class("stats")
                        .child(
                            li().class("stat stat--sun")
                                .child(strong().text("0"))
                                .text(l.t(" clé d'API", " API key", " clave de API")),
                        )
                        .child(
                            li().class("stat stat--mint")
                                .child(strong().text("0"))
                                .text(l.t(" compte à créer", " account to create", " cuenta que crear")),
                        )
                        .child(
                            li().class("stat stat--sky")
                                .child(strong().text("100\u{a0}%"))
                                .text(l.t(" gratuit", " free", " gratis")),
                        ),
                ),
        )
        .child(
            div()
                .class("hero__art")
                .child(island("frigo-dessin", l.code(), frigo_dessin)),
        )
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
fn marquee(l: Lang) -> Element {
    let words: &[&'static str] = match l {
        Lang::Fr => &[
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
        Lang::En => &[
            "🍅 tomato",
            "🥚 eggs",
            "🧀 cheese",
            "🍗 chicken",
            "🥕 carrot",
            "🍚 rice",
            "🧅 onion",
            "🍋 lemon",
            "🍄 mushroom",
            "🥔 potato",
        ],
        Lang::Es => &[
            "🍅 tomate",
            "🥚 huevos",
            "🧀 queso",
            "🍗 pollo",
            "🥕 zanahoria",
            "🍚 arroz",
            "🧅 cebolla",
            "🍋 limón",
            "🍄 champiñón",
            "🥔 patata",
        ],
    };
    band(words, "")
}

/// Le bandeau de l'app, avant l'architecture (dans l'autre sens).
fn tech_band(l: Lang) -> Element {
    let words: &[&'static str] = match l {
        Lang::Fr => &[
            "📱 iPhone",
            "🍎 SwiftUI",
            "🫧 Liquid Glass",
            "💾 SwiftData",
            "🔎 TheMealDB",
            "📳 Haptique",
            "🌍 Recettes du monde",
            "💚 Open source",
        ],
        Lang::En => &[
            "📱 iPhone",
            "🍎 SwiftUI",
            "🫧 Liquid Glass",
            "💾 SwiftData",
            "🔎 TheMealDB",
            "📳 Haptics",
            "🌍 World recipes",
            "💚 Open source",
        ],
        Lang::Es => &[
            "📱 iPhone",
            "🍎 SwiftUI",
            "🫧 Liquid Glass",
            "💾 SwiftData",
            "🔎 TheMealDB",
            "📳 Háptica",
            "🌍 Recetas del mundo",
            "💚 Código abierto",
        ],
    };
    band(words, "tomato")
}

/// Le bandeau des plats.
fn dishes_band(l: Lang) -> Element {
    let words: &[&'static str] = match l {
        Lang::Fr => &[
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
        Lang::En => &[
            "🍝 pasta",
            "🥗 salad",
            "🍲 soup",
            "🌮 tacos",
            "🍛 curry",
            "🥘 paella",
            "🍳 omelette",
            "🥞 pancakes",
            "🍜 ramen",
            "🥧 quiche",
        ],
        Lang::Es => &[
            "🍝 pasta",
            "🥗 ensalada",
            "🍲 sopa",
            "🌮 tacos",
            "🍛 curry",
            "🥘 paella",
            "🍳 tortilla",
            "🥞 crepes",
            "🍜 ramen",
            "🥧 quiche",
        ],
    };
    band(words, "sun")
}

/// Le bandeau du pied de page.
fn footer_band(l: Lang) -> Element {
    let words: &[&'static str] = match l {
        Lang::Fr => &[
            "✨ Gratuit",
            "🔑 Sans clé d'API",
            "🙅 Sans compte",
            "🍪 Sans cookie",
            "📱 Pour iPhone",
            "🔒 Privée",
            "🌍 FR · EN · ES",
            "💚 Open source",
        ],
        Lang::En => &[
            "✨ Free",
            "🔑 No API key",
            "🙅 No account",
            "🍪 No cookies",
            "📱 For iPhone",
            "🔒 Private",
            "🌍 FR · EN · ES",
            "💚 Open source",
        ],
        Lang::Es => &[
            "✨ Gratis",
            "🔑 Sin clave de API",
            "🙅 Sin cuenta",
            "🍪 Sin cookies",
            "📱 Para iPhone",
            "🔒 Privada",
            "🌍 FR · EN · ES",
            "💚 Código abierto",
        ],
    };
    band(words, "mint")
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

fn steps(l: Lang) -> Element {
    let items = [
        (
            "🧊",
            l.t("Videz le frigo", "Empty the fridge", "Vacía la nevera"),
            l.t(
                "Dans l'app, tapez vos ingrédients, en français, en anglais ou en espagnol\u{a0}: «\u{a0}poulet\u{a0}», «\u{a0}œufs\u{a0}», «\u{a0}pommes de terre\u{a0}».",
                "In the app, type your ingredients, in English, French or Spanish: “chicken”, “eggs”, “potatoes”.",
                "En la app, escribe tus ingredientes, en español, inglés o francés: «pollo», «huevos», «patatas».",
            ),
            "sun",
        ),
        (
            "🔎",
            l.t("L'app cherche", "The app searches", "La app busca"),
            l.t(
                "Touchez «\u{a0}Trouver une recette\u{a0}»\u{a0}: chaque ingrédient est cherché dans TheMealDB, en même temps.",
                "Tap “Find a recipe”: every ingredient is looked up in TheMealDB, all at once.",
                "Toca «Buscar una receta»: cada ingrediente se busca en TheMealDB, todos a la vez.",
            ),
            "mint",
        ),
        (
            "⚖️",
            l.t("On compare", "It compares", "Compara"),
            l.t(
                "Ce que vous avez, ce qui manque\u{a0}: le sel, l'huile et l'eau sont au placard.",
                "What you have, what's missing: salt, oil and water are in the cupboard.",
                "Lo que tienes y lo que falta: la sal, el aceite y el agua están en la despensa.",
            ),
            "sky",
        ),
        (
            "⏱️",
            l.t("Le meilleur gagne", "The best one wins", "Gana la mejor"),
            l.t(
                "Le temps est lu dans les instructions («\u{a0}bake 35 minutes\u{a0}»). À égalité, la plus rapide gagne.",
                "The time is read from the instructions (“bake 35 minutes”). On a tie, the quickest wins.",
                "El tiempo se lee en las instrucciones («bake 35 minutes»). En caso de empate, gana la más rápida.",
            ),
            "peach",
        ),
    ];
    section()
        .class("section")
        .id("comment")
        .child(head(
            l.t("Comment ça marche", "How it works", "Cómo funciona"),
            l.t(
                "Quatre étapes, zéro courses",
                "Four steps, zero shopping",
                "Cuatro pasos, cero compras",
            ),
            Some(l.t(
                "Quelques secondes entre «\u{a0}qu'est-ce qu'on mange\u{a0}?\u{a0}» et le premier coup de couteau.",
                "A few seconds between “what's for dinner?” and the first chop.",
                "Unos segundos entre «¿qué comemos?» y el primer corte de cuchillo.",
            )),
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

fn ios(l: Lang) -> Element {
    let cards = [
        ("Chicken Congee", "94 min · -3", "sun"),
        ("Poulet Roti a l'Algerienne", "105 min · -3", "mint"),
        ("Leek and Rice Bake", "90 min · -3", "peach"),
    ];
    let chips: [&'static str; 4] = match l {
        Lang::Fr => ["🍗 poulet", "🍚 riz", "🧅 oignon", "🧄 ail"],
        Lang::En => ["🍗 chicken", "🍚 rice", "🧅 onion", "🧄 garlic"],
        Lang::Es => ["🍗 pollo", "🍚 arroz", "🧅 cebolla", "🧄 ajo"],
    };
    section()
        .class("section section--band section--grape split")
        .id("ios")
        .child(
            div()
                .child(head(
                    l.t("Pensée pour l'iPhone", "Made for iPhone", "Pensada para iPhone"),
                    l.t("Douce au toucher", "Soft to the touch", "Suave al tacto"),
                    Some(l.t(
                        "Écrite en SwiftUI pour iOS 26\u{a0}: le verre liquide d'iOS, des cartes qui arrivent en relief quand on fait défiler, et une petite vibration à chaque étape cochée.",
                        "Written in SwiftUI for iOS 26: iOS's Liquid Glass, cards that tilt in as you scroll, and a gentle tap of haptics for every step you tick off.",
                        "Escrita en SwiftUI para iOS 26: el cristal líquido de iOS, tarjetas que llegan en relieve al desplazarte y una vibración suave en cada paso marcado.",
                    )),
                ))
                .child(
                    ul().class("features")
                        .child(li().text(l.t(
                            "🫧 Liquid Glass, le style d'iOS 26",
                            "🫧 Liquid Glass, the iOS 26 look",
                            "🫧 Liquid Glass, el estilo de iOS 26",
                        )))
                        .child(li().text(l.t(
                            "🃏 Des cartes qui arrivent en 3D",
                            "🃏 Cards that arrive in 3D",
                            "🃏 Tarjetas que llegan en 3D",
                        )))
                        .child(li().text(l.t(
                            "📳 Des vibrations douces",
                            "📳 Gentle haptics",
                            "📳 Vibraciones suaves",
                        )))
                        .child(li().text(l.t(
                            "⚡ Une recherche rapide, tout en parallèle",
                            "⚡ Fast search, all in parallel",
                            "⚡ Búsqueda rápida, todo en paralelo",
                        ))),
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
                        .attr(
                            "aria-label",
                            l.t(
                                "L'application Frigo vide sur iPhone",
                                "The Frigo vide app on iPhone",
                                "La app Frigo vide en iPhone",
                            ),
                        )
                        .child(div().class("phone__island"))
                        .child(p().class("phone__time").text("9:41"))
                        .child(h3().class("phone__title").text(l.t(
                            "Mon frigo",
                            "My fridge",
                            "Mi nevera",
                        )))
                        .child(div().class("phone__chips").children(chips.map(|c| span().text(c))))
                        .child(div().class("phone__cta").text(l.t(
                            "Trouver une recette",
                            "Find a recipe",
                            "Buscar una receta",
                        )))
                        .child(div().class("phone__cards").children(cards.map(
                            |(name, meta, tone)| {
                                div()
                                    .class(format!("phone__card phone__card--{tone}"))
                                    .child(span().class("phone__thumb"))
                                    .child(span().class("phone__name").text(name))
                                    .child(span().class("phone__meta").text(meta))
                            },
                        ))),
                ),
        )
}

fn architecture(l: Lang) -> Element {
    let blocks = [
        (
            "📝",
            l.t("Entrée", "Input", "Entrada"),
            l.t(
                "Le frigo de l'app, gardé avec SwiftData",
                "The app's fridge, kept with SwiftData",
                "La nevera de la app, guardada con SwiftData",
            ),
            "sun",
        ),
        (
            "🧠",
            "FrigoKit",
            l.t(
                "Le français et l'espagnol traduits pour TheMealDB, pluriels et accents compris",
                "French and Spanish translated for TheMealDB, plurals and accents included",
                "El español y el francés traducidos para TheMealDB, con plurales y acentos",
            ),
            "tomato",
        ),
        (
            "🌍",
            "TheMealDB",
            l.t(
                "filter.php?i= pour chaque ingrédient, lookup.php?i= pour le détail",
                "filter.php?i= for each ingredient, lookup.php?i= for the details",
                "filter.php?i= para cada ingrediente, lookup.php?i= para el detalle",
            ),
            "sky",
        ),
        (
            "🏆",
            l.t("Classement", "Ranking", "Clasificación"),
            l.t(
                "Ingrédients manquants, puis frigo le mieux utilisé, puis temps estimé",
                "Missing ingredients, then best use of your fridge, then estimated time",
                "Ingredientes que faltan, luego la nevera mejor aprovechada, luego el tiempo",
            ),
            "mint",
        ),
        (
            "🖥️",
            l.t("Affichage", "Display", "Pantalla"),
            l.t(
                "Des cartes SwiftUI, la meilleure en premier",
                "SwiftUI cards, the best one first",
                "Tarjetas SwiftUI, la mejor primero",
            ),
            "grape",
        ),
    ];
    section()
        .class("section")
        .id("architecture")
        .child(head(
            l.t("Sous le capot", "Under the hood", "Bajo el capó"),
            l.t(
                "Du frigo à l'assiette",
                "From fridge to plate",
                "De la nevera al plato",
            ),
            None,
        ))
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

/// « Partager sur X (nouvel onglet) ».
fn share_on(l: Lang, name: &str) -> String {
    match l {
        Lang::Fr => format!("Partager sur {name} (nouvel onglet)"),
        Lang::En => format!("Share on {name} (new tab)"),
        Lang::Es => format!("Compartir en {name} (nueva pestaña)"),
    }
}

/// La carte de partage : une carte à scanner (QR code), le partage du téléphone, la copie
/// du lien et des liens directs vers les messageries et les réseaux.
fn share(url: &str, l: Lang) -> Element {
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
                        .child(span().class("share-card__tag").text(l.t(
                            "La recette avec ce qu'il reste",
                            "The recipe with what's left",
                            "La receta con lo que queda",
                        ))),
                ),
        )
        .child(
            div()
                .class("share-card__body")
                .child(div().class("share-card__qr").html(qr_svg(url, l)))
                .child(
                    div()
                        .class("share-card__text")
                        .child(p().class("share-card__claim").text(l.t(
                            "Le frigo est vide\u{a0}? Pas l'assiette.",
                            "Empty fridge? Full plate.",
                            "¿La nevera vacía? El plato no.",
                        )))
                        .child(p().class("share-card__hint").text(l.t(
                            "📱 Scannez avec l'appareil photo",
                            "📱 Scan with your camera",
                            "📱 Escanea con la cámara",
                        )))
                        .child(
                            div()
                                .class("share-card__chips")
                                .child(
                                    span()
                                        .class("pill pill--white")
                                        .text(l.t("Gratuit", "Free", "Gratis")),
                                )
                                .child(span().class("pill pill--mint").text(l.t(
                                    "Sans compte",
                                    "No account",
                                    "Sin cuenta",
                                )))
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
                        .attr("data-text", share_text(l))
                        .attr("hidden", "")
                        .child(span().attr("aria-hidden", "true").text("↗"))
                        .child(span().text(l.t("Partager…", "Share…", "Compartir…"))),
                )
                .child(
                    button()
                        .class("btn btn--white btn--big")
                        .attr("type", "button")
                        .attr("data-copy", url.to_owned())
                        .attr("hidden", "")
                        .child(span().attr("aria-hidden", "true").text("🔗"))
                        .child(span().attr("data-copy-label", "").text(l.t(
                            "Copier le lien",
                            "Copy the link",
                            "Copiar el enlace",
                        ))),
                ),
        )
        .child(
            label()
                .class("share-link")
                .child(span().class("share-link__label").text(l.t(
                    "Le lien",
                    "The link",
                    "El enlace",
                )))
                .child(
                    input()
                        .attr("type", "url")
                        .attr("readonly", "")
                        .attr("value", url.to_owned())
                        .attr(
                            "aria-label",
                            l.t(
                                "Le lien du site, à copier",
                                "The site's link, to copy",
                                "El enlace del sitio, para copiar",
                            ),
                        ),
                ),
        )
        .child(
            p().class("share-actions__title")
                .text(l.t("Envoyer par", "Send via", "Enviar por")),
        )
        .child(
            ul().class("networks")
                .children(networks(url, l).into_iter().map(|n| {
                    li().child(
                        a().class(format!("network network--{}", n.tone))
                            .href(n.href)
                            .attr("target", "_blank")
                            .attr("rel", "noopener noreferrer")
                            .attr("aria-label", share_on(l, n.name))
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
                    .href(format!("/assets/{}", og_image(l)))
                    .attr("download", "frigo-vide.png")
                    .text(l.t(
                        "Télécharger l'image de partage",
                        "Download the share image",
                        "Descargar la imagen para compartir",
                    )),
            ),
        );

    section()
        .class("section section--band section--sun share")
        .id("partager")
        .child(head(
            l.t("Partager", "Share", "Compartir"),
            l.t(
                "Faites tourner l'assiette",
                "Pass the plate around",
                "Pasa el plato",
            ),
            Some(l.t(
                "Montrez la carte à un ami ou envoyez-lui le lien\u{a0}: il découvrira Frigo vide avant même sa sortie sur l'App Store.",
                "Show the card to a friend or send them the link: they'll discover Frigo vide before it even hits the App Store.",
                "Enseña la tarjeta a un amigo o envíale el enlace: descubrirá Frigo vide antes incluso de su llegada al App Store.",
            )),
        ))
        .child(div().class("share__grid").child(card).child(actions))
}

fn footer_column(title: &str, links: Vec<(&'static str, String, bool)>) -> Element {
    nav()
        .class("footer__col")
        .attr("aria-label", title.to_owned())
        .child(h2().class("footer__title").text(title.to_owned()))
        .child(
            ul().children(links.into_iter().map(|(text, href, external)| {
                let link = a().href(href).text(text);
                let link = if external {
                    link.attr("rel", "noopener")
                } else {
                    link
                };
                li().child(link)
            })),
        )
}

fn site_footer(url: &str, l: Lang) -> Element {
    let networks = networks(url, l);
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
                .attr("aria-label", share_on(l, n.name))
                .attr("title", n.name)
                .text(n.badge)
        })
        .collect();
    let external = |text: &'static str, href: &str| (text, href.to_owned(), true);
    let inside = |text: &'static str, id: &str| (text, anchor(l, id), false);
    footer()
        .class("footer")
        .child(footer_band(l))
        .child(
            div()
                .class("footer__inner")
                .child(
                    div()
                        .class("footer__brand")
                        .child(
                            a().class("nav__brand footer__logo")
                                .href(l.home())
                                .attr(
                                    "aria-label",
                                    l.t("Frigo vide, accueil", "Frigo vide, home", "Frigo vide, inicio"),
                                )
                                .child(span().class("logo").attr("aria-hidden", "true").text("🧊"))
                                .child(span().text("Frigo vide")),
                        )
                        .child(p().class("footer__pitch").text(l.t(
                            "L'app iPhone qui trouve la meilleure recette avec ce qu'il reste dans le frigo. Gratuite, sans compte, sans publicité.",
                            "The iPhone app that finds the best recipe with what's left in your fridge. Free, no account, no ads.",
                            "La app para iPhone que encuentra la mejor receta con lo que queda en la nevera. Gratis, sin cuenta, sin anuncios.",
                        )))
                        .child(languages(l, "langs--footer"))
                        .child(div().class("footer__socials").children(quick)),
                )
                .child(footer_column(
                    l.t("Produit", "Product", "Producto"),
                    vec![
                        inside(l.t("Les écrans", "Screens", "Pantallas"), "ecrans"),
                        inside(l.t("Fonctionnalités", "Features", "Funciones"), "fonctions"),
                        inside(l.t("Confidentialité", "Privacy", "Privacidad"), "confidentialite"),
                        inside(l.t("Questions fréquentes", "FAQ", "Preguntas frecuentes"), "faq"),
                        inside(
                            l.t(
                                "Bientôt sur l'App Store",
                                "Coming to the App Store",
                                "Pronto en el App Store",
                            ),
                            "telecharger",
                        ),
                    ],
                ))
                .child(footer_column(
                    l.t("Ressources", "Resources", "Recursos"),
                    vec![
                        external(l.t("Code source", "Source code", "Código fuente"), REPO),
                        external(l.t("Documentation", "Documentation", "Documentación"), README),
                        external(
                            l.t("Signaler un problème", "Report an issue", "Informar de un problema"),
                            ISSUES,
                        ),
                        external("TheMealDB", MEALDB),
                        external("Paquets", PAQUETS),
                    ],
                ))
                .child(footer_column(
                    l.t("Partager", "Share", "Compartir"),
                    vec![
                        inside(
                            l.t("La carte à scanner", "The card to scan", "La tarjeta para escanear"),
                            "partager",
                        ),
                        (
                            l.t("Image de partage", "Share image", "Imagen para compartir"),
                            format!("/assets/{}", og_image(l)),
                            false,
                        ),
                        external(l.t("Licence MIT", "MIT License", "Licencia MIT"), LICENSE),
                    ],
                )),
        )
        .child(
            div()
                .class("footer__bottom")
                .child(p().text(format!(
                    "© {} Frigo vide · {}",
                    current_year(),
                    l.t("Licence MIT", "MIT License", "Licencia MIT")
                )))
                .child(
                    p().class("footer__trust")
                        .child(span().text(l.t("🍪 Aucun cookie", "🍪 No cookies", "🍪 Sin cookies")))
                        .child(span().text(l.t("🔒 Aucun traçage", "🔒 No tracking", "🔒 Sin rastreo")))
                        .child(span().text("🦀 Rust & Swift")),
                )
                .child(a().class("footer__top").href("#main").text(l.t(
                    "Haut de page ↑",
                    "Back to top ↑",
                    "Volver arriba ↑",
                ))),
        )
}
