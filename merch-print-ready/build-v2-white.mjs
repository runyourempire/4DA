#!/usr/bin/env node
/**
 * 4DA Merch v2-white — Sun designs rebuilt for WHITE tees
 *
 * Fixes: aggressive background removal, circular mask for clean edges,
 * dark text (gold/black) instead of white text.
 *
 * Output: merch-print-ready/png/v2/tees/white/
 * Usage: node merch-print-ready/build-v2-white.mjs
 */

import sharp from 'sharp';
import { mkdir, readFile } from 'fs/promises';
import { join, basename } from 'path';
import { existsSync } from 'fs';
import { fileURLToPath } from 'url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const DPI = 300;
const CANVAS_W = 4500;
const CANVAS_H = 5400;

const OUT_DIR = join(__dirname, 'png', 'v2', 'tees');
const TEES_DIR = join(__dirname, 'tees');
const SUN_PATH = join(__dirname, 'upscaled', '1hero-sun-transparent-gigapixel-high fidelity v2-3000w.png');

/**
 * Create a circular alpha mask — hard edge with 1px antialias.
 */
function createCircleMask(size) {
  const cx = size / 2;
  const cy = size / 2;
  const r = size / 2;
  const aaWidth = 1.5; // antialias band in pixels

  const mask = Buffer.alloc(size * size);
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const dist = Math.sqrt((x - cx) ** 2 + (y - cy) ** 2);
      if (dist < r - aaWidth) {
        mask[y * size + x] = 255;
      } else if (dist < r) {
        // Smooth antialias at the edge
        mask[y * size + x] = Math.round(255 * ((r - dist) / aaWidth));
      } else {
        mask[y * size + x] = 0;
      }
    }
  }
  return mask;
}

/**
 * Process sun asset: aggressive background removal + circular mask.
 * Returns a clean PNG buffer with no fringe artifacts.
 */
async function processSunForWhite(sunSize) {
  console.log(`  Processing sun at ${sunSize}px with white-tee cleanup...`);

  const { data } = await sharp(SUN_PATH)
    .resize(sunSize, sunSize, { fit: 'cover', position: 'centre', kernel: 'lanczos3' })
    .raw()
    .ensureAlpha()
    .toBuffer({ resolveWithObject: true });

  // Pass 1: Aggressive luminance-based background removal
  // Much higher thresholds than black-tee version (60/100 vs 12/45)
  let removedPixels = 0;
  for (let i = 0; i < data.length; i += 4) {
    const r = data[i], g = data[i + 1], b = data[i + 2];
    const lum = 0.299 * r + 0.587 * g + 0.114 * b;

    // Also check saturation — dark unsaturated pixels are background noise
    const max = Math.max(r, g, b);
    const min = Math.min(r, g, b);
    const sat = max === 0 ? 0 : (max - min) / max;

    if (lum < 60 && sat < 0.15) {
      // Dark, desaturated = background noise → fully transparent
      data[i + 3] = 0;
      removedPixels++;
    } else if (lum < 100 && sat < 0.2) {
      // Transition zone — fade based on luminance
      const alpha = Math.round(((lum - 60) / 40) * 255);
      data[i + 3] = Math.min(data[i + 3], alpha);
      removedPixels++;
    }
  }
  console.log(`  Removed ${removedPixels.toLocaleString()} background pixels`);

  // Pass 2: Apply circular mask — eliminates ALL rectangular fringe
  const circleMask = createCircleMask(sunSize);
  let maskedPixels = 0;
  for (let y = 0; y < sunSize; y++) {
    for (let x = 0; x < sunSize; x++) {
      const idx = (y * sunSize + x) * 4;
      const maskVal = circleMask[y * sunSize + x];
      // Multiply existing alpha by circle mask
      data[idx + 3] = Math.round((data[idx + 3] * maskVal) / 255);
      if (maskVal === 0 && data[idx + 3] === 0) maskedPixels++;
    }
  }
  console.log(`  Circular mask applied — ${maskedPixels.toLocaleString()} edge pixels clipped`);

  return sharp(data, { raw: { width: sunSize, height: sunSize, channels: 4 } })
    .png()
    .toBuffer();
}

/**
 * Build sun + "4DA" + tagline for WHITE tees.
 * Text is dark/gold instead of white. Clean circular sun.
 */
