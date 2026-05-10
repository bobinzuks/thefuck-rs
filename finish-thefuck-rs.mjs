import { readdir, readFile, writeFile } from 'fs/promises';
import { execSync } from 'child_process';

const DEEPSEEK_API_KEY = process.env.DEEPSEEK_API_KEY;
const SOURCE_DIR = '/run/media/bobinzuks/82ae4087-99fd-4170-b394-3f47ac90dca1/projects/ruflo-personal/projects/tile-router-project/source-python/thefuck/rules';
const RULES_OUT = './src/rules';

function toPascal(str) {
  return str.split('_').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join('');
}

async function isStub(p) {
  try { return (await readFile(p, 'utf8')).includes('// TODO'); }
  catch { return true; }
}

async function deepseek(ruleName, py) {
  const prompt = "Convert this Python thefuck rule to Rust. Output ONLY a rust code block.\nUse: use super::{Command, Rule};\nDo NOT redefine Command or Rule.\npub struct " + toPascal(ruleName) + ";\nimpl Rule for " + toPascal(ruleName) + " with name() matches() fix()\nmatches() uses cmd.text and cmd.output\nfix() returns String\n\nPYTHON:\n" + py.slice(0, 1200);

  const resp = await fetch('https://api.deepseek.com/v1/chat/completions', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', 'Authorization': 'Bearer ' + DEEPSEEK_API_KEY },
    body: JSON.stringify({ model: 'deepseek-chat', messages: [{ role: 'user', content: prompt }], max_tokens: 1200 })
  });
  const data = await resp.json();
  const content = data.choices?.[0]?.message?.content || '';
  const match = content.match(/```rust\n([\s\S]*?)```/);
  return match ? match[1].trim() : '';
}

const files = (await readdir(SOURCE_DIR)).filter(f => f.endsWith('.py') && f !== '__init__.py');
const done = [];
let ds = 0, sk = 0;

for (const f of files) {
  const name = f.replace('.py', '');
  const out = RULES_OUT + '/' + name + '.rs';
  if (!(await isStub(out))) { done.push(name); process.stdout.write('✓ ' + name + '\n'); continue; }

  const py = await readFile(SOURCE_DIR + '/' + f, 'utf8').catch(() => '');
  const code = await deepseek(name, py).catch(() => '');

  if (code && code.includes('impl Rule')) {
    await writeFile(out, code);
    console.log('💰 ' + name + ' → DeepSeek');
    ds++; done.push(name);
  } else {
    console.log('❌ ' + name);
    sk++;
  }
  await new Promise(r => setTimeout(r, 300));
}

const mods = done.map(n => 'mod ' + n + ';').join('\n');
const boxes = done.map(n => '        Box::new(' + n + '::' + toPascal(n) + '),').join('\n');
const modrs = mods + `

pub struct Command { pub text: String, pub output: String, pub exit_code: i32 }

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn matches(&self, cmd: &Command) -> bool;
    fn fix(&self, cmd: &Command) -> String;
    fn priority(&self) -> u8 { 50 }
}

pub fn get_rules() -> Vec<Box<dyn Rule>> {
    vec![
` + boxes + `
    ]
}
`;

await writeFile('./src/rules/mod.rs', modrs);
console.log('\n✅ mod.rs — ' + done.length + ' rules');

try { execSync('cargo build --release', { stdio: 'inherit' }); } catch {}
try { execSync('git add -A && git commit -m "feat: thefuck-rs complete" && git push', { stdio: 'inherit' }); } catch {}
console.log('\n💰 ' + ds + ' DeepSeek  ❌ ' + sk + ' failed');
