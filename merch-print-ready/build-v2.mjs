#!/usr/bin/env node
/**
 * 4DA Merch v2 — Improved designs build
 *
 * Generates v2 versions of the selected 6 designs with professional polish.
 * Does NOT overwrite originals — outputs to merch-print-ready/png/v2/
 *
 * Usage: node merch-print-ready/build-v2.mjs
 */

import sharp from 'sharp';
import { mkdir } from 'fs/promises';
import { join, basename } from 'path';
import { existsSync, readFileSync } from 'fs';
import { fileURLToPath } from 'url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const DPI = 300;
const CANVAS_W = 4500;
const CANVAS_H = 5400;

const V2_DIR = join(__dirname, 'png', 'v2');
const V2_TEES = join(V2_DIR, 'tees');
const TEES_DIR = join(__dirname, 'tees');
const SUN_PATH = join(__dirname, 'upscaled', '1hero-sun-transparent-gigapixel-high fidelity v2-3000w.png');
const DA_SHIRT_PATH = join(__dirname, 'upscaled', '114da t shirt-gigapixel-high fidelity v2-2x-gigapixel-high fidelity v2-3000w.png');

/**
 * Render SVG to PNG centered on Printful canvas.
 */
async function svgToCanvas(svgPath, outputPath, designW, designH) {
  const svgBuffer = await readFileSync(svgPath);
  const svgMeta = await sharp(svgBuffer).metadata();
  const maxIntermediate = 6000;
  const naturalMax = Math.max(svgMeta.width, svgMeta.height);
  const density = Math.min(Math.floor((maxIntermediate / naturalMax) * 72), DPI);

  const designPng = await sharp(svgBuffer, { density })
    .resize(designW, designH, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
    .png()
    .toBuffer();

  const left = Math.round((CANVAS_W - designW) / 2);
  const top = Math.round((CANVAS_H - designH) / 2);

  await sharp({ create: { width: CANVAS_W, height: CANVAS_H, channels: 4, background: { r: 0, g: 0, b: 0, alpha: 0 } } })
    .composite([{ input: designPng, left, top }])
    .png({ compressionLevel: 6 })
    .toFile(outputPath);

  const meta = await sharp(outputPath).metadata();
  console.log(`  [ok] ${basename(outputPath)} — ${meta.width}x${meta.height}px`);
}

/**
 * Build improved sun + "4DA" + tagline composition.
 * Fixes: remove orange arc, tighten spacing, gold "4DA", better hierarchy.
 */
async function buildSunDesignV2(outputPath) {
  console.log('\n  [sun-v2] Building improved sun composition...');

  // Step 1: Load and process the sun
  const sunSize = 2800;
  const { data, info } = await sharp(SUN_PATH)
    .resize(sunSize, sunSize, { fit: 'cover', position: 'centre', kernel: 'lanczos3' })
    .raw()
    .ensureAlpha()
    .toBuffer({ resolveWithObject: true });

  // Remove black background
  for (let i = 0; i < data.length; i += 4) {
    const lum = 0.299 * data[i] + 0.587 * data[i + 1] + 0.114 * data[i + 2];
    if (lum < 12) data[i + 3] = 0;
    else if (lum < 45) data[i + 3] = Math.round(((lum - 12) / 33) * 255);
  }

  const sunPng = await sharp(data, { raw: { width: sunSize, height: sunSize, channels: 4 } })
    .png()
    .toBuffer();

  console.log(`  [sun-v2] Sun processed at ${sunSize}x${sunSize}`);

  // Step 2: Bold text — matches original da-t-shirt style, white for black fabric
  const textBlockH = 600;
  const textSvg = `<svg width="${CANVAS_W}" height="${textBlockH}" xmlns="http://www.w3.org/2000/svg">
    <text x="${CANVAS_W / 2}" y="280"
      text-anchor="middle"
      font-family="'Arial Black', 'Helvetica Neue', Arial, sans-serif"
      font-size="320" font-weight="900" letter-spacing="12"
      fill="#FFFFFF">4DA</text>
    <text x="${CANVAS_W / 2}" y="480"
      text-anchor="middle"
      font-family="'Arial', 'Helvetica Neue', sans-serif"
      font-size="120" font-weight="400" letter-spacing="6"
      fill="#F97316">All signal. No feed.</text>
  </svg>`;

  const textPng = await sharp(Buffer.from(textSvg), { density: DPI })
    .resize(CANVAS_W, textBlockH, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
    .png()
    .toBuffer();

  // Step 3: Orange loading arc — left side of sun
  const arcSize = sunSize + 200;
  const arcR = arcSize / 2 - 20;
  const arcSvg = `<svg width="${arcSize}" height="${arcSize}" xmlns="http://www.w3.org/2000/svg">
    <path d="M ${arcSize / 2} ${arcSize / 2 - arcR}
             A ${arcR} ${arcR} 0 0 0 ${arcSize / 2 - arcR * Math.sin(Math.PI / 3)} ${arcSize / 2 + arcR * Math.cos(Math.PI / 3)}"
      fill="none" stroke="#F97316" stroke-width="16" stroke-linecap="round" opacity="0.85"/>
  </svg>`;

  const arcPng = await sharp(Buffer.from(arcSvg), { density: DPI })
    .resize(arcSize, arcSize, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
    .png()
    .toBuffer();

  // Step 4: Compose on canvas
  const sunLeft = Math.round((CANVAS_W - sunSize) / 2);
  const sunTop = 400;
  const arcLeft = Math.round((CANVAS_W - arcSize) / 2);
  const arcTop = sunTop - 100;
  const textTop = sunTop + sunSize + 100;

  await sharp({ create: { width: CANVAS_W, height: CANVAS_H, channels: 4, background: { r: 0, g: 0, b: 0, alpha: 0 } } })
    .composite([
      { input: arcPng, left: arcLeft, top: arcTop },
      { input: sunPng, left: sunLeft, top: sunTop },
      { input: textPng, left: 0, top: textTop },
    ])
    .png({ compressionLevel: 6 })
    .toFile(outputPath);

  const meta = await sharp(outputPath).metadata();
  console.log(`  [ok] ${basename(outputPath)} — ${meta.width}x${meta.height}px`);
}

/**
 * Build sun orb with minimal "4DA" brand mark below.
 * Fixes: adds brand context so it's not just a random orb.
 */
async function buildSunBrandedV2(outputPath) {
  console.log('\n  [sun-branded] Building branded sun orb...');

  const sunSize = 3200;
  const { data, info } = await sharp(SUN_PATH)
    .resize(sunSize, sunSize, { fit: 'cover', position: 'centre', kernel: 'lanczos3' })
    .raw()
    .ensureAlpha()
    .toBuffer({ resolveWithObject: true });

  for (let i = 0; i < data.length; i += 4) {
    const lum = 0.299 * data[i] + 0.587 * data[i + 1] + 0.114 * data[i + 2];
    if (lum < 12) data[i + 3] = 0;
    else if (lum < 45) data[i + 3] = Math.round(((lum - 12) / 33) * 255);
  }

  const sunPng = await sharp(data, { raw: { width: sunSize, height: sunSize, channels: 4 } })
    .png()
    .toBuffer();

  // Sun only — no text. The "4" in the sun IS the brand.
  const sunLeft = Math.round((CANVAS_W - sunSize) / 2);
  const sunTop = Math.round((CANVAS_H - sunSize) / 2);

  await sharp({ create: { width: CANVAS_W, height: CANVAS_H, channels: 4, background: { r: 0, g: 0, b: 0, alpha: 0 } } })
    .composite([
      { input: sunPng, left: sunLeft, top: sunTop },
    ])
    .png({ compressionLevel: 6 })
    .toFile(outputPath);

  const meta = await sharp(outputPath).metadata();
  console.log(`  [ok] ${basename(outputPath)} — ${meta.width}x${meta.height}px`);
}

/**
 * Build a better "4" logo — thicker weight, slightly stylized.
 * Uses bold weight and tighter proportions.
 */
async function buildFourLogoV2(outputPath) {
  console.log('\n  [4-v2] Building improved "4" logo...');

  const logoSize = 3000;
  const svg = `<svg width="${logoSize}" height="${logoSize}" xmlns="http://www.w3.org/2000/svg">
    <text x="${logoSize / 2}" y="${logoSize * 0.74}"
      text-anchor="middle"
      font-family="'Arial Black', 'Helvetica Neue', Arial, sans-serif"
      font-size="${Math.round(logoSize * 0.75)}"
      font-weight="900"
      fill="white">4</text>
  </svg>`;

  const logoPng = await sharp(Buffer.from(svg), { density: DPI })
    .resize(logoSize, logoSize, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
    .png()
    .toBuffer();

  // Center on Printful canvas
  const left = Math.round((CANVAS_W - logoSize) / 2);
  const top = Math.round((CANVAS_H - logoSize) / 2);

  await sharp({ create: { width: CANVAS_W, height: CANVAS_H, channels: 4, background: { r: 0, g: 0, b: 0, alpha: 0 } } })
    .composite([{ input: logoPng, left, top }])
    .png({ compressionLevel: 6 })
    .toFile(outputPath);

  const meta = await sharp(outputPath).metadata();
  console.log(`  [ok] ${basename(outputPath)} — ${meta.width}x${meta.height}px`);
}

// ─── Main ─────────────────────────────────────────────────────

async function main() {
  console.log('╔══════════════════════════════════════════════╗');
  console.log('║  4DA Merch v2 — Professional Polish         ║');
  console.log('║  Output: merch-print-ready/png/v2/          ║');
  console.log('╚══════════════════════════════════════════════╝\n');

  await mkdir(V2_TEES, { recursive: true });

  // ── SVG designs ──
  console.log('── SVG Designs (v2) ──');

  // 1. Code Fragment v2 — taller canvas, One Dark syntax colors
  await svgToCanvas(
    join(TEES_DIR, '05-code-fragment-tee-v2.svg'),
    join(V2_TEES, '05-code-fragment-tee-v2-black.png'),
    4200, 5200
  );

  // 2. Void Pulse v2 — bold strokes, no filter, white pulse
  await svgToCanvas(
    join(TEES_DIR, '03-void-pulse-tee-v2.svg'),
    join(V2_TEES, '03-void-pulse-tee-v2-black.png'),
    3600, 4200
  );

  // 3. Streets Wordmark v2 — statement piece
  await svgToCanvas(
    join(TEES_DIR, '07-streets-wordmark-tee-v2.svg'),
    join(V2_TEES, '07-streets-wordmark-tee-v2-black.png'),
    4500, 3000
  );

  // ── Raster compositions ──
  console.log('\n── Raster Compositions (v2) ──');

  // 4. Sun + "4DA" + tagline — the hero tee
  if (existsSync(SUN_PATH)) {
    await buildSunDesignV2(join(V2_TEES, 'da-t-shirt-v2-black.png'));
  }

  // 5. Sun orb branded — sun + subtle "4DA" below
  if (existsSync(SUN_PATH)) {
    await buildSunBrandedV2(join(V2_TEES, 'hero-sun-branded-v2-black.png'));
  }

  // 6. White "4" logo — heavier weight
  await buildFourLogoV2(join(V2_TEES, '4-logo-v2-black.png'));

  // ── Summary ───────────────────────────────────
  console.log('\n══════════════════════════════════════════════');
  console.log('v2 build complete! Output: ' + V2_DIR);
  console.log('\nAll files: 4500x5400px Printful-ready canvas');
  console.log('Use on BLACK tees only (Bella+Canvas 3001 Black)');
  console.log('══════════════════════════════════════════════');
}

main().catch((err) => {
  console.error('\nBuild failed:', err.message);
  console.error(err.stack);
  process.exit(1);
});
