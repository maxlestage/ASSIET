//! Les îlots : le frigo dessiné du haut de la page, et la démo.

use paquets::prelude::*;

/// Une recette telle que l'API du site l'envoie à la démo.
#[data]
#[derive(Default)]
pub struct Card {
    pub id: String,
    pub name: String,
    /// La vignette, servie par le site (`/vignette/…`) : la page n'accepte que ses
    /// propres images.
    pub thumb: Option<String>,
    pub minutes: u32,
    pub category: Option<String>,
    pub area: Option<String>,
    pub have: Vec<String>,
    pub missing: Vec<String>,
    pub coverage: u32,
    pub steps: Vec<String>,
    pub youtube: Option<String>,
}

/// La réponse de `/api/recettes`.
#[data]
#[derive(Default)]
pub struct Answer {
    pub fridge: Vec<String>,
    pub cards: Vec<Card>,
    pub error: Option<String>,
}

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

/// Les ingrédients proposés d'un toucher.
pub const SUGGESTIONS: [&str; 12] = [
    "poulet",
    "riz",
    "oignon",
    "ail",
    "tomate",
    "œufs",
    "pâtes",
    "fromage",
    "carotte",
    "pomme de terre",
    "lait",
    "citron",
];

/// La démo : le frigo (des étiquettes), « Trouver une recette », et les recettes.
/// Les props sont la liste de départ, séparée par des virgules.
pub fn frigo(props: &str) -> Node {
    let start: Vec<String> = props
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    let items = use_state(start);
    let draft = use_state(String::new());
    let strict = use_state(false);
    let loading = use_state(false);
    let answer = use_state(None::<Answer>);

    let add = move |text: String| {
        let text = text.trim().to_lowercase();
        if !text.is_empty() {
            items.update(|list| {
                if !list.contains(&text) {
                    list.push(text);
                }
            });
        }
    };
    let search = move || {
        if loading.get() {
            return;
        }
        let list = items.get();
        if list.is_empty() {
            answer.set(Some(Answer {
                error: Some("Le frigo est vide\u{a0}: ajoutez au moins un ingrédient.".into()),
                ..Answer::default()
            }));
            return;
        }
        loading.set(true);
        fetch(list, strict.get(), move |result| {
            loading.set(false);
            answer.set(Some(result));
        });
    };

    div()
        .class("demo")
        .class_if("demo--loading", move || loading.get())
        .child(
            div()
                .class("panel panel--sun demo__fridge")
                .child(h3().class("panel__title").text("🧊 Mon frigo"))
                .child(
                    div()
                        .class("demo__items")
                        .attr("aria-live", "polite")
                        .children_dyn(move || {
                            let list = items.get();
                            if list.is_empty() {
                                return vec![span().class("muted").text("Le frigo est vide…").into()];
                            }
                            list.into_iter()
                                .enumerate()
                                .map(|(i, item)| {
                                    let label = item.clone();
                                    button()
                                        .class(format!("tag tag--{}", ["tomato", "mint", "sky", "grape", "peach"][i % 5]))
                                        .attr("type", "button")
                                        .attr("aria-label", format!("Retirer {label}"))
                                        .style(format!("--i:{i}"))
                                        .on_click(move |_| {
                                            let gone = item.clone();
                                            items.update(|l| l.retain(|x| *x != gone));
                                        })
                                        .text(label)
                                        .child(span().attr("aria-hidden", "true").text("\u{a0}✕"))
                                        .into()
                                })
                                .collect()
                        }),
                )
                .child(
                    div()
                        .class("demo__add")
                        .child(
                            input()
                                .attr("type", "text")
                                .attr("name", "ingredient")
                                .attr("placeholder", "Ajouter un ingrédient…")
                                .attr("aria-label", "Ajouter un ingrédient")
                                .attr("autocomplete", "off")
                                .attr("enterkeyhint", "done")
                                .attr_dyn("value", move || draft.get())
                                .on_input(move |e| draft.set(e.value()))
                                .on("keydown", move |e| {
                                    if e.key() == "Enter" {
                                        e.prevent_default();
                                        add(draft.get());
                                        draft.set(String::new());
                                    }
                                }),
                        )
                        .child(
                            button()
                                .class("btn btn--ink")
                                .attr("type", "button")
                                .on_click(move |_| {
                                    add(draft.get());
                                    draft.set(String::new());
                                })
                                .text("Ajouter"),
                        ),
                )
                .child(div().class("demo__suggest").children(SUGGESTIONS.iter().map(|s| {
                    let s = *s;
                    button()
                        .class("tag tag--ghost")
                        .attr("type", "button")
                        .class_if("is-in", move || items.with(|l| l.iter().any(|x| x == s)))
                        .on_click(move |_| add(s.to_owned()))
                        .text(format!("+ {s}"))
                })))
                .child(
                    div()
                        .class("demo__actions")
                        .child(
                            label()
                                .class("switch")
                                .child(
                                    input()
                                        .attr("type", "checkbox")
                                        .bool_attr("checked", move || strict.get())
                                        .on("change", move |_| strict.update(|s| *s = !*s)),
                                )
                                .child(span().class("switch__track").attr("aria-hidden", "true"))
                                .child(span().text("Sans courses")),
                        )
                        .child(
                            button()
                                .class("btn btn--tomato btn--big")
                                .attr("type", "button")
                                .bool_attr("disabled", move || loading.get())
                                .on_click(move |_| search())
                                .child(span().class("btn__icon").attr("aria-hidden", "true").text("🍳"))
                                .text_dyn(move || {
                                    if loading.get() { "Je cherche…".into() } else { "Trouver une recette".into() }
                                }),
                        ),
                ),
        )
        .child(
            div()
                .class("demo__results")
                .attr("aria-live", "polite")
                .children_dyn(move || match answer.get() {
                    None => vec![
                        div()
                            .class("empty")
                            .child(div().class("empty__plate").attr("aria-hidden", "true").child(span().text("🍽️")))
                            .child(p().text("Les recettes arrivent ici, de la meilleure à la moins bonne."))
                            .into(),
                    ],
                    Some(Answer { error: Some(e), .. }) => vec![p().class("alert").text(e).into()],
                    Some(Answer { cards, .. }) if cards.is_empty() => vec![
                        p().class("alert")
                            .text("Aucune recette avec ce frigo. Essayez sans « Sans courses », ou ajoutez un ingrédient.")
                            .into(),
                    ],
                    Some(Answer { cards, .. }) => cards.into_iter().enumerate().map(|(i, c)| card(i, c)).collect(),
                }),
        )
        .into()
}

