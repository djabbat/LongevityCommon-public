/**
 * Generates PWA icons from public/icon.svg
 * Run: pnpm gen-icons
 * Requires: sharp (installed as devDependency)
 */

import sharp from 'sharp'
import { fileURLToPath } from 'url'
import { dirname, join } from 'path'

const __dirname = dirname(fileURLToPath(import.meta.url))
const src = join(__dirname, '../public/icon.svg')
const out = join(__dirname, '../public')

const sizes = [192, 512]

for (const size of sizes) {
  await sharp(src)
    .resize(size, size)
    .png()
    .toFile(join(out, `icon-${size}.png`))

  console.log(`✓ icon-${size}.png`)
}

console.log('PWA icons generated in web/public/')
