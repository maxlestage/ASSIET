//! Le bundle du navigateur : rend le frigo dessiné interactif et anime la page (apparitions
//! au défilement, barre de progression, barre de navigation), et fait marcher le partage
//! (partage du téléphone, copie du lien). Sans lui, la page reste lisible et complète, et
//! les liens de partage marchent.

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    HtmlElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};

#[wasm_bindgen(start)]
pub fn start() {
    paquets::hydrate(&crate::ISLANDS);
    let root: HtmlElement = document().document_element().unwrap().unchecked_into();
    reveals();
    // Seulement maintenant, le CSS peut cacher ce qui n'est pas encore apparu.
    let _ = root.class_list().add_1("hydrated");
    scroll_state(&root);
    sharing();
}

fn window() -> web_sys::Window {
    web_sys::window().expect("window")
}

fn document() -> web_sys::Document {
    window().document().expect("document")
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

/// `--scroll` (0 → 1 sur la page, pour la barre de progression) et `.scrolled` (la barre
/// de navigation change de fond).
fn scroll_state(root: &HtmlElement) {
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
        }
    };
    update();
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

/// Lit `obj[key]`.
fn get(obj: &JsValue, key: &str) -> JsValue {
    js_sys::Reflect::get(obj, &JsValue::from_str(key)).unwrap_or(JsValue::UNDEFINED)
}

/// Appelle la méthode `key` de `obj` avec `args`.
fn call(obj: &JsValue, key: &str, args: &[JsValue]) -> Option<JsValue> {
    let f: js_sys::Function = get(obj, key).dyn_into().ok()?;
    let args: js_sys::Array = args.iter().collect();
    f.apply(obj, &args).ok()
}

/// Attend une promesse, puis `then(réussie)` (un partage annulé n'est pas une erreur).
fn settle(promise: Option<JsValue>, then: impl FnOnce(bool) + 'static) {
    let Some(promise) = promise.and_then(|p| p.dyn_into::<js_sys::Promise>().ok()) else {
        then(false);
        return;
    };
    wasm_bindgen_futures::spawn_local(async move {
        then(JsFuture::from(promise).await.is_ok());
    });
}

fn on_click(el: &HtmlElement, f: impl Fn() + 'static) {
    let closure = Closure::<dyn Fn()>::new(f);
    let _ = el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Le bouton « Partager… » (le partage du téléphone, s'il existe) et « Copier le lien ».
fn sharing() {
    let navigator: JsValue = window().navigator().into();
    if get(&navigator, "share").is_function() {
        for button in all("[data-share]") {
            button.set_hidden(false);
            let data = js_sys::Object::new();
            for key in ["url", "title", "text"] {
                if let Some(value) = button.get_attribute(&format!("data-{key}")) {
                    let _ = js_sys::Reflect::set(&data, &key.into(), &value.into());
                }
            }
            let navigator = navigator.clone();
            on_click(&button, move || {
                settle(call(&navigator, "share", &[data.clone().into()]), |_| {});
            });
        }
    }
    for button in all("[data-copy]") {
        button.set_hidden(false);
        let url = button.get_attribute("data-copy").unwrap_or_default();
        let target = button.clone();
        let navigator = navigator.clone();
        on_click(&button, move || {
            let clipboard = get(&navigator, "clipboard");
            let target = target.clone();
            let url = url.clone();
            let copy = if clipboard.is_object() {
                call(&clipboard, "writeText", &[url.clone().into()])
            } else {
                None
            };
            settle(copy, move |copied| {
                if copied {
                    copied_feedback(&target);
                } else {
                    // Pas de presse-papiers (page non sécurisée, vieux navigateur).
                    let _ = window().prompt_with_message_and_default("Copiez ce lien\u{a0}:", &url);
                }
            });
        });
    }
}

/// « Lien copié ✓ » pendant deux secondes.
fn copied_feedback(button: &HtmlElement) {
    let Ok(Some(label)) = button.query_selector("[data-copy-label]") else {
        return;
    };
    label.set_text_content(Some("Lien copié ✓"));
    let _ = button.class_list().add_1("is-done");
    let button = button.clone();
    let reset = Closure::once_into_js(move || {
        label.set_text_content(Some("Copier le lien"));
        let _ = button.class_list().remove_1("is-done");
    });
    let _ =
        window().set_timeout_with_callback_and_timeout_and_arguments_0(reset.unchecked_ref(), 2000);
}
