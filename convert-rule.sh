#!/usr/bin/env bash
# Converts a Python thefuck rule to Rust via DeepSeek

export DEEPSEEK_API_KEY="sk-e3d9aba0ad964f168095f5f01288b706"
API_URL="https://api.deepseek.com/v1/chat/completions"
MODEL="deepseek-chat"

RULE_FILE="$1"
RULE_NAME=$(basename "$RULE_FILE" .py)

if [ -z "$RULE_FILE" ]; then
    echo "Usage: ./convert-rule.sh source-python/thefuck/rules/git_push.py"
    exit 1
fi

PYTHON_SOURCE=$(cat "$RULE_FILE")

PROMPT="You are converting Python thefuck rules to Rust.

Here is the Rust Rule trait already defined:
pub struct Command {
    pub text: String,
    pub output: String,
    pub exit_code: i32,
}
pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn matches(&self, cmd: &Command) -> bool;
    fn fix(&self, cmd: &Command) -> String;
    fn priority(&self) -> u8 { 50 }
}

Convert this Python rule to Rust:

$PYTHON_SOURCE

Output ONLY a single Rust file with:
- use super::{Command, Rule};
- A pub struct named $(echo $RULE_NAME | sed 's/_\([a-z]\)/\U\1/g' | sed 's/^\([a-z]\)/\U\1/')
- impl Rule for that struct
- matches() using cmd.text and cmd.output string checks
- fix() returning the corrected command string
- No main(), no mod declarations, just the struct and impl

Output in a single rust code block."

ESCAPED=$(jq -Rns --arg p "$PROMPT" '$p')

echo "🔄 Converting $RULE_NAME via DeepSeek..."

RESPONSE=$(curl -s "$API_URL" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $DEEPSEEK_API_KEY" \
  -d "{
    \"model\": \"$MODEL\",
    \"messages\": [{\"role\": \"user\", \"content\": $ESCAPED}],
    \"temperature\": 0.1,
    \"max_tokens\": 2000
  }")

CONTENT=$(echo "$RESPONSE" | jq -r '.choices[0].message.content // empty')

if [ -z "$CONTENT" ]; then
    echo "❌ Failed: $(echo "$RESPONSE" | head -5)"
    exit 1
fi

# Extract rust block
echo "$CONTENT" | awk '/```rust/{flag=1; next} /```/{if(flag) flag=0; next} flag' > "src/rules/${RULE_NAME}.rs"

echo "✅ Written to src/rules/${RULE_NAME}.rs"
cat "src/rules/${RULE_NAME}.rs"
