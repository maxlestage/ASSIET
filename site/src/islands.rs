//! L'îlot du frigo dessiné, en haut de la page. Ses props sont le code de la langue
//! (`fr`, `en`, `es`).

use paquets::prelude::*;

use crate::i18n::Lang;

/// Ce que le frigo dessiné peut contenir : un emoji, un nom (fr, en, es), une couleur.
const SHELF: [(&str, [&str; 3], &str); 12] = [
    ("🍅", ["tomate", "tomato", "tomate"], "tomato"),
    ("🥕", ["carotte", "carrot", "zanahoria"], "orange"),
    ("🧀", ["fromage", "cheese", "queso"], "sun"),
    ("🥚", ["œufs", "eggs", "huevos"], "cream"),
    ("🍗", ["poulet", "chicken", "pollo"], "peach"),
    ("🥦", ["brocoli", "broccoli", "brócoli"], "mint"),
    ("🍋", ["citron", "lemon", "limón"], "sun"),
    ("🧅", ["oignon", "onion", "cebolla"], "grape"),
    ("🍄", ["champignon", "mushroom", "champiñón"], "peach"),
    ("🥛", ["lait", "milk", "leche"], "sky"),
    ("🫑", ["poivron", "pepper", "pimiento"], "mint"),
    ("🍚", ["riz", "rice", "arroz"], "cream"),
];

/// Le frigo du haut de la page : un dessin plat et coloré. Toucher la porte l'ouvre
/// (les ingrédients sautent sur les étagères), la toucher encore change le contenu.
pub fn frigo_dessin(props: &str) -> Node {
    let lang = Lang::from_code(props);
    let index = match lang {
        Lang::Fr => 0,
        Lang::En => 1,
        Lang::Es => 2,
    };
    let open = use_state(false);
    let round = use_state(0usize);
    let shelves = move || {
        let r = round.get();
        (0..3)
            .map(|row| {
                div()
                    .class("fridge__shelf")
                    .children((0..3).map(move |col| {
                        let (emoji, names, tone) = SHELF[(r * 5 + row * 3 + col) % SHELF.len()];
                        span()
                            .class(format!("food food--{tone}"))
                            .style(format!("--i:{}", row * 3 + col))
                            .attr("title", names[index])
                            .text(emoji)
                    }))
                    .into()
            })
            .collect::<Vec<Node>>()
    };
    div()
        .class("fridge")
        .class_if("is-open", move || open.get())
        .child(
            div()
                .class("fridge__body")
                .child(
                    div()
                        .class("fridge__freezer")
                        .child(span().class("fridge__ice").text("❄️")),
                )
                .child(
                    div()
                        .class("fridge__inside")
                        .attr("aria-live", "polite")
                        .children_dyn(move || {
                            // Une nouvelle liste à chaque tour : les ingrédients sautent.
                            let _ = round.get();
                            shelves()
                        }),
                )
                .child(
                    button()
                        .class("fridge__door")
                        .attr("type", "button")
                        .attr_dyn("aria-label", move || {
                            if open.get() {
                                lang.t(
                                    "Changer le contenu du frigo",
                                    "Change what's in the fridge",
                                    "Cambiar lo que hay en la nevera",
                                )
                                .into()
                            } else {
                                lang.t("Ouvrir le frigo", "Open the fridge", "Abrir la nevera")
                                    .into()
                            }
                        })
                        .attr_dyn("aria-pressed", move || open.get().to_string())
                        .on_click(move |_| {
                            if open.get() {
                                round.update(|r| *r += 1);
                            } else {
                                open.set(true);
                            }
                        })
                        .child(span().class("fridge__handle"))
                        .child(span().class("fridge__magnet fridge__magnet--a").text("🍕"))
                        .child(span().class("fridge__magnet fridge__magnet--b").text("⭐"))
                        .child(span().class("fridge__note").text(lang.t(
                            "Ouvre-moi !",
                            "Open me!",
                            "¡Ábreme!",
                        ))),
                )
                .child(div().class("fridge__feet").child(span()).child(span())),
        )
        .child(
            div()
                .class("fridge__caption")
                .child(span().class("fridge__count").text_dyn(move || {
                    if open.get() {
                        let n = round.get() + 1;
                        match lang {
                            Lang::Fr => format!("Tour n°\u{a0}{n} · touchez encore pour remplir"),
                            Lang::En => format!("Round {n} · tap again to refill"),
                            Lang::Es => format!("Ronda {n} · toca otra vez para llenar"),
                        }
                    } else {
                        lang.t("Touchez la porte", "Tap the door", "Toca la puerta")
                            .into()
                    }
                }))
                .child(
                    button()
                        .class("link")
                        .attr("type", "button")
                        .bool_attr("hidden", move || !open.get())
                        .on_click(move |_| open.set(false))
                        .text(lang.t("Fermer", "Close", "Cerrar")),
                ),
        )
        .into()
}
