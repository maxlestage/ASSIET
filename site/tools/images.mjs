// Les images PNG du site, dessinées depuis leurs SVG (public/assets) par Chromium :
// les icônes de l'app installable (PWA) et l'image des aperçus de partage.
//
//   cd site && node tools/images.mjs     (Playwright et Chromium installés)
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// Playwright est souvent installé globalement : require le trouve (NODE_PATH).
const { chromium } = createRequire(import.meta.url)("playwright");
const assets = join(dirname(fileURLToPath(import.meta.url)), "..", "public", "assets");
const svg = (name) => readFileSync(join(assets, name), "utf8");

// [fichier, source, largeur, hauteur, fond]
const images = [
  ["icon-192.png", "logo.svg", 192, 192, "transparent"],
  ["icon-512.png", "logo.svg", 512, 512, "transparent"],
  ["icon-maskable-512.png", "icon-maskable.svg", 512, 512, "#4cc9f0"],
  ["apple-touch-icon.png", "icon-maskable.svg", 180, 180, "#4cc9f0"],
  ["favicon-32.png", "logo.svg", 32, 32, "transparent"],
  ["og.png", "og-card.svg", 1200, 630, "#fff8ec"],
];

const browser = await chromium.launch();
const page = await browser.newPage({ deviceScaleFactor: 1 });
for (const [file, source, width, height, background] of images) {
  await page.setViewportSize({ width, height });
  const inner = svg(source).replace("<svg ", `<svg width="${width}" height="${height}" `);
  await page.setContent(
    `<html><body style="margin:0;background:${background}">${inner}</body></html>`,
  );
  await page.screenshot({ path: join(assets, file), omitBackground: background === "transparent" });
  console.log(file);
}
await browser.close();
