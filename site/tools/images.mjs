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

// L'image de partage en anglais et en espagnol : les mêmes dessins, d'autres mots.
const translate = (pairs) => (text) =>
  pairs.reduce((out, [from, to]) => out.replace(from, to), text);
const english = translate([
  [">Le frigo est vide ?<", ">Empty fridge?<"],
  [">Pas l'assiette.<", ">Full plate.<"],
  ['textLength="536"', 'textLength="420"'],
  ['width="600" height="118" rx="22" fill="#1f1a3d"', 'width="484" height="118" rx="22" fill="#1f1a3d"'],
  ['width="600" height="118" rx="22" fill="#ffd23f"', 'width="484" height="118" rx="22" fill="#ffd23f"'],
  [">Vos ingrédients → la meilleure recette, en un clic.<", ">Your ingredients → the best recipe, in one tap.<"],
]);
const spanish = translate([
  [">Le frigo est vide ?<", ">¿La nevera vacía?<"],
  [">Pas l'assiette.<", ">El plato no.<"],
  ['textLength="536"', 'textLength="470"'],
  ['width="600" height="118" rx="22" fill="#1f1a3d"', 'width="534" height="118" rx="22" fill="#1f1a3d"'],
  ['width="600" height="118" rx="22" fill="#ffd23f"', 'width="534" height="118" rx="22" fill="#ffd23f"'],
  [">Vos ingrédients → la meilleure recette, en un clic.<", ">Tus ingredientes → la mejor receta, en un toque.<"],
]);

// [fichier, source, largeur, hauteur, fond, échelle] : l'image de partage est dessinée
// en 2× (2400 × 1260), nette même zoomée sur un téléphone.
const images = [
  ["icon-192.png", "logo.svg", 192, 192, "transparent"],
  ["icon-512.png", "logo.svg", 512, 512, "transparent"],
  ["icon-maskable-512.png", "icon-maskable.svg", 512, 512, "#4cc9f0"],
  ["apple-touch-icon.png", "icon-maskable.svg", 180, 180, "#4cc9f0"],
  ["favicon-32.png", "logo.svg", 32, 32, "transparent"],
  ["og.png", "og-card.svg", 1200, 630, "#fff8ec", 2],
  ["og-en.png", "og-card.svg", 1200, 630, "#fff8ec", 2, english],
  ["og-es.png", "og-card.svg", 1200, 630, "#fff8ec", 2, spanish],
];

const browser = await chromium.launch();
for (const [file, source, width, height, background, scale = 1, words = (t) => t] of images) {
  const page = await browser.newPage({ deviceScaleFactor: scale });
  await page.setViewportSize({ width, height });
  const inner = words(svg(source)).replace("<svg ", `<svg width="${width}" height="${height}" `);
  await page.setContent(
    `<html><body style="margin:0;background:${background}">${inner}</body></html>`,
  );
  await page.screenshot({ path: join(assets, file), omitBackground: background === "transparent" });
  await page.close();
  console.log(file);
}
await browser.close();
