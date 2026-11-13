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

#[tokio::test]
async fn the_app_is_installable_with_its_own_icons() {
    let client = frigo_site::server::app().test().await;
    let manifest = client.get("/manifest.webmanifest").await.text();
    for icon in [
        "/assets/icon-192.png",
        "/assets/icon-512.png",
        "/assets/icon-maskable-512.png",
    ] {
        assert!(manifest.contains(icon), "{icon} dans le manifeste");
        assert_eq!(client.get(icon).await.status(), 200, "{icon}");
    }
    assert!(manifest.contains(r#""purpose":"maskable""#));
    assert!(manifest.contains(r#""display":"standalone""#));
    let worker = client.get("/sw.js").await.text();
    assert!(
        worker.contains("/assets/icon-512.png"),
        "les icônes sont gardées hors ligne"
    );
    let html = client.get("/").await.text();
    assert!(html.contains(r#"<link rel="apple-touch-icon" href="/assets/apple-touch-icon.png"/>"#));
    assert!(html.contains("data-install"), "le bouton d'installation");
}

#[tokio::test]
async fn the_share_card_points_to_the_public_url() {
    let client = frigo_site::server::app().test().await;
    let html = client
        .get("/")
        .header("host", "frigo.example.com")
        .header("x-forwarded-proto", "https")
        .await
        .text();
    assert!(html.contains("id=\"partager\""));
    assert!(html.contains("<svg class=\"qr\""), "le QR code");
    assert!(html.contains("frigo.example.com"));
    assert!(html.contains("https://wa.me/?text="));
    assert!(html.contains("https%3A%2F%2Ffrigo.example.com%2F"));
    assert!(
        html.contains(
            r#"<meta property="og:image" content="https://frigo.example.com/assets/og.png"/>"#
        ),
        "l'aperçu des réseaux est une image PNG à une adresse absolue"
    );
    assert!(html.contains("footer__bottom"), "le pied de page");
    assert!(html.contains("Signaler un problème"));
}