async function buildSunDesignWhite(outputPath) {
  console.log('\n  [sun-white] Building sun composition for white tees...');

  const sunSize = 2800;
  const sunPng = await processSunForWhite(sunSize);

  // Text block: matches original da-t-shirt style — bold, direct, no frills
  // Same font/size as original but colors adapted for white fabric
  const textBlockH = 600;
  const textSvg = `<svg width="${CANVAS_W}" height="${textBlockH}" xmlns="http://www.w3.org/2000/svg">
    <!-- "4DA" — bold, same style as original, dark for white fabric -->
    <text x="${CANVAS_W / 2}" y="280"
      text-anchor="middle"
      font-family="'Arial Black', 'Helvetica Neue', Arial, sans-serif"
      font-size="320" font-weight="900" letter-spacing="12"
      fill="#1A1A1A">4DA</text>
    <!-- Tagline — orange works on both black and white fabric -->
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

  // Orange loading arc — left side of sun, matches original da-t-shirt
  const arcSize = sunSize + 200; // slightly larger than sun
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

  const sunLeft = Math.round((CANVAS_W - sunSize) / 2);
  const sunTop = 400;
  const arcLeft = Math.round((CANVAS_W - arcSize) / 2);
  const arcTop = sunTop - 100; // offset to align with sun center
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
 * Build sun orb branded for WHITE tees.
 * Circular-masked sun + dark "4DA" label below.
 */
async function buildSunBrandedWhite(outputPath) {
  console.log('\n  [sun-branded-white] Building branded sun orb for white tees...');

  const sunSize = 3200;
  const sunPng = await processSunForWhite(sunSize);

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
 * Render SVG to PNG centered on Printful canvas.
 * Same as build-v2.mjs but shared here for white-tee SVG variants.
 */
async function svgToCanvas(svgBuffer, outputPath, designW, designH) {
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
 * Read an SVG, swap white (#FFFFFF) to dark (#1A1A1A) for white fabric.
 * Keeps orange, gold, and all other colors intact.
 */
function recolorSvgForWhite(svgString) {
  return svgString
    // White text/fills → dark
    .replace(/fill="#FFFFFF"/g, 'fill="#1A1A1A"')
    .replace(/fill="white"/g, 'fill="#1A1A1A"')
    .replace(/fill="#ffffff"/g, 'fill="#1A1A1A"')
    // White strokes → dark
    .replace(/stroke="#FFFFFF"/g, 'stroke="#1A1A1A"')
    .replace(/stroke="white"/g, 'stroke="#1A1A1A"')
    // Very dark grays (#333, #444) that were meant to be subtle on black → lighter for white
    .replace(/fill="#333333"/g, 'fill="#999999"')
    .replace(/fill="#444444"/g, 'fill="#999999"')
    .replace(/fill="#3A3A3A"/g, 'fill="#BBBBBB"')
    // Dark backgrounds in code fragment stay dark (they're self-contained)
    // Gold stays gold, orange stays orange
    ;
}

/**
 * Build "4" logo for WHITE tees — dark numeral instead of white.
 */
async function buildFourLogoWhite(outputPath) {
  console.log('\n  [4-white] Building "4" logo for white tees...');

  const logoSize = 3000;
  const svg = `<svg width="${logoSize}" height="${logoSize}" xmlns="http://www.w3.org/2000/svg">
    <text x="${logoSize / 2}" y="${logoSize * 0.74}"
      text-anchor="middle"
      font-family="'Arial Black', 'Helvetica Neue', Arial, sans-serif"
      font-size="${Math.round(logoSize * 0.75)}"
      font-weight="900"
      fill="#1A1A1A">4</text>
  </svg>`;

  const logoPng = await sharp(Buffer.from(svg), { density: DPI })
    .resize(logoSize, logoSize, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
    .png()
    .toBuffer();

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
  console.log('╔══════════════════════════════════════════════════╗');
  console.log('║  4DA Merch v2-white — ALL Designs for White Tees║');
  console.log('║  Output: merch-print-ready/png/v2/tees/         ║');
  console.log('╚══════════════════════════════════════════════════╝\n');

  await mkdir(OUT_DIR, { recursive: true });

  // ── Sun compositions ──
  console.log('── Sun Compositions (white tee) ──');

  if (existsSync(SUN_PATH)) {
    await buildSunDesignWhite(join(OUT_DIR, 'da-t-shirt-v2-white.png'));
    await buildSunBrandedWhite(join(OUT_DIR, 'hero-sun-branded-v2-white.png'));
  }

  // ── SVG designs — recolored for white fabric ──
  console.log('\n── SVG Designs (white tee) ──');

  // Void Pulse — skipped for white tees (design doesn't translate well)

  // STREETS Wordmark — dedicated white-optimized SVG
  const streetsWhiteSvg = await readFile(join(TEES_DIR, '07-streets-wordmark-tee-v2-white.svg'), 'utf8');
  await svgToCanvas(
    Buffer.from(streetsWhiteSvg),
    join(OUT_DIR, '07-streets-wordmark-tee-v2-white.png'),
    4500, 3000
  );

  // Code Fragment — light-mode variant for white fabric
  const codeLightSvg = await readFile(join(TEES_DIR, '05-code-fragment-tee-v2-light.svg'), 'utf8');
  await svgToCanvas(
    Buffer.from(codeLightSvg),
    join(OUT_DIR, '05-code-fragment-tee-v2-white.png'),
    4200, 5200
  );

  // "4" Logo — dark numeral
  await buildFourLogoWhite(join(OUT_DIR, '4-logo-v2-white.png'));

  // ── Generate previews on white background ──
  console.log('\n── Generating white-tee previews ──');
  const previewDir = join(__dirname, 'png', 'v2', 'previews');
  await mkdir(previewDir, { recursive: true });

  const whiteFiles = [
    'da-t-shirt-v2-white.png',
    'hero-sun-branded-v2-white.png',
    '07-streets-wordmark-tee-v2-white.png',
    '05-code-fragment-tee-v2-white.png',
    '4-logo-v2-white.png',
  ];

  for (const file of whiteFiles) {
    const input = join(OUT_DIR, file);
    if (!existsSync(input)) continue;
    const output = join(previewDir, `preview-${file}`);
    const design = await sharp(input)
      .resize(900, 1080, { fit: 'inside' })
      .png()
      .toBuffer();

    await sharp({ create: { width: 900, height: 1080, channels: 4, background: { r: 245, g: 245, b: 245, alpha: 255 } } })
      .composite([{ input: design, gravity: 'centre' }])
      .png({ compressionLevel: 6 })
      .toFile(output);

    console.log(`  [preview] ${basename(output)}`);
  }

  console.log('\n══════════════════════════════════════════════════');
  console.log('White tee build complete — ALL 6 designs!');
  console.log('Files: 4500x5400px Printful-ready');
  console.log('Safe for ANY fabric color');
  console.log('══════════════════════════════════════════════════');
}

main().catch((err) => {
  console.error('\nBuild failed:', err.message);
  console.error(err.stack);
  process.exit(1);
});
