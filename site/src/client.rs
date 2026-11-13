//! Le bundle du navigateur : rend les îlots interactifs et anime la page (apparitions au
//! défilement, barre de progression, barre de navigation), et fait marcher le partage
//! (partage du téléphone, copie du lien) et l'installation de l'app. Sans lui, la page
//! reste lisible et complète, et les liens de partage marchent.

use std::cell::{Cell, RefCell};
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
    scroll_state(&root);
    sharing();
    installing(&root);
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

/// Montre ou cache les boutons « Installer l'app ».
fn show_install(visible: bool) {
    for button in all("[data-install]") {
        button.set_hidden(!visible);
    }
}

/// L'installation de l'app : le bouton « Installer l'app » apparaît quand le navigateur
/// la propose (`beforeinstallprompt`), l'astuce pour Safari sur iPhone et iPad, et rien
/// quand l'app est déjà installée (ouverte en plein écran).
fn installing(root: &HtmlElement) {
    let win = window();
    let nav = win.navigator();
    let navigator: JsValue = nav.clone().into();
    let standalone = win
        .match_media("(display-mode: standalone)")
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
        || get(&navigator, "standalone").as_bool() == Some(true);
    if standalone {
        let _ = root.class_list().add_1("standalone");
        return;
    }
    let agent = nav.user_agent().unwrap_or_default();
    let touch_mac = agent.contains("Macintosh") && nav.max_touch_points() > 1;
    let ios = ["iPhone", "iPad", "iPod"].iter().any(|d| agent.contains(d)) || touch_mac;
    if ios {
        for hint in all("[data-install-ios]") {
            hint.set_hidden(false);
        }
    }

    let deferred: Rc<RefCell<Option<JsValue>>> = Rc::new(RefCell::new(None));
    let offered = {
        let deferred = deferred.clone();
        Closure::<dyn Fn(web_sys::Event)>::new(move |event: web_sys::Event| {
            // Le navigateur garde sa proposition : c'est notre bouton qui la montrera.
            event.prevent_default();
            deferred.replace(Some(event.into()));
            show_install(true);
        })
    };
    let _ = win
        .add_event_listener_with_callback("beforeinstallprompt", offered.as_ref().unchecked_ref());
    offered.forget();
    let installed = Closure::<dyn Fn()>::new(|| {
        show_install(false);
        for hint in all("[data-install-ios]") {
            hint.set_hidden(true);
        }
    });
    let _ =
        win.add_event_listener_with_callback("appinstalled", installed.as_ref().unchecked_ref());
    installed.forget();

    for button in all("[data-install]") {
        let deferred = deferred.clone();
        on_click(&button, move || {
            let Some(event) = deferred.borrow_mut().take() else {
                return;
            };
            let _ = call(&event, "prompt", &[]);
            // La proposition ne sert qu'une fois, acceptée ou non.
            settle(Some(get(&event, "userChoice")), |_| show_install(false));
        });
    }
}

/// `GET /api/recettes?i=…&strict=…`, puis `done(réponse)`.
pub fn fetch_answer(items: Vec<String>, strict: bool, done: impl FnOnce(Answer) + 'static) {
    wasm_bindgen_futures::spawn_local(async move {
        let result = if is_static() {
            search_mealdb(&items, strict).await
        } else {
            let query: String = items
                .iter()
                .map(|i| String::from(js_sys::encode_uri_component(i)))
                .collect::<Vec<_>>()
                .join(",");
            load(&format!(
                "/api/recettes?i={query}&strict={}",
                u8::from(strict)
            ))
            .await
        };
        let answer = result.unwrap_or_else(|e| Answer {
            fridge: items,
            cards: Vec::new(),
            error: Some(e),
        });
        done(answer);
    });
}

/// La page vient du site statique (GitHub Pages) : pas de serveur, pas d'`/api`.
fn is_static() -> bool {
    document()
        .query_selector("[data-mode='static']")
        .ok()
        .flatten()
        .is_some()
}

const MEALDB: &str = "https://www.themealdb.com/api/json/v1/1";

/// Le texte d'une adresse.
async fn get_text(url: &str) -> Result<String, String> {
    let offline = || "Pas de réseau\u{a0}: la démo a besoin de TheMealDB.".to_string();
    let response: web_sys::Response = JsFuture::from(window().fetch_with_str(url))
        .await
        .map_err(|_| offline())?
        .unchecked_into();
    if !response.ok() {
        return Err(format!("TheMealDB a répondu {}.", response.status()));
    }
    JsFuture::from(response.text().map_err(|_| offline())?)
        .await
        .map_err(|_| offline())
        .map(|t| t.as_string().unwrap_or_default())
}

/// La recherche de `frigo-core` (comme `MealDb::suggest`), faite par le navigateur :
/// TheMealDB autorise les appels de toutes les origines.
async fn search_mealdb(fridge: &[String], strict: bool) -> Result<Answer, String> {
    use std::collections::HashMap;

    let unreadable = |_| "Réponse illisible de TheMealDB.".to_string();
    // 1. Chaque ingrédient, en parallèle.
    let lists = futures::future::join_all(fridge.iter().map(|item| {
        let url = format!(
            "{MEALDB}/filter.php?i={}",
            String::from(js_sys::encode_uri_component(&frigo_core::api_name(item)))
        );
        async move { get_text(&url).await }
    }))
    .await;
    let mut hits: HashMap<String, (usize, usize)> = HashMap::new();
    let mut order = 0;
    for list in lists {
        for summary in frigo_core::parse_summaries(&list?).map_err(unreadable)? {
            order += 1;
            hits.entry(summary.id).or_insert((0, order)).0 += 1;
        }
    }
    let mut ids: Vec<(String, (usize, usize))> = hits.into_iter().collect();
    ids.sort_by(|(_, (a, oa)), (_, (b, ob))| b.cmp(a).then(oa.cmp(ob)));
    ids.truncate(24);

    // 2. Le détail des meilleures, en parallèle.
    let details = futures::future::join_all(
        ids.iter()
            .map(|(id, _)| format!("{MEALDB}/lookup.php?i={id}"))
            .map(|url| async move { get_text(&url).await }),
    )
    .await;
    let mut meals = Vec::new();
    for detail in details {
        meals.extend(frigo_core::parse_meals(&detail?).map_err(unreadable)?);
    }

    // 3. Le classement, avec le même cœur que la CLI.
    let criteria = frigo_core::Criteria {
        strict,
        ..frigo_core::Criteria::default()
    };
    let cards = frigo_core::rank(&meals, fridge, &criteria)
        .iter()
        .take(crate::recipes::CARDS)
        .map(|m| {
            let thumb = m.meal.thumbnail.as_ref().map(|t| format!("{t}/small"));
            crate::recipes::card(m, thumb)
        })
        .collect();
    Ok(Answer {
        fridge: fridge.to_vec(),
        cards,
        error: None,
    })
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
