//! `cargo run` : le site de Frigo vide sur http://localhost:3000 (`PORT` sinon).
//! Lancez-le depuis `site/` après avoir construit le bundle du navigateur (voir le README).

#[cfg(not(target_arch = "wasm32"))]
fn main() -> std::io::Result<()> {
    frigo_site::server::run()
}

#[cfg(target_arch = "wasm32")]
fn main() {}
