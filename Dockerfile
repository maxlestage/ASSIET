# Le site de Frigo vide (une application Paquets, îlots WebAssembly, API /api/recettes).
# Heroku le construit avec le buildpack emk/rust (RustConfig, Procfile) ; cette image est
# l'autre voie : la pile container (heroku.yml), ou n'importe quel hôte Docker.
#
#   docker build -t frigo-vide . && docker run --rm -p 8080:8080 frigo-vide

FROM rust:1-slim-bookworm AS build
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates git \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add wasm32-unknown-unknown
WORKDIR /src
COPY . .
WORKDIR /src/site
RUN cargo build --lib --target wasm32-unknown-unknown --profile wasm-release \
    && version=$(grep -A1 '^name = "wasm-bindgen"$' Cargo.lock | sed -n 's/^version = "\(.*\)"$/\1/p') \
    && (curl -sSfL "https://github.com/wasm-bindgen/wasm-bindgen/releases/download/${version}/wasm-bindgen-${version}-x86_64-unknown-linux-musl.tar.gz" \
            | tar -xz --strip-components=1 -C /usr/local/bin \
        || cargo install wasm-bindgen-cli --version "$version" --locked) \
    && wasm-bindgen --target web --no-typescript --remove-name-section --remove-producers-section \
        --out-dir pkg target/wasm32-unknown-unknown/wasm-release/frigo_site.wasm
RUN cargo build --release --bin frigo-site && cp target/release/frigo-site /server

# Une petite image : le programme, ses fichiers statiques et les îlots.
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* && useradd --system --uid 10001 frigo
WORKDIR /app
COPY --from=build /server /usr/local/bin/server
COPY --from=build /src/site/public ./public
COPY --from=build /src/site/pkg ./pkg
USER frigo
# Heroku donne PORT au démarrage ; 8080 ailleurs.
ENV PORT=8080
EXPOSE 8080
CMD ["server"]
