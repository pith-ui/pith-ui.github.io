#!/usr/bin/env bash
# Downloads base-ui component markdown files for reference.
# Usage: bash .claude/scripts/fetch-base-ui-docs.sh

set -euo pipefail

OUT_DIR=".claude/reference/base-ui"
BASE_URL="https://base-ui.com/react/components"

COMPONENTS=(
    accordion
    alert-dialog
    autocomplete
    avatar
    button
    calendar
    checkbox
    checkbox-group
    collapsible
    combobox
    context-menu
    dialog
    drawer
    field
    fieldset
    form
    input
    menu
    menubar
    meter
    navigation-menu
    number-field
    popover
    preview-card
    progress
    radio
    scroll-area
    select
    separator
    slider
    switch
    tabs
    toast
    toggle
    toggle-group
    toolbar
    tooltip
)

mkdir -p "$OUT_DIR"

for name in "${COMPONENTS[@]}"; do
    url="${BASE_URL}/${name}.md"
    dest="${OUT_DIR}/${name}.md"
    printf "%-24s" "$name"
    status=$(curl -s -o "$dest" -w "%{http_code}" "$url")
    if [ "$status" = "200" ]; then
        echo "ok"
    else
        echo "HTTP $status (skipped)"
        rm -f "$dest"
    fi
done

echo ""
echo "Done. Files saved to $OUT_DIR/"
ls -1 "$OUT_DIR"/*.md 2>/dev/null | wc -l | xargs printf "%d files downloaded\n"
