#!/usr/bin/env node
/**
 * 4DA Merch — SVG to PNG Conversion Pipeline
 *
 * Converts all print-ready SVG designs to Printful-compatible PNGs.
 * Also composites the hero-sun.jpg for the solar crown design.
 *
 * Usage: node merch-print-ready/build.mjs
 * Requirements: sharp (already in project dependencies)
 *
 * Output: merch-print-ready/png/tees/*.png, merch-print-ready/png/stickers/*.png
 */

import sharp from 'sharp';
import { readdir, mkdir, readFile } from 'fs/promises';
import { join, basename, extname } from 'path';
import { existsSync } from 'fs';
import { fileURLToPath } from 'url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const ROOT = join(__dirname, '..');

// Paths
const TEES_DIR = join(__dirname, 'tees');
const STICKERS_DIR = join(__dirname, 'stickers');
const PNG_TEES = join(__dirname, 'png', 'tees');
const PNG_STICKERS = join(__dirname, 'png', 'stickers');
// Prefer Topaz-upscaled sun if available, fall back to original
const SUN_UPSCALED = join(__dirname, 'upscaled', '1hero-sun-transparent-gigapixel-high fidelity v2-3000w.png');
const SUN_ORIGINAL = join(ROOT, 'site', 'hero-sun.jpg');
const SUN_PATH = existsSync(SUN_UPSCALED) ? SUN_UPSCALED : SUN_ORIGINAL;

// Print specs: Printful requires minimum 150 DPI, we target 300 DPI
const DPI = 300;

// Each design mapped to its pixel dimensions (inches * DPI)
const TEE_SPECS = {
  '01-4da-logo-tee':         { w: 4 * DPI, h: Math.round(2.5 * DPI) },
  '02-solar-crown-tee':      { w: 8 * DPI, h: 8 * DPI },
  '03-void-pulse-tee':       { w: 8 * DPI, h: 8 * DPI },
  '04-dimensions-tee':       { w: 8 * DPI, h: 8 * DPI },
  '05-code-fragment-tee':    { w: 12 * DPI, h: 16 * DPI },
  '06-pasifa-schematic-tee': { w: 12 * DPI, h: 12 * DPI },
  '07-streets-wordmark-tee': { w: 10 * DPI, h: Math.round(3.5 * DPI) },
  '08-for-the-streets-back': { w: 12 * DPI, h: 16 * DPI },
  '09-for-the-streets-front': { w: 4 * DPI, h: Math.round(1.5 * DPI) },
};

const STICKER_SPECS = {
  '4da-circle-badge':   { w: 3 * DPI, h: 3 * DPI },
  'void-pulse-mini':    { w: 3 * DPI, h: 3 * DPI },
  'privacy-first':      { w: Math.round(3.5 * DPI), h: Math.round(1.5 * DPI) },
  'streets-pill':       { w: Math.round(3.5 * DPI), h: Math.round(1.3 * DPI) },
  'assert-stays-local': { w: 4 * DPI, h: Math.round(1.1 * DPI) },
};

/**
 * Convert an SVG file to PNG at specified dimensions.
 * Uses transparent background (essential for DTG printing on dark fabric).
 */
async function svgToPng(svgPath, outputPath, width, height) {
  const svgBuffer = await readFile(svgPath);

  // Calculate density to render SVG close to target size without exceeding pixel limits.
  // SVGs at density 72 render at their viewBox px dimensions. For large SVGs targeting
  // large outputs, we render at native size then resize — avoids sharp's pixel limit.
  // For small SVGs targeting large outputs, we boost density for quality.
  const svgMeta = await sharp(svgBuffer).metadata();
  const scaleFactor = Math.max(width / svgMeta.width, height / svgMeta.height);
  // Cap intermediate render at 6000px on longest side to stay within limits
  const maxIntermediate = 6000;
  const naturalMax = Math.max(svgMeta.width, svgMeta.height);
  const density = Math.min(
    Math.floor((maxIntermediate / naturalMax) * 72),
    DPI
  );

  await sharp(svgBuffer, { density })
    .resize(width, height, {
      fit: 'contain',
      background: { r: 0, g: 0, b: 0, alpha: 0 },
    })
    .png({ compressionLevel: 6 })
    .toFile(outputPath);

  const meta = await sharp(outputPath).metadata();
  console.log(`  [ok] ${basename(outputPath)} — ${meta.width}x${meta.height}px`);
}

/**
 * Build the solar crown design from the sun source image.
 *
 * When using the Topaz-upscaled source (3000px, already has "4" baked in):
 *   1. Resize to target → remove black background → done
 *
 * When using the original 600px source:
 *   1. Upscale → remove black background → overlay system-font "4"
 */
