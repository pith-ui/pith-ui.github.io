# Bug Reports

This folder contains detailed bug reports for issues discovered during development of the pith-ui documentation site. Reports here may reference issues in the pith-ui library itself, the docs site, or the interaction between them.

## Writing a bug report

Each bug report is a standalone markdown file. Use `_template.md` as a starting point.

### File naming

`<component>-<short-description>.md` in kebab-case. Examples:
- `accordion-multiple-animation-replay.md`
- `tooltip-positioning-offset.md`
- `dialog-focus-trap-escape.md`

### Required sections

1. **Title** — one-line summary as an H1
2. **Component** — which pith-ui component(s) are affected
3. **Severity** — how impactful: `blocking`, `major`, `minor`, `cosmetic`
4. **Summary** — 2-3 sentence description of the problem
5. **Expected behavior** — what should happen
6. **Actual behavior** — what does happen, with specific observable symptoms
7. **Steps to reproduce** — minimal sequence to trigger the bug
8. **Analysis** — research into the root cause. Include:
   - Relevant source file paths and line numbers
   - The chain of events that leads to the bug
   - Which layer owns the bug (library, docs site, CSS, browser)
9. **Workarounds** — any temporary mitigations currently in use
10. **References** — links to source files, related issues, or prior art

### Guidelines

- **Be specific about what you observed.** "Animation replays" is vague. "The already-open panel's slide-down animation restarts from height:0 when a sibling panel is opened" is actionable.
- **Don't propose fixes.** Describe the problem and your analysis of the cause. The maintainer with deeper context on the library's architecture will determine the right solution pattern.
- **Include file paths and line numbers.** These are snapshots — note the git revision if relevant so the reader can find the exact code.
- **Distinguish symptoms from root cause.** The symptom might be "animation replays" but the root cause might be "effect re-runs when signal value hasn't changed."
- **Note workarounds separately.** If the docs site is using a temporary workaround, describe it so it can be removed once the underlying issue is fixed.