/// Les couleurs des cartes, à tour de rôle.
const TONES: [&str; 5] = ["sun", "mint", "sky", "peach", "grape"];

/// Une recette : la photo, le temps, ce qui manque ; « Voir la recette » déplie le détail.
fn card(index: usize, c: Card) -> Node {
    let open = use_state(false);
    let tone = TONES[index % TONES.len()];
    let meta: Vec<String> = [c.category.clone(), c.area.clone()]
        .into_iter()
        .flatten()
        .collect();
    let mut media = div().class("card__media");
    if let Some(src) = &c.thumb {
        media = media.child(
            img()
                .attr("src", src.clone())
                .attr("alt", "")
                .attr("loading", "lazy")
                .attr("decoding", "async")
                .attr("width", "350")
                .attr("height", "350"),
        );
    }
    if index == 0 {
        media = media.child(span().class("card__badge").text("⭐ Meilleur choix"));
    }
    let mut details = div()
        .class("card__details")
        .child(h4().text("Il vous faut"))
        .child(
            ul().class("checks").children(
                c.have
                    .iter()
                    .map(|h| li().class("ok").text(h.clone()))
                    .chain(c.missing.iter().map(|m| li().class("miss").text(m.clone()))),
            ),
        )
        .child(h4().text("En bref"))
        .child(
            ol().class("steps")
                .children(c.steps.iter().take(3).map(|s| li().text(s.clone()))),
        );
    if let Some(video) = c.youtube.clone() {
        details = details.child(
            a().class("btn btn--ink btn--small")
                .href(video)
                .attr("target", "_blank")
                .attr("rel", "noopener")
                .text("▶ La vidéo"),
        );
    }
    article()
        .class(format!("card card--{tone}"))
        .class_if("is-open", move || open.get())
        .style(format!("--i:{index}"))
        .child(media)
        .child(
            div()
                .class("card__body")
                .child(
                    div()
                        .class("card__tags")
                        .child(
                            span()
                                .class("pill pill--white")
                                .text(format!("⏱ ≈ {} min", c.minutes)),
                        )
                        .child(if c.missing.is_empty() {
                            span().class("pill pill--mint").text("✓ Rien à acheter")
                        } else {
                            span()
                                .class("pill pill--tomato")
                                .text(format!("🛒 Manque {}", c.missing.len()))
                        }),
                )
                .child(h3().text(c.name.clone()))
                .child(p().class("card__meta").text(meta.join(" · ")))
                .child(
                    div()
                        .class("meter")
                        .attr("role", "img")
                        .attr(
                            "aria-label",
                            format!("{}\u{a0}% des ingrédients", c.coverage),
                        )
                        .child(span().style(format!("--p:{}%", c.coverage))),
                )
                .child(
                    button()
                        .class("card__toggle")
                        .attr("type", "button")
                        .attr_dyn("aria-expanded", move || open.get().to_string())
                        .on_click(move |_| open.update(|o| *o = !*o))
                        .text_dyn(move || {
                            if open.get() {
                                "Masquer ▲".into()
                            } else {
                                "Voir la recette ▼".into()
                            }
                        }),
                )
                .child(div().class("card__fold").child(details)),
        )
        .into()
}

/// Appelle `/api/recettes` et donne la réponse à `done`.
#[cfg(target_arch = "wasm32")]
fn fetch(items: Vec<String>, strict: bool, done: impl FnOnce(Answer) + 'static) {
    crate::client::fetch_answer(items, strict, done);
}

/// Le serveur ne cherche rien : la démo n'existe que dans le navigateur.
#[cfg(not(target_arch = "wasm32"))]
fn fetch(_items: Vec<String>, _strict: bool, _done: impl FnOnce(Answer) + 'static) {}