async function buildSunComposite(outputPath, size) {
  const isUpscaled = SUN_PATH.includes('upscaled');
  console.log(`\n  [sun] Building solar crown ${isUpscaled ? '(Topaz HiFi source)' : '(original 600px)'}...`);

  // Step 1: Resize to target size, crop to square
  const sunResized = await sharp(SUN_PATH)
    .resize(size, size, {
      fit: 'cover',
      position: 'centre',
      kernel: 'lanczos3',
    })
    .raw()
    .ensureAlpha()
    .toBuffer({ resolveWithObject: true });

  console.log(`  [sun] Resized to ${sunResized.info.width}x${sunResized.info.height}`);

  // Step 2: Remove black background via luminance threshold
  // For DTG on dark fabric: black → transparent (no ink), colors → opaque
  const pixels = sunResized.data;
  for (let i = 0; i < pixels.length; i += 4) {
    const r = pixels[i], g = pixels[i + 1], b = pixels[i + 2];
    const luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    if (luminance < 10) {
      pixels[i + 3] = 0;
    } else if (luminance < 40) {
      pixels[i + 3] = Math.round(((luminance - 10) / 30) * 255);
    }
  }

  console.log('  [sun] Black background removed');

  // Step 3: Create sun PNG buffer
  const sunPng = await sharp(pixels, {
    raw: {
      width: sunResized.info.width,
      height: sunResized.info.height,
      channels: 4,
    },
  })
    .png()
    .toBuffer();

  if (isUpscaled) {
    // Topaz source already has the "4" baked in at high quality — just save
    await sharp(sunPng)
      .png({ compressionLevel: 6 })
      .toFile(outputPath);
  } else {
    // Original source: overlay a system-font "4"
    const fourSvg = `<svg width="${size}" height="${size}" xmlns="http://www.w3.org/2000/svg">
      <text x="${size / 2}" y="${size * 0.68}"
        text-anchor="middle"
        font-family="Arial, Helvetica, sans-serif"
        font-size="${Math.round(size * 0.55)}"
        font-weight="600"
        fill="white"
        opacity="0.95">4</text>
    </svg>`;

    const fourPng = await sharp(Buffer.from(fourSvg), { density: DPI })
      .resize(size, size)
      .png()
      .toBuffer();

    console.log('  [sun] "4" overlay rendered (fallback mode)');

    await sharp(sunPng)
      .composite([{ input: fourPng, blend: 'over' }])
      .png({ compressionLevel: 6 })
      .toFile(outputPath);
  }

  const meta = await sharp(outputPath).metadata();
  console.log(`  [ok] ${basename(outputPath)} — ${meta.width}x${meta.height}px (sun composite)`);
}

/**
 * Build a clean high-res "4" logo on transparent background.
 * Creates a standalone asset for Printful upload.
 */
async function buildFourLogo(outputDir) {
  const size = 2400;
  console.log('\n  [4] Building standalone "4" logo...');

  const svg = `<svg width="${size}" height="${size}" xmlns="http://www.w3.org/2000/svg">
    <text x="${size / 2}" y="${size * 0.72}"
      text-anchor="middle"
      font-family="Arial, Helvetica, sans-serif"
      font-size="${Math.round(size * 0.7)}"
      font-weight="600"
      fill="white">4</text>
  </svg>`;

  const outputPath = join(outputDir, '4-logo-white-transparent.png');
  await sharp(Buffer.from(svg), { density: DPI })
    .resize(size, size)
    .png({ compressionLevel: 6 })
    .toFile(outputPath);

  const meta = await sharp(outputPath).metadata();
  console.log(`  [ok] ${basename(outputPath)} — ${meta.width}x${meta.height}px`);
}

/**
 * Remove black background from any raster image → transparent PNG.
 * Used for Topaz-upscaled assets that have black backgrounds.
 */
async function removeBlackBackground(inputPath, outputPath) {
  const { data, info } = await sharp(inputPath)
    .raw()
    .ensureAlpha()
    .toBuffer({ resolveWithObject: true });

  const w = info.width, h = info.height, ch = 4;

  // Step 1: Force edge rows/columns fully transparent.
  // Topaz upscaling creates artifact pixels at image borders (luminance 30-60)
  // that survive the threshold and create a visible unprofessional border.
  const EDGE_PX = 5;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      if (y < EDGE_PX || y >= h - EDGE_PX || x < EDGE_PX || x >= w - EDGE_PX) {
        data[(y * w + x) * ch + 3] = 0;
      }
    }
  }

  // Step 2: Luminance-based black removal on remaining pixels
  for (let i = 0; i < data.length; i += 4) {
    if (data[i + 3] === 0) continue; // already cleared by edge pass
    const r = data[i], g = data[i + 1], b = data[i + 2];
    const luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    if (luminance < 10) {
      data[i + 3] = 0;
    } else if (luminance < 40) {
      data[i + 3] = Math.round(((luminance - 10) / 30) * 255);
    }
  }

  // Step 3: Save then trim transparent borders for a clean edge
  await sharp(data, { raw: { width: w, height: h, channels: 4 } })
    .trim()
    .png({ compressionLevel: 6 })
    .toFile(outputPath);

  const meta = await sharp(outputPath).metadata();
  console.log(`  [ok] ${basename(outputPath)} — ${meta.width}x${meta.height}px (black → transparent, trimmed)`);
}

