const toIco = require('to-ico');
const fs = require('fs');
const path = require('path');

const iconsDir = path.join(__dirname, '..', 'src-tauri', 'icons');

async function createIco() {
  const sizes = [16, 32, 48, 64, 128, 256];
  const pngBuffers = [];

  // Read all the PNG files we need
  for (const size of sizes) {
    let pngPath;
    if (size === 32) {
      pngPath = path.join(iconsDir, '32x32.png');
    } else if (size === 128) {
      pngPath = path.join(iconsDir, '128x128.png');
    } else if (size === 256) {
      pngPath = path.join(iconsDir, 'icon-256.png');
    } else {
      // For other sizes, use the closest available
      pngPath = path.join(iconsDir, 'icon-256.png');
    }

    if (fs.existsSync(pngPath)) {
      pngBuffers.push(fs.readFileSync(pngPath));
    }
  }

  if (pngBuffers.length === 0) {
    console.error('No PNG files found!');
    return;
  }

  try {
    const icoBuffer = await toIco(pngBuffers);
    fs.writeFileSync(path.join(iconsDir, 'icon.ico'), icoBuffer);
    console.log('Created: icon.ico');
  } catch (error) {
    console.error('Error creating ICO:', error);
  }
}

createIco();
