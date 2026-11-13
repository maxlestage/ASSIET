//! L'îlot du frigo dessiné, en haut de la page.

use paquets::prelude::*;

/// Ce que le frigo dessiné peut contenir : un emoji, un nom, une couleur de fond.
const SHELF: [(&str, &str, &str); 12] = [
    ("🍅", "tomate", "tomato"),
    ("🥕", "carotte", "orange"),
    ("🧀", "fromage", "sun"),
    ("🥚", "œufs", "cream"),
    ("🍗", "poulet", "peach"),
    ("🥦", "brocoli", "mint"),
    ("🍋", "citron", "sun"),
    ("🧅", "oignon", "grape"),
    ("🍄", "champignon", "peach"),
    ("🥛", "lait", "sky"),
    ("🫑", "poivron", "mint"),
    ("🍚", "riz", "cream"),
];

/// Le frigo du haut de la page : un dessin plat et coloré. Toucher la porte l'ouvre
/// (les ingrédients sautent sur les étagères), la toucher encore change le contenu.
pub fn frigo_dessin(_props: &str) -> Node {
    let open = use_state(false);
    let round = use_state(0usize);
    let shelves = move || {
        let r = round.get();
        (0..3)
            .map(|row| {
                div()
                    .class("fridge__shelf")
                    .children((0..3).map(move |col| {
                        let (emoji, name, tone) = SHELF[(r * 5 + row * 3 + col) % SHELF.len()];
                        span()
                            .class(format!("food food--{tone}"))
                            .style(format!("--i:{}", row * 3 + col))
                            .attr("title", name)
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
                                "Changer le contenu du frigo".into()
                            } else {
                                "Ouvrir le frigo".into()
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
                        .child(span().class("fridge__note").text("Ouvre-moi !")),
                )
                .child(div().class("fridge__feet").child(span()).child(span())),
        )
        .child(
            div()
                .class("fridge__caption")
                .child(span().class("fridge__count").text_dyn(move || {
                    if open.get() {
                        format!(
                            "Tour n°\u{a0}{} · touchez encore pour remplir",
                            round.get() + 1
                        )
                    } else {
                        "Touchez la porte".into()
                    }
                }))
                .child(
                    button()
                        .class("link")
                        .attr("type", "button")
                        .bool_attr("hidden", move || !open.get())
                        .on_click(move |_| open.set(false))
                        .text("Fermer"),
                ),
        )
        .into()
}
