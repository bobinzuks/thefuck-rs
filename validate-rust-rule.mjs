export function validateRustRule(code, ruleName) {
  const errors = [];
  const pascal = ruleName.split('_').map(w => w[0].toUpperCase() + w.slice(1)).join('');

  // Must-have checks
  if (!code.includes('impl Rule')) errors.push('missing impl Rule');
  if (!code.includes('fn matches')) errors.push('missing fn matches');
  if (!code.includes('fn fix')) errors.push('missing fn fix');
  if (!code.includes(`pub struct ${pascal}`)) errors.push(`wrong struct name, expected ${pascal}`);

  // Must-not-have checks
  if (code.includes('.script_parts')) errors.push('uses script_parts — use cmd.text');
  if (code.includes('cmd.script')) errors.push('uses cmd.script — use cmd.text');
  if (code.includes('command.script')) errors.push('uses command.script — use cmd.text');
  if (code.includes('Vec<String>')) errors.push('uses Vec<String> — use String');
  if (code.includes('.iter().')) errors.push('calls .iter() on String');
  if (code.includes('[0]') && !code.includes('as_bytes')) errors.push('indexes String like array');
  if (code.includes('redefine') || code.includes('pub struct Command')) errors.push('redefines Command');
  if (code.includes('pub trait Rule')) errors.push('redefines Rule trait');

  return { valid: errors.length === 0, errors };
}
