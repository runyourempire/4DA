const sharp = require('sharp');
const fs = require('fs');
const path = require('path');

const iconsDir = path.join(__dirname, '..', 'src-tauri', 'icons');

// Create SVG for the "4" icon
function createSvg(size) {
  const fontSize = Math.floor(size * 0.75);
  const yOffset = Math.floor(size * 0.58); // Raised higher for better centering

  return `<svg width="${size}" height="${size}" xmlns="http://www.w3.org/2000/svg">
    <rect width="${size}" height="${size}" fill="#0A0A0A"/>
    <text x="50%" y="${yOffset}"
          font-family="Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif"
          font-size="${fontSize}"
          font-weight="700"
          fill="white"
          text-anchor="middle"
          dominant-baseline="middle">4</text>
  </svg>`;
}

async function generateIcons() {
  const sizes = [
    { name: '32x32.png', size: 32 },
    { name: '128x128.png', size: 128 },
    { name: '128x128@2x.png', size: 256 },
    { name: 'icon.png', size: 512 },
    { name: 'Square30x30Logo.png', size: 30 },
    { name: 'Square44x44Logo.png', size: 44 },
    { name: 'Square71x71Logo.png', size: 71 },
    { name: 'Square89x89Logo.png', size: 89 },
    { name: 'Square107x107Logo.png', size: 107 },
    { name: 'Square142x142Logo.png', size: 142 },
    { name: 'Square150x150Logo.png', size: 150 },
    { name: 'Square284x284Logo.png', size: 284 },
    { name: 'Square310x310Logo.png', size: 310 },
    { name: 'StoreLogo.png', size: 50 },
  ];

  console.log('Generating icons...');

  for (const { name, size } of sizes) {
    const svg = Buffer.from(createSvg(size));
    const outputPath = path.join(iconsDir, name);

    await sharp(svg)
      .png()
      .toFile(outputPath);

    console.log(`Created: ${name} (${size}x${size})`);
  }

  // Create ICO file (Windows)
  const icoSizes = [16, 32, 48, 64, 128, 256];
  const icoBuffers = await Promise.all(
    icoSizes.map(async (size) => {
      const svg = Buffer.from(createSvg(size));
      return sharp(svg).png().toBuffer();
    })
  );

  // For ICO, we'll just use the 256x256 as the main icon
  // Sharp doesn't directly create ICO, so we'll create a large PNG
  const svg256 = Buffer.from(createSvg(256));
  await sharp(svg256)
    .png()
    .toFile(path.join(iconsDir, 'icon-256.png'));

  console.log('Created: icon-256.png for ICO conversion');

  // Create ICNS placeholder (macOS) - just copy the 512 version
  const svg512 = Buffer.from(createSvg(512));
  await sharp(svg512)
    .png()
    .toFile(path.join(iconsDir, 'icon-512.png'));

  console.log('Created: icon-512.png for ICNS conversion');

  console.log('\nDone! Icons generated in src-tauri/icons/');
  console.log('Note: For production, convert icon-256.png to icon.ico using an online converter');
}

generateIcons().catch(console.error);
