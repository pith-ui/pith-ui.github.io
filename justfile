# Serve the site locally (dev mode with hot reload)
serve:
    trunk serve

# Build the site in release mode
build:
    trunk build --release

# Run Lighthouse accessibility audit against all pages (reads unlighthouse.config.ts)
audit: _ensure-serving
    npx unlighthouse

# Run a quick single-page Lighthouse audit
audit-page url="http://localhost:8080":
    npx lighthouse "{{url}}" --only-categories=accessibility --output=html --output-path=.results/lighthouse-report.html --chrome-flags="--headless=new"
    @echo "Report saved to .results/lighthouse-report.html"

# Verify the dev server is reachable before auditing
[private]
_ensure-serving:
    #!/usr/bin/env sh
    if ! curl -s -o /dev/null -w '' http://localhost:8080 2>/dev/null; then
        echo "Error: dev server not running. Start it first with: just serve" >&2
        exit 1
    fi
