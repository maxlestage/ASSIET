#!/usr/bin/env bash
# Construit le serveur du site et ses îlots (WebAssembly), comme le Dockerfile.
# Lancé sur Heroku par RustConfig une fois que le buildpack emk/rust a installé Rust ;
# marche aussi à la main depuis le dépôt : ./heroku-build.sh
# Le Procfile lance site/target/release/frigo-site.
set -euo pipefail

# Si une dépendance git devenait privée : une variable de config REPOS_TOKEN (un jeton
# GitHub en lecture) est donnée à git par un assistant d'identification, jamais dans une URL.
if [ -z "${REPOS_TOKEN:-}" ] && [ -n "${ENV_DIR:-}" ] && [ -f "$ENV_DIR/REPOS_TOKEN" ]; then
  REPOS_TOKEN=$(<"$ENV_DIR/REPOS_TOKEN")
fi
if [ -n "${REPOS_TOKEN:-}" ]; then
  export REPOS_TOKEN CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1 \
    GIT_CONFIG_KEY_0=credential.https://github.com.helper \
    GIT_CONFIG_VALUE_0='!f() { test "$1" = get && echo username=x-access-token && echo "password=$REPOS_TOKEN"; }; f'
fi
cd "$(dirname "$0")/site"
# Sur Heroku, Cargo compile dans le cache du buildpack (CARGO_TARGET_DIR).
target="${CARGO_TARGET_DIR:-target}"

echo "-----> Le serveur du site"
cargo build --release --bin frigo-site
mkdir -p target/release
if [ "$target" != target ]; then
  cp "$target/release/frigo-site" target/release/
fi
echo "       target/release/frigo-site prêt"

echo "-----> Les îlots (WebAssembly)"
rustup target add wasm32-unknown-unknown
cargo build --lib --target wasm32-unknown-unknown --profile wasm-release

# wasm-bindgen doit être à la version de site/Cargo.lock : un binaire publié, ou compilé
# s'il ne peut pas être téléchargé.
version=$(grep -A1 '^name = "wasm-bindgen"$' Cargo.lock | sed -n 's/^version = "\(.*\)"$/\1/p')
bin="$target/wasm-bindgen-$version"
if [ ! -x "$bin/wasm-bindgen" ]; then
  mkdir -p "$bin"
  curl -sSfL "https://github.com/wasm-bindgen/wasm-bindgen/releases/download/${version}/wasm-bindgen-${version}-x86_64-unknown-linux-musl.tar.gz" \
    | tar -xz --strip-components=1 -C "$bin" \
    || cargo install wasm-bindgen-cli --version "$version" --locked --root "$bin" --bin wasm-bindgen
  [ -x "$bin/wasm-bindgen" ] || mv "$bin/bin/wasm-bindgen" "$bin/"
fi
"$bin/wasm-bindgen" --target web --no-typescript --remove-name-section --remove-producers-section \
  --out-dir pkg \
  "$target/wasm32-unknown-unknown/wasm-release/frigo_site.wasm"
echo "       îlots prêts dans site/pkg"
