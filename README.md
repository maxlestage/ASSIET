# 🧊 Frigo vide

**Le frigo est vide ? Pas l'assiette.** Frigo vide lit la liste des ingrédients qu'il vous
reste, interroge [TheMealDB](https://www.themealdb.com/) (gratuit, sans clé), garde les
recettes que vous pouvez faire avec, et vous donne la meilleure : celle où il manque le
moins de choses, qui utilise le plus votre frigo, et la plus rapide à préparer.

| Partie | Dossier | Technologies |
|---|---|---|
| Le cœur | `crates/frigo-core` | Rust 2024, tokio, reqwest 0.13, serde |
| La CLI et la TUI | `crates/frigo` | clap 4.6, ratatui 0.30, crossterm 0.29, color-eyre |
| Le site de présentation | `site/` | [Paquets](https://github.com/maxlestage/Paquets), îlots WebAssembly, design 2D plat et coloré |
| L'app iPhone | `ios/` | Swift 6.3 (Xcode 26), SwiftUI (iOS 26), SwiftData, Swift Testing, XcodeGen |

## La CLI

```sh
cargo install --git https://github.com/maxlestage/ASSIET frigo --locked
```

```sh
frigo poulet riz oignon               # la meilleure recette, en couleurs
frigo "pomme de terre" œufs -t 30     # au plus 30 minutes
frigo -f frigo.txt --strict           # rien à acheter (le fichier : un ingrédient par ligne)
echo "lait, oeufs, farine" | frigo --json | jq '.[0].meal.name'
frigo --tui                           # l'interface plein écran (aussi sans arguments)
```

| Option | Rôle |
|---|---|
| `-f, --fichier <FICHIER>` | une liste d'ingrédients (`-` : l'entrée standard) ; sans ingrédients, `frigo.txt` est lu s'il existe |
| `-s, --strict` | seulement les recettes faisables sans rien acheter |
| `--sans-placard` | le sel, le poivre, l'huile, l'eau et le sucre ne sont plus considérés comme acquis |
| `-m, --manque <N>` | au plus N ingrédients manquants |
| `-t, --temps <MINUTES>` | au plus N minutes (estimées) |
| `-c, --categorie`, `-a, --cuisine` | `Chicken`, `Vegetarian`…, `French`, `Italian`… |
| `-n, --nombre <N>` | le nombre de recettes affichées (5) |
| `--json`, `--sans-couleur` | pour les scripts (`NO_COLOR` est respecté) |
| `--api <URL>` | une autre adresse de l'API (`FRIGO_API`) |

Les ingrédients s'écrivent en français ou en anglais : « poulet », « œufs », « pommes de
terre » deviennent `chicken`, `egg`, `potato`. Un ingrédient de plusieurs mots s'écrit entre
guillemets ou avec des virgules.

### La TUI

`frigo --tui` : le frigo en haut (tapez, `Entrée` pour chercher), les recettes à gauche, la
recette choisie à droite, empilées sur un terminal étroit. `Tab` change de zone, `↑ ↓`
choisit, `s` bascule le mode strict, `p` le placard, `r` relance, `q` quitte.

## Comment ça marche

```
liste d'ingrédients ──► normalisation (français → anglais, pluriels, accents)
                         │
                         ▼
        filter.php?i=<ingrédient>  ×N, en parallèle (tokio + reqwest)
                         │  les recettes trouvées le plus souvent
                         ▼
        lookup.php?i=<id>  ×30 au plus, en parallèle
                         │  JSON → structs (serde), strIngredient1…20 lus dynamiquement
                         ▼
        classement : ingrédients manquants ▸ frigo le mieux utilisé ▸ temps estimé
                         │
                         ▼
        console · TUI · JSON · cartes du site · app iOS
```

- TheMealDB ne cherche que par **un** ingrédient à la fois : chaque ingrédient du frigo est
  cherché, puis les recettes trouvées par le plus d'ingrédients sont lues en détail.
- Deux ingrédients sont « les mêmes » quand les mots de l'un sont tous dans l'autre :
  `chicken` couvre `chicken breasts`, `ail` couvre `minced garlic`, mais `egg` ne couvre pas
  `eggplant`.
- TheMealDB ne donne pas de temps de préparation : il est lu dans les instructions
  (« bake 35 minutes », « simmer 1 1/2 hours », « cook 15-20 minutes »), plus une minute et
  demie par ingrédient.

## Le site

Une application [Paquets](https://github.com/maxlestage/Paquets) : la page est rendue par le
serveur (lisible sans JavaScript), les parties interactives sont des îlots Rust compilés en
WebAssembly. Un style plat en 2D, très coloré (bordures épaisses, ombres décalées) : un frigo
dessiné qui s'ouvre et se remplit quand on le touche, un bandeau d'ingrédients qui défile,
une démo en direct qui appelle `/api/recettes` (le même `frigo-core`), un terminal animé, la
TUI et l'iPhone dessinés à plat. Conçu pour le mobile d'abord, sans aucun défilement
horizontal (vérifié à 320, 375 et 1440 px). Le bundle WebAssembly pèse environ 85 ko
compressé.

```sh
cd site
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked
cargo build --lib --target wasm32-unknown-unknown --profile wasm-release
wasm-bindgen --target web --no-typescript --remove-name-section --remove-producers-section \
  --out-dir pkg target/wasm32-unknown-unknown/wasm-release/frigo_site.wasm
cargo run --release                    # http://localhost:3000 (PORT sinon)
```

### Sur GitHub Pages

Le workflow `.github/workflows/pages.yml` publie le site à chaque push sur `master`, sur
<https://maxlestage.github.io/ASSIET/>. Il construit le bundle WebAssembly, puis
`cargo run --release -- export dist` écrit les pages en fichiers statiques (`BASE_PATH`
préfixe les adresses). Sans serveur, la démo interroge TheMealDB directement depuis le
navigateur et classe les recettes avec `frigo-core` compilé en WebAssembly.

À faire une seule fois dans le dépôt : **Settings → Pages → Source : GitHub Actions**.

```sh
cd site
BASE_PATH=/ASSIET cargo run --release -- export dist   # pour essayer en local
```

Routes : `/` (la page), `/api/recettes?i=poulet,riz&strict=1` (JSON, gardé 15 minutes),
`/vignette/{fichier}` (les photos de TheMealDB relayées : la page n'accepte que ses propres
images), `/manifest.webmanifest` (installable).

## L'app iPhone

`ios/` contient **FrigoKit**, le même algorithme en Swift (sans dépendance, compilable aussi
sur macOS et Linux), et l'app SwiftUI : le frigo gardé avec SwiftData, la recherche en
parallèle avec un `TaskGroup` dans un `actor`, des cartes qui arrivent en 3D, Liquid Glass,
retours haptiques.

```sh
cd ios
swift test                              # FrigoKit, avec Swift Testing
brew install xcodegen && xcodegen       # crée FrigoVide.xcodeproj
open FrigoVide.xcodeproj                # Xcode avec le SDK iOS 26 ou plus récent
```

## Développer

```sh
cargo test && cargo clippy --all-targets -- -D warnings   # le cœur, la CLI et la TUI
(cd site && cargo test)                                    # le site
(cd ios && swift test)                                     # FrigoKit
```

La CI (`.github/workflows/ci.yml`) fait tout cela sur Linux et macOS, et compile l'app iOS.

## Licence

MIT. Les recettes et les photos viennent de [TheMealDB](https://www.themealdb.com/).
