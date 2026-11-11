//! Le site sans réseau : la page d'accueil, les îlots, les erreurs de l'API.
#![cfg(not(target_arch = "wasm32"))]

#[tokio::test]
async fn the_home_page_has_its_text_and_islands() {
    let client = frigo_site::server::app().test().await;
    let res = client.get("/").await;
    assert_eq!(res.status(), 200);
    let html = res.text();
    assert!(html.contains("<html lang=\"fr\""));
    assert!(html.contains("on mange"));
    assert!(
        html.contains("active-island"),
        "les îlots sont rendus par le serveur"
    );
    assert!(html.contains("fridge__door"), "le frigo dessiné est rendu");
    assert!(html.contains("/site.css?v="));
    for id in ["demo", "terminal", "tui", "ios", "installer"] {
        assert!(html.contains(&format!("id=\"{id}\"")), "section #{id}");
    }
}

#[tokio::test]
async fn an_empty_fridge_is_a_bad_request() {
    let client = frigo_site::server::app().test().await;
    let res = client.get("/api/recettes?i=").await;
    assert_eq!(res.status(), 400);
    assert!(res.text().contains("vide"));
}

#[tokio::test]
async fn only_themealdb_images_are_relayed() {
    let client = frigo_site::server::app().test().await;
    // Bloqué par le pare-feu de Paquets (400) ou refusé par la route (404).
    let traversal = client
        .get("/vignette/..%2F..%2Fetc%2Fpasswd")
        .await
        .status();
    assert!(traversal == 400 || traversal == 404, "{traversal}");
    assert_eq!(client.get("/vignette/a.svg").await.status(), 404);
}

#[tokio::test]
async fn unknown_pages_are_404() {
    let client = frigo_site::server::app().test().await;
    let res = client.get("/nulle-part").await;
    assert_eq!(res.status(), 404);
    assert!(res.text().contains("Ce placard est vide"));
}
