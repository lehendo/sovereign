const fs = require("fs");
const path = require("path");

const sizes = [16, 32, 64, 128, 256];
const iconsDir = __dirname;

function createIco() {
  const headerSize = 6;
  const entrySize = 16;
  let offset = headerSize + entrySize * sizes.length;

  const entries = [];
  const imageData = [];

  for (const size of sizes) {
    const filename = path.join(iconsDir, `${size}x${size}.png`);
    const data = fs.readFileSync(filename);

    const entry = Buffer.alloc(entrySize);
    entry[0] = size === 256 ? 0 : size;
    entry[1] = size === 256 ? 0 : size;
    entry[2] = 0;
    entry[3] = 0;
    entry.writeUInt16LE(1, 4);
    entry.writeUInt16LE(32, 6);
    entry.writeUInt32LE(data.length, 8);
    entry.writeUInt32LE(offset, 12);

    entries.push(entry);
    imageData.push(data);
    offset += data.length;
  }

  const header = Buffer.alloc(headerSize);
  header.writeUInt16LE(0, 0); // reserved
  header.writeUInt16LE(1, 2); // icon type
  header.writeUInt16LE(sizes.length, 4); // image count

  const icoBuffer = Buffer.concat([header, ...entries, ...imageData]);
  fs.writeFileSync(path.join(iconsDir, "icon.ico"), icoBuffer);
}

createIco();

