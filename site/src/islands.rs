//! Les îlots : la scène 3D du haut de la page et la démo du frigo.

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

/// L'assiette 3D : des ingrédients tombent, rebondissent et s'empilent (la physique
/// d'Active). Le serveur rend un canvas vide ; le navigateur lance la scène.
pub fn assiette(_props: &str) -> Node {
    let count = use_state(0usize);
    fragment([
        canvas()
            .class("assiette")
            .attr("role", "img")
            .attr(
                "aria-label",
                "Des ingrédients en 3D tombent dans une assiette",
            )
            .use_action(move |canvas| play(canvas, count)),
        p().class("assiette__hud")
            .attr("aria-hidden", "true")
            .child(
                span()
                    .class("chip chip--live")
                    .text("WebGL 2 · active::ecs"),
            )
            .child(
                span()
                    .class("assiette__count")
                    .text_dyn(move || match count.get() {
                        0 => "Touchez l'assiette".into(),
                        1 => "1 ingrédient".into(),
                        n => format!("{n} ingrédients"),
                    }),
            ),
    ])
}

#[cfg(target_arch = "wasm32")]
fn play(canvas: &paquets::active::Dom, count: State<usize>) -> impl FnOnce() + use<> {
    let running = crate::scene::start(canvas, count);
    move || drop(running)
}

#[cfg(not(target_arch = "wasm32"))]
fn play(_canvas: &paquets::active::Dom, _count: State<usize>) -> impl FnOnce() + use<> {
    || {}
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

/// La démo : le frigo (des étiquettes), « Ouvrir le frigo », et les recettes.
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
        .class("frigo")
        .class_if("frigo--open", move || answer.with(|a| a.is_some()))
        .class_if("frigo--loading", move || loading.get())
        .child(
            div()
                .class("frigo__door")
                .child(
                    div()
                        .class("frigo__shelf")
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
                                        .class("chip chip--item")
                                        .attr("type", "button")
                                        .attr("aria-label", format!("Retirer {label}"))
                                        .style(format!("--i:{i}"))
                                        .on_click(move |_| {
                                            let gone = item.clone();
                                            items.update(|l| l.retain(|x| *x != gone));
                                        })
                                        .text(label)
                                        .child(span().attr("aria-hidden", "true").text(" ×"))
                                        .into()
                                })
                                .collect()
                        }),
                )
                .child(
                    div()
                        .class("frigo__add")
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
                                .class("button button--ghost")
                                .attr("type", "button")
                                .on_click(move |_| {
                                    add(draft.get());
                                    draft.set(String::new());
                                })
                                .text("Ajouter"),
                        ),
                )
                .child(
                    div().class("frigo__suggest").children(SUGGESTIONS.iter().map(|s| {
                        let s = *s;
                        button()
                            .class("chip chip--suggest")
                            .attr("type", "button")
                            .class_if("is-in", move || items.with(|l| l.iter().any(|x| x == s)))
                            .on_click(move |_| add(s.to_owned()))
                            .text(format!("+ {s}"))
                    })),
                )
                .child(
                    div()
                        .class("frigo__actions")
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
                                .child(span().text("Sans courses (strict)")),
                        )
                        .child(
                            button()
                                .class("button button--primary")
                                .attr("type", "button")
                                .bool_attr("disabled", move || loading.get())
                                .on_click(move |_| search())
                                .child(span().class("button__icon").attr("aria-hidden", "true").text("🧊"))
                                .text_dyn(move || {
                                    if loading.get() { "Je fouille…".into() } else { "Ouvrir le frigo".into() }
                                }),
                        ),
                ),
        )
        .child(
            div()
                .class("frigo__results")
                .attr("aria-live", "polite")
                .children_dyn(move || match answer.get() {
                    None => vec![
                        div()
                            .class("empty")
                            .child(div().class("empty__plate").attr("aria-hidden", "true"))
                            .child(p().class("muted").text("Les recettes apparaîtront ici, de la plus rapide à la plus longue."))
                            .into(),
                    ],
                    Some(Answer { error: Some(e), .. }) => {
                        vec![p().class("alert").text(e).into()]
                    }
                    Some(Answer { cards, .. }) if cards.is_empty() => vec![
                        p().class("alert")
                            .text("Aucune recette avec ce frigo. Essayez sans le mode strict, ou ajoutez un ingrédient.")
                            .into(),
                    ],
                    Some(Answer { cards, .. }) => cards
                        .into_iter()
                        .enumerate()
                        .map(|(i, c)| card(i, c))
                        .collect(),
                }),
        )
        .into()
}

/// Une recette : une carte qui se retourne (la photo devant, les étapes derrière).
fn card(index: usize, c: Card) -> Node {
    let flipped = use_state(false);
    let best = index == 0;
    let mut front = div().class("card__face card__front");
    if let Some(src) = &c.thumb {
        front = front.child(
            img()
                .attr("src", src.clone())
                .attr("alt", "")
                .attr("loading", "lazy")
                .attr("decoding", "async")
                .attr("width", "350")
                .attr("height", "350"),
        );
    }
    let meta: Vec<String> = [c.category.clone(), c.area.clone()]
        .into_iter()
        .flatten()
        .collect();
    front = front
        .child(
            div()
                .class("card__body")
                .child(
                    div()
                        .class("card__tags")
                        .child(
                            span()
                                .class("chip chip--time")
                                .text(format!("≈ {} min", c.minutes)),
                        )
                        .child(if c.missing.is_empty() {
                            span().class("chip chip--ok").text("Rien à acheter")
                        } else {
                            span()
                                .class("chip chip--miss")
                                .text(format!("Manque {}", c.missing.len()))
                        }),
                )
                .child(h3().text(c.name.clone()))
                .child(p().class("muted").text(meta.join(" · ")))
                .child(
                    div()
                        .class("meter")
                        .attr("role", "img")
                        .attr(
                            "aria-label",
                            format!("{}\u{a0}% des ingrédients", c.coverage),
                        )
                        .child(span().style(format!("--p:{}%", c.coverage))),
                ),
        )
        .child(
            span()
                .class("card__hint")
                .attr("aria-hidden", "true")
                .text("Touchez pour retourner ↻"),
        );
    let mut back = div()
        .class("card__face card__back")
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
        back = back.child(
            a().class("button button--ghost")
                .href(video)
                .attr("target", "_blank")
                .attr("rel", "noopener")
                .text("▶ Voir la vidéo"),
        );
    }
    article()
        .class("card")
        .class_if("card--best", move || best)
        .class_if("is-flipped", move || flipped.get())
        .style(format!("--i:{index}"))
        .child(
            button()
                .class("card__flip")
                .attr("type", "button")
                .attr_dyn("aria-pressed", move || flipped.get().to_string())
                .attr("aria-label", format!("Retourner la carte {}", c.name))
                .on_click(move |_| flipped.update(|f| *f = !*f)),
        )
        .child(if best {
            span().class("card__ribbon").text("Meilleur choix")
        } else {
            span().class("card__ribbon card__ribbon--none")
        })
        .child(div().class("card__inner").child(front).child(back))
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
