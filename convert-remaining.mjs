import { readdir, writeFile, access } from 'fs/promises';

const ROUTER = '/run/media/bobinzuks/82ae4087-99fd-4170-b394-3f47ac90dca1/projects/old-ruflo-personal/hooks/tile-first-router.mjs';
const SOURCE_DIR = '/run/media/bobinzuks/82ae4087-99fd-4170-b394-3f47ac90dca1/projects/ruflo-personal/projects/tile-router-project/source-python/thefuck/rules';
const RULES_OUT = './src/rules';
const TIMEOUT_MS = 8000;

const { route } = await import(ROUTER);
const files = await readdir(SOURCE_DIR);
let tileHits = 0, deepseekHits = 0, skipped = 0;

for (const f of files) {
  if (!f.endsWith('.py') || f === '__init__.py') continue;
  const ruleName = f.replace('.py', '');
  const outPath = `${RULES_OUT}/${ruleName}.rs`;

  // Skip already written
  try { await access(outPath); console.log(`✓ ${ruleName} exists`); continue; } catch {}

  try {
    const result = await Promise.race([
      route(`${ruleName} rule convert to rust`),
      new Promise((_, rej) => setTimeout(() => rej(new Error('timeout')), TIMEOUT_MS))
    ]);
    if (result?.answer) {
      await writeFile(outPath, result.answer);
      if (result.source === 'tile-code') {
        console.log(`⚡ ${ruleName} → TILE`);
        tileHits++;
      } else {
        console.log(`💰 ${ruleName} → ${result.source}`);
        deepseekHits++;
      }
    }
  } catch(e) {
    console.log(`⏭ ${ruleName} skipped (${e.message})`);
    skipped++;
  }
}

console.log(`\n⚡ ${tileHits} tiles  💰 ${deepseekHits} deepseek  ⏭ ${skipped} skipped`);
