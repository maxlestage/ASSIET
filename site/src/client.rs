//! Le bundle du navigateur : rend les îlots interactifs et anime la page (apparitions au
//! défilement, profondeur 3D qui suit le défilement, inclinaison des cartes, compteurs).
//! Sans lui, la page reste lisible et complète.

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    HtmlElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};

use crate::islands::Answer;

#[wasm_bindgen(start)]
pub fn start() {
    paquets::hydrate(&crate::ISLANDS);
    let root: HtmlElement = document().document_element().unwrap().unchecked_into();
    reveals();
    // Seulement maintenant, le CSS peut cacher ce qui n'est pas encore apparu.
    let _ = root.class_list().add_1("hydrated");
    scroll_depth(&root);
    if matches("(hover: hover) and (pointer: fine)") && !reduced_motion() {
        tilt();
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("window")
}

fn document() -> web_sys::Document {
    window().document().expect("document")
}

fn matches(query: &str) -> bool {
    window()
        .match_media(query)
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}

pub fn reduced_motion() -> bool {
    matches("(prefers-reduced-motion: reduce)")
}

fn all(selector: &str) -> Vec<HtmlElement> {
    let Ok(list) = document().query_selector_all(selector) else {
        return Vec::new();
    };
    (0..list.length())
        .filter_map(|i| list.item(i)?.dyn_into().ok())
        .collect()
}

fn on_passive<E: JsCast + 'static>(
    target: &web_sys::EventTarget,
    event: &str,
    f: impl Fn(E) + 'static,
) {
    let closure =
        Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| f(e.unchecked_into()));
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(true);
    let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
        event,
        closure.as_ref().unchecked_ref(),
        &options,
    );
    closure.forget();
}

/// `[data-reveal]` apparaît quand il entre dans l'écran (une seule fois).
fn reveals() {
    let callback = Closure::<dyn Fn(js_sys::Array, IntersectionObserver)>::new(
        |entries: js_sys::Array, io: IntersectionObserver| {
            for entry in entries.iter() {
                let Ok(entry) = entry.dyn_into::<IntersectionObserverEntry>() else {
                    continue;
                };
                if entry.is_intersecting() {
                    let _ = entry.target().class_list().add_1("in");
                    io.unobserve(&entry.target());
                }
            }
        },
    );
    let options = IntersectionObserverInit::new();
    options.set_root_margin("0px 0px -8% 0px");
    options.set_threshold(&JsValue::from_f64(0.12));
    if let Ok(io) =
        IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
    {
        let height = window()
            .inner_height()
            .ok()
            .and_then(|h| h.as_f64())
            .unwrap_or(800.0);
        for el in all("[data-reveal]") {
            // Déjà à l'écran au chargement : visible tout de suite, sans clignoter.
            if el.get_bounding_client_rect().top() < height {
                let _ = el.class_list().add_1("in");
            } else {
                io.observe(&el);
            }
        }
    }
    callback.forget();
}

/// `--scroll` (0 → 1 sur la page) et `--view` (0 → 1 pour chaque `[data-depth]` qui
/// traverse l'écran) : le CSS en tire la profondeur 3D (téléphone, terminal, étapes).
fn scroll_depth(root: &HtmlElement) {
    let root = root.clone();
    let pending = Rc::new(Cell::new(false));
    let update = {
        let root = root.clone();
        let pending = pending.clone();
        move || {
            pending.set(false);
            let win = window();
            let y = win.scroll_y().unwrap_or(0.0);
            let height = win
                .inner_height()
                .ok()
                .and_then(|h| h.as_f64())
                .unwrap_or(800.0);
            let total = (root.scroll_height() as f64 - height).max(1.0);
            let _ = root
                .style()
                .set_property("--scroll", &format!("{:.4}", (y / total).clamp(0.0, 1.0)));
            let _ = root.class_list().toggle_with_force("scrolled", y > 24.0);
            for el in all("[data-depth]") {
                let rect = el.get_bounding_client_rect();
                let progress = ((height - rect.top()) / (height + rect.height())).clamp(0.0, 1.0);
                let _ = el.style().set_property("--view", &format!("{progress:.4}"));
            }
        }
    };
    update();
    if reduced_motion() {
        return;
    }
    let update = Rc::new(update);
    let schedule = move |_: web_sys::Event| {
        if pending.replace(true) {
            return;
        }
        let update = update.clone();
        let frame = Closure::once_into_js(move || update());
        let _ = window().request_animation_frame(frame.unchecked_ref());
    };
    let schedule = Rc::new(schedule);
    let s = schedule.clone();
    on_passive::<web_sys::Event>(&window(), "scroll", move |e| s(e));
    on_passive::<web_sys::Event>(&window(), "resize", move |e| schedule(e));
}

/// Les `[data-tilt]` s'inclinent vers le pointeur (souris seulement).
fn tilt() {
    for el in all("[data-tilt]") {
        let target = el.clone();
        on_passive::<web_sys::PointerEvent>(&el, "pointermove", move |e| {
            let rect = target.get_bounding_client_rect();
            let x = (f64::from(e.client_x()) - rect.left()) / rect.width() - 0.5;
            let y = (f64::from(e.client_y()) - rect.top()) / rect.height() - 0.5;
            let style = target.style();
            let _ = style.set_property("--rx", &format!("{:.2}deg", -y * 12.0));
            let _ = style.set_property("--ry", &format!("{:.2}deg", x * 14.0));
            let _ = style.set_property("--mx", &format!("{:.1}%", (x + 0.5) * 100.0));
            let _ = style.set_property("--my", &format!("{:.1}%", (y + 0.5) * 100.0));
        });
        let target = el.clone();
        on_passive::<web_sys::PointerEvent>(&el, "pointerleave", move |_| {
            let style = target.style();
            let _ = style.set_property("--rx", "0deg");
            let _ = style.set_property("--ry", "0deg");
        });
    }
}

/// `GET /api/recettes?i=…&strict=…`, puis `done(réponse)`.
pub fn fetch_answer(items: Vec<String>, strict: bool, done: impl FnOnce(Answer) + 'static) {
    let query: String = items
        .iter()
        .map(|i| String::from(js_sys::encode_uri_component(i)))
        .collect::<Vec<_>>()
        .join(",");
    let url = format!("/api/recettes?i={query}&strict={}", u8::from(strict));
    wasm_bindgen_futures::spawn_local(async move {
        let answer = load(&url).await.unwrap_or_else(|e| Answer {
            fridge: items,
            cards: Vec::new(),
            error: Some(e),
        });
        done(answer);
    });
}

async fn load(url: &str) -> Result<Answer, String> {
    let offline = || "Pas de réseau : la démo a besoin de TheMealDB.".to_string();
    let response: web_sys::Response = JsFuture::from(window().fetch_with_str(url))
        .await
        .map_err(|_| offline())?
        .unchecked_into();
    let text = JsFuture::from(response.text().map_err(|_| offline())?)
        .await
        .map_err(|_| offline())?
        .as_string()
        .unwrap_or_default();
    if !response.ok() && text.is_empty() {
        return Err(format!("Le serveur a répondu {}.", response.status()));
    }
    paquets::serde_json::from_str(&text)
        .map_err(|_| format!("Réponse illisible ({}).", response.status()))
}
