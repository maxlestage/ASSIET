//! Le site sans réseau : la page d'accueil, les îlots, les erreurs de l'API.
#![cfg(not(target_arch = "wasm32"))]

#[tokio::test]
async fn the_home_page_has_its_text_and_islands() {
    let client = frigo_site::server::app().test().await;
    let res = client.get("/").await;
    assert_eq!(res.status(), 200);
    let html = res.text();
    assert!(html.contains("<html lang=\"fr\""));
    assert!(html.contains("Trois écrans, un repas"));
    assert!(
        html.contains("active-island"),
        "les îlots sont rendus par le serveur"
    );
    assert!(html.contains("fridge__door"), "le frigo dessiné est rendu");
    assert!(html.contains("/site.css?v="));
    for id in [
        "ecrans",
        "fonctions",
        "ios",
        "placard",
        "semaine",
        "anti-gaspi",
        "cuisines",
        "confidentialite",
        "faq",
        "telecharger",
        "partager",
    ] {
        assert!(html.contains(&format!("id=\"{id}\"")), "section #{id}");
    }
    for gone in [
        "id=\"terminal\"",
        "id=\"tui\"",
        "cargo install",
        "id=\"demo\"",
        "/api/",
    ] {
        assert!(
            !html.contains(gone),
            "le site ne fait que montrer l'app : {gone}"
        );
    }
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

#[tokio::test]
async fn the_site_speaks_french_english_and_spanish() {
    let client = frigo_site::server::app().test().await;
    let cases = [
        ("/", "<html lang=\"fr\"", "Trois écrans, un repas", "og.png"),
        (
            "/en/",
            "<html lang=\"en\"",
            "Three screens, one meal",
            "og-en.png",
        ),
        (
            "/es/",
            "<html lang=\"es\"",
            "Tres pantallas, una comida",
            "og-es.png",
        ),
    ];
    for (path, lang, title, og) in cases {
        let res = client.get(path).await;
        assert_eq!(res.status(), 200, "{path}");
        let html = res.text();
        assert!(html.contains(lang), "{path} : la langue de la page");
        assert!(html.contains(title), "{path} : le texte traduit");
        assert!(html.contains(og), "{path} : l'image de partage traduite");
        for code in ["fr", "en", "es"] {
            assert!(
                html.contains(&format!("hreflang=\"{code}\"")),
                "{path} : le lien vers la version {code}"
            );
        }
        assert!(
            html.contains("aria-current=\"true\""),
            "{path} : la langue choisie"
        );
    }
    // Sans la barre finale, la page existe aussi.
    assert_eq!(client.get("/en").await.status(), 200);
    // Une page inconnue sous /es/ répond en espagnol.
    let missing = client.get("/es/nada").await;
    assert_eq!(missing.status(), 404);
    assert!(missing.text().contains("Esta despensa está vacía"));
}

#[tokio::test]
async fn the_pricing_section_shows_free_and_premium() {
    let client = frigo_site::server::app().test().await;
    let fr = client.get("/").await.text();
    assert!(fr.contains("id=\"tarifs\""));
    assert!(fr.contains("Prix annoncé à la sortie"), "pas de prix inventé");
    assert!(fr.contains("La liste de courses"));
    let en = client.get("/en/").await.text();
    assert!(en.contains("Price announced at launch"));
    assert!(en.contains("href=\"#tarifs\""));
}
