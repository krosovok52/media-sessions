# 📚 How to Build & Deploy Documentation

This guide explains how to build and deploy the Media Sessions documentation.

---

## 🛠️ Prerequisites

### Option 1: mdBook (Recommended)

**Install mdBook:**

```bash
# Using cargo
cargo install mdbook

# Using winget (Windows)
winget install Rust.mdbook

# Using homebrew (macOS)
brew install mdbook
```

**Install plugins (optional):**

```bash
cargo install mdbook-linkcheck
cargo install mdbook-alerts
```

---

## 📖 Building Documentation

### Local Build

```bash
# Navigate to docs directory
cd docs

# Build the book
mdbook build

# Output will be in docs/book/
```

### Serve Locally

```bash
# Start local server
mdbook serve

# Open in browser
# http://localhost:3000
```

### Watch Mode

```bash
# Auto-rebuild on changes
mdbook serve --watch
```

---

## 🌐 Deploy to GitHub Pages

### Step 1: Enable GitHub Actions

Create `.github/workflows/deploy-docs.yml`:

```yaml
name: Deploy Documentation

on:
  push:
    branches:
      - main
    paths:
      - 'docs/**'
      - 'book.toml'

jobs:
  deploy:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    concurrency:
      group: ${{ github.workflow }}-${{ github.ref }}
    
    steps:
      - uses: actions/checkout@v4

      - name: Install mdBook
        run: cargo install mdbook

      - name: Build documentation
        run: |
          cd docs
          mdbook build

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs/book
          publish_branch: gh-pages
```

### Step 2: Configure GitHub Pages

1. Go to **Settings** → **Pages**
2. Select **Source**: Deploy from a branch
3. Select **Branch**: `gh-pages`
4. Click **Save**

### Step 3: Access Documentation

Your documentation will be available at:

```
https://<username>.github.io/<repo>/
```

For example:
```
https://krosovok52.github.io/media-sessions/
```

---

## 🎨 Customization

### Change Theme

Edit `book.toml`:

```toml
[output.html]
default-theme = "navy"  # Options: light, navy, ayu, coal
preferred-dark-theme = "navy"
```

### Add Custom CSS

Create `docs/src/custom.css`:

```css
/* Custom styles */
.sidebar {
    background-color: #f5f5f5;
}

.page-footer {
    text-align: center;
    padding: 20px;
}
```

Update `book.toml`:

```toml
[output.html]
additional-css = ["custom.css"]
```

### Add Custom JavaScript

Create `docs/src/custom.js`:

```javascript
// Add analytics, custom behaviors, etc.
console.log('Documentation loaded');
```

Update `book.toml`:

```toml
[output.html]
additional-js = ["custom.js"]
```

---

## 🔄 Multi-language Setup

The documentation structure supports multiple languages:

```
docs/src/
├── SUMMARY.md          # Root summary (language selector)
├── ru/
│   ├── SUMMARY.md      # Russian table of contents
│   └── README.md       # Russian home page
└── en/
    ├── SUMMARY.md      # English table of contents
    └── README.md       # English home page
```

### Language Switcher

The root `SUMMARY.md` includes links to both languages:

```markdown
## 📚 Language Selection

- **[🇷🇺 Русская версия](ru/)** — полная документация на русском
- **[🇬🇧 English version](en/)** — full documentation in English
```

---

## 📊 Alternative: Docusaurus

If you prefer React-based documentation:

### Install Docusaurus

```bash
npx create-docusaurus@latest my-website classic
cd my-website
```

### Configure Multi-language

Edit `docusaurus.config.js`:

```javascript
module.exports = {
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'ru'],
    localeConfigs: {
      en: { label: 'English' },
      ru: { label: 'Русский' },
    },
  },
};
```

### Deploy

```bash
npm run build
npm run deploy
```

---

## 📊 Alternative: VitePress

Fast, Vue-based documentation:

### Install

```bash
npm add -D vitepress
```

### Configure

Create `docs/.vitepress/config.mjs`:

```javascript
export default {
  title: 'Media Sessions',
  description: 'Cross-platform media session control',
  locales: {
    root: {
      label: 'English',
      lang: 'en'
    },
    ru: {
      label: 'Русский',
      lang: 'ru',
      link: '/ru/'
    }
  }
}
```

### Build

```bash
npx vitepress build docs
```

---

## 🎯 Comparison

| Feature | mdBook | Docusaurus | VitePress | GitBook |
|---------|--------|------------|-----------|---------|
| **Setup Time** | 5 min | 30 min | 15 min | 2 min |
| **Multi-language** | ✅ Manual | ✅ Built-in | ✅ Built-in | ✅ Built-in |
| **Search** | ✅ Built-in | ✅ Plugin | ✅ Built-in | ✅ Built-in |
| **Customization** | Medium | High | High | Low |
| **Hosting** | Any | Any | Any | GitBook |
| **Best For** | Rust projects | Large docs | Modern docs | Quick start |

---

## ✅ Recommended Setup

For **Media Sessions** (Rust project), I recommend **mdBook**:

1. ✅ Native Rust ecosystem integration
2. ✅ Simple setup
3. ✅ Free GitHub Pages hosting
4. ✅ Fast build times
5. ✅ Good enough multi-language support

---

## 🚀 Quick Deploy Script

Create `deploy-docs.sh`:

```bash
#!/bin/bash

# Build documentation
cd docs
mdbook build

# Deploy to GitHub Pages
git add docs/book
git commit -m "docs: update documentation"
git push origin gh-pages:gh-pages

echo "✅ Documentation deployed!"
```

Make executable:

```bash
chmod +x deploy-docs.sh
```

Run:

```bash
./deploy-docs.sh
```

---

## 📚 Resources

- **mdBook Docs:** https://rust-lang.github.io/mdBook/
- **GitHub Pages:** https://pages.github.com/
- **Docusaurus:** https://docusaurus.io/
- **VitePress:** https://vitepress.dev/

---

*For questions: [@krosov_ok](https://t.me/krosov_ok)*
