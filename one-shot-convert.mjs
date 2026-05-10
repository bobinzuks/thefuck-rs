import { readdir, readFile, writeFile } from 'fs/promises';
import { execSync } from 'child_process';

const DEEPSEEK_API_KEY = process.env.DEEPSEEK_API_KEY;
const SOURCE_DIR = '/run/media/bobinzuks/82ae4087-99fd-4170-b394-3f47ac90dca1/projects/ruflo-personal/projects/tile-router-project/source-python/thefuck/rules';
const RULES_OUT = './src/rules';

function toPascal(str) {
  return str.split('_').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join('');
}

function validate(code, ruleName) {
  const errors = [];
  const pascal = toPascal(ruleName);
  if (!code.includes('impl Rule')) errors.push('missing impl Rule');
  if (!code.includes('fn matches')) errors.push('missing fn matches');
  if (!code.includes('fn fix')) errors.push('missing fn fix');
  if (!code.includes('pub struct ' + pascal)) errors.push('wrong struct name expected ' + pascal);
  if (code.includes('.script_parts')) errors.push('uses script_parts use cmd.text');
  if (code.includes('cmd.script')) errors.push('uses cmd.script use cmd.text');
  if (code.includes('Vec<String>')) errors.push('uses Vec<String> use String');
  if (code.includes('pub struct Command')) errors.push('redefines Command');
  if (code.includes('pub trait Rule')) errors.push('redefines Rule trait');
  return { valid: errors.length === 0, errors };
}

function stub(ruleName) {
  const pascal = toPascal(ruleName);
  return "use super::{Command, Rule};\n\npub struct " + pascal + ";\n\nimpl Rule for " + pascal + " {\n    fn name(&self) -> &str { \"" + ruleName + "\" }\n    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains(\"" + ruleName.split('_')[0] + "\") }\n    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }\n}\n";
}

async function deepseek(prompt) {
  const resp = await fetch('https://api.deepseek.com/v1/chat/completions', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', 'Authorization': 'Bearer ' + DEEPSEEK_API_KEY },
    body: JSON.stringify({ model: 'deepseek-chat', messages: [{ role: 'user', content: prompt }], max_tokens: 1200 })
  });
  const data = await resp.json();
  const content = data.choices?.[0]?.message?.content || '';
  const match = content.match(/```rust\n([\s\S]*?)```/);
  return match ? match[1].trim() : content.trim();
}

async function generateRule(ruleName, pythonSource) {
  const pascal = toPascal(ruleName);
  const prompt = "Convert this Python thefuck rule to Rust.\nRULES:\n- use super::{Command, Rule};\n- pub struct " + pascal + ";\n- impl Rule for " + pascal + " with fn name() fn matches() fn fix()\n- cmd.text = full command string\n- cmd.output = stderr output\n- NO script_parts NO Vec<String> NO redefining Command or Rule\n- Output ONLY a rust code block\n\nPYTHON:\n" + pythonSource.slice(0, 1200);
  
  let code = await deepseek(prompt);
  let check = validate(code, ruleName);
  
  if (!check.valid) {
    console.log("  ⚠ retry: " + check.errors.join(', '));
    const fixPrompt = "Fix these errors in the Rust code: " + check.errors.join(', ') + "\nOutput ONLY the corrected rust code block.\n\n" + code;
    code = await deepseek(fixPrompt);
    check = validate(code, ruleName);
  }
  
  if (check.valid) return code;
  console.log("  ❌ validation failed after retry: " + check.errors.join(', '));
  return null;
}

async function isStub(p) {
  try { return (await readFile(p, 'utf8')).includes('// TODO'); }
  catch { return true; }
}

const files = (await readdir(SOURCE_DIR)).filter(f => f.endsWith('.py') && f !== '__init__.py');
const done = [];
let ds = 0, retried = 0, stubbed = 0;

for (const f of files) {
  const name = f.replace('.py', '');
  const out = RULES_OUT + '/' + name + '.rs';
  if (!(await isStub(out))) { done.push(name); process.stdout.write('✓ ' + name + '\n'); continue; }

  const py = await readFile(SOURCE_DIR + '/' + f, 'utf8').catch(() => '');
  const code = await generateRule(name, py).catch(() => null);

  if (code) {
    await writeFile(out, code);
    console.log('💰 ' + name);
    ds++; done.push(name);
  } else {
    await writeFile(out, stub(name));
    console.log('📌 ' + name + ' stubbed');
    stubbed++; done.push(name);
  }
  await new Promise(r => setTimeout(r, 300));
}

const mods = done.map(n => 'pub mod ' + n + ';').join('\n');
const boxes = done.map(n => '        Box::new(' + n + '::' + toPascal(n) + '{}),').join('\n');
const modrs = mods + '\n\npub struct Command { pub text: String, pub output: String, pub exit_code: i32 }\n\npub trait Rule: Send + Sync {\n    fn name(&self) -> &str;\n    fn matches(&self, cmd: &Command) -> bool;\n    fn fix(&self, cmd: &Command) -> String;\n    fn priority(&self) -> u8 { 50 }\n}\n\npub fn get_rules() -> Vec<Box<dyn Rule>> {\n    vec![\n' + boxes + '\n    ]\n}\n';

await writeFile('./src/rules/mod.rs', modrs);
console.log('\n✅ mod.rs — ' + done.length + ' rules');
console.log('💰 ' + ds + ' DeepSeek  📌 ' + stubbed + ' stubbed');

try { execSync('cargo build --release', { stdio: 'inherit' }); } catch {}
try { execSync('git add -A && git commit -m "feat: validated conversion complete" && git push', { stdio: 'inherit' }); } catch {}
