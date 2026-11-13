//! Le site de **Frigo vide**, écrit avec Paquets.
//!
//! La bibliothèque contient ce que le serveur et le navigateur partagent : les îlots
//! (rendus en HTML par le serveur, rendus interactifs par le bundle WebAssembly). Le serveur (`server`) n'existe qu'en natif, le mouvement de la page
//! (`client`) que dans le navigateur.

pub mod islands;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

#[cfg(target_arch = "wasm32")]
mod client;

/// Un îlot : une fonction qui reçoit ses props et rend une vue.
pub type Island = fn(&str) -> paquets::active::Node;

/// Les îlots de la page, par nom : le même tableau sert au serveur et au navigateur.
pub const ISLANDS: [(&str, Island); 1] = [("frigo-dessin", islands::frigo_dessin)];
