#!/usr/bin/env bash
export DEEPSEEK_API_KEY="sk-e3d9aba0ad964f168095f5f01288b706"

ROUTER="/run/media/bobinzuks/82ae4087-99fd-4170-b394-3f47ac90dca1/projects/old-ruflo-personal/hooks/tile-first-router.mjs"
SOURCE_DIR="/run/media/bobinzuks/82ae4087-99fd-4170-b394-3f47ac90dca1/projects/ruflo-personal/projects/tile-router-project/source-python/thefuck/rules"
RULES_OUT="./src/rules"

TILE_HITS=0
DEEPSEEK_HITS=0
FAILED=0

for f in "$SOURCE_DIR"/*.py; do
    [ -f "$f" ] || continue
    RULE_NAME=$(basename "$f" .py)
    [ "$RULE_NAME" = "__init__" ] && continue

    RESULT=$(node "$ROUTER" "${RULE_NAME} rule convert to rust" 2>/dev/null)
    
    # Extract source line
    SOURCE=$(echo "$RESULT" | grep "^Source:" | awk '{print $2}')
    
    # Extract answer — everything after "Answer:" line
    ANSWER=$(echo "$RESULT" | awk '/^Answer:/{found=1; next} found{print}')

    if [ -n "$ANSWER" ]; then
        echo "$ANSWER" > "$RULES_OUT/${RULE_NAME}.rs"
        if [ "$SOURCE" = "tile-code" ]; then
            echo "⚡ $RULE_NAME → TILE (0 tokens)"
            TILE_HITS=$((TILE_HITS + 1))
        else
            echo "💰 $RULE_NAME → $SOURCE"
            DEEPSEEK_HITS=$((DEEPSEEK_HITS + 1))
        fi
    else
        echo "❌ $RULE_NAME failed"
        FAILED=$((FAILED + 1))
    fi
done

echo ""
echo "⚡ Tile hits:     $TILE_HITS (free)"
echo "💰 DeepSeek hits: $DEEPSEEK_HITS (paid)"
echo "❌ Failed:        $FAILED"