// ─── Main ─────────────────────────────────────────────────────

async function main() {
  console.log('╔══════════════════════════════════════════════╗');
  console.log('║  4DA Merch — SVG → PNG Build Pipeline       ║');
  console.log('║  Target: Printful DTG @ 300 DPI             ║');
  console.log('╚══════════════════════════════════════════════╝\n');

  // Ensure output dirs exist
  await mkdir(PNG_TEES, { recursive: true });
  await mkdir(PNG_STICKERS, { recursive: true });

  // ── Tees ──────────────────────────────────────
  console.log('── T-Shirts ──');

  for (const [name, spec] of Object.entries(TEE_SPECS)) {
    const svgPath = join(TEES_DIR, `${name}.svg`);

    if (!existsSync(svgPath)) {
      console.log(`  [skip] ${name}.svg — not found`);
      continue;
    }

    // Special case: solar crown uses hero-sun.jpg composite
    if (name === '02-solar-crown-tee') {
      if (existsSync(SUN_PATH)) {
        await buildSunComposite(join(PNG_TEES, `${name}.png`), spec.w);
      } else {
        console.log(`  [skip] ${name} — hero-sun.jpg not found at ${SUN_PATH}`);
        // Fall back to SVG render
        await svgToPng(svgPath, join(PNG_TEES, `${name}.png`), spec.w, spec.h);
      }
      continue;
    }

    await svgToPng(svgPath, join(PNG_TEES, `${name}.png`), spec.w, spec.h);
  }

  // ── Stickers ──────────────────────────────────
  console.log('\n── Stickers ──');

  for (const [name, spec] of Object.entries(STICKER_SPECS)) {
    const svgPath = join(STICKERS_DIR, `${name}.svg`);

    if (!existsSync(svgPath)) {
      console.log(`  [skip] ${name}.svg — not found`);
      continue;
    }

    await svgToPng(svgPath, join(PNG_STICKERS, `${name}.png`), spec.w, spec.h);
  }

  // ── Upscaled assets (Topaz) — remove black → transparent ──
  console.log('\n── Upscaled Assets (black → transparent) ──');
  const UPSCALED_DIR = join(__dirname, 'upscaled');

  if (existsSync(UPSCALED_DIR)) {
    const upscaledFiles = (await readdir(UPSCALED_DIR)).filter(f => f.endsWith('.png'));

    for (const file of upscaledFiles) {
      // Skip the sun — it's handled by the solar crown pipeline
      if (file.includes('hero-sun')) continue;

      const inputPath = join(UPSCALED_DIR, file);
      // Clean up the filename for output
      const cleanName = file
        .replace(/gigapixel-.*?(?=\.png)/g, '')
        .replace(/^[0-9!]+/, '')
        .replace(/[-_ ]+/g, '-')
        .replace(/-+\.png/, '.png')
        .replace(/^-/, '');
      const outputPath = join(__dirname, 'png', cleanName);

      await removeBlackBackground(inputPath, outputPath);
    }
  }

  // ── Bonus: standalone assets ──────────────────
  console.log('\n── Standalone Assets ──');
  await buildFourLogo(join(__dirname, 'png'));

  if (existsSync(SUN_PATH)) {
    const sunSize = 3000;
    console.log('\n  [sun] Building standalone sun asset...');
    await buildSunComposite(join(__dirname, 'png', 'hero-sun-transparent.png'), sunSize);
  }

  // ── Summary ───────────────────────────────────
  console.log('\n══════════════════════════════════════════════');
  console.log('Build complete! Output:');
  console.log(`  Tees:     ${PNG_TEES}`);
  console.log(`  Stickers: ${PNG_STICKERS}`);
  console.log(`  Assets:   ${join(__dirname, 'png')}`);
  console.log('\nUpload PNGs directly to Printful. All files have:');
  console.log('  - Transparent backgrounds (for dark fabric DTG)');
  console.log('  - 300 DPI resolution');
  console.log('  - PNG format (Printful-compatible)');
  console.log('══════════════════════════════════════════════');
}

main().catch((err) => {
  console.error('\nBuild failed:', err.message);
  process.exit(1);
});
