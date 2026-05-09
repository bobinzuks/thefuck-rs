import { readdir, writeFile } from 'fs/promises';
import { fileURLToPath } from 'url';
import path from 'path';

const ROUTER = '/run/media/bobinzuks/82ae4087-99fd-4170-b394-3f47ac90dca1/projects/old-ruflo-personal/hooks/tile-first-router.mjs';
const SOURCE_DIR = '/run/media/bobinzuks/82ae4087-99fd-4170-b394-3f47ac90dca1/projects/ruflo-personal/projects/tile-router-project/source-python/thefuck/rules';
const RULES_OUT = './src/rules';

// Import router once — tiles load once
const { route } = await import(ROUTER);

const files = await readdir(SOURCE_DIR);
let tileHits = 0, deepseekHits = 0, failed = 0;

for (const f of files) {
  if (!f.endsWith('.py') || f === '__init__.py') continue;
  const ruleName = f.replace('.py', '');

  try {
    const result = await route(`${ruleName} rule convert to rust`);
    if (result.answer) {
      await writeFile(`${RULES_OUT}/${ruleName}.rs`, result.answer);
      if (result.source === 'tile-code') {
        console.log(`⚡ ${ruleName} → TILE (0 tokens)`);
        tileHits++;
      } else {
        console.log(`💰 ${ruleName} → ${result.source}`);
        deepseekHits++;
      }
    } else {
      console.log(`❌ ${ruleName} failed`);
      failed++;
    }
  } catch(e) {
    console.log(`❌ ${ruleName} error: ${e.message}`);
    failed++;
  }
}

console.log(`\n⚡ Tile hits:     ${tileHits} (free)`);
console.log(`💰 DeepSeek hits: ${deepseekHits} (paid)`);
console.log(`❌ Failed:        ${failed}`);
