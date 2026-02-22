#!/bin/bash
#
# Script to build and publish Media Sessions documentation to GitHub Pages
#
# Usage:
#   ./publish-docs.sh          # Build and deploy
#   ./publish-docs.sh --check  # Check mdBook installation
#   ./publish-docs.sh --clean  # Clean build artifacts
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if mdBook is installed
check_mdbook() {
    log_info "Checking mdBook installation..."
    
    if command -v mdbook &> /dev/null; then
        log_success "mdBook is installed: $(mdbook --version)"
        return 0
    else
        log_error "mdBook is not installed"
        echo ""
        echo "Install mdBook using one of these commands:"
        echo "  cargo install mdbook"
        echo "  winget install Rust.mdbook    # Windows"
        echo "  brew install mdbook           # macOS"
        echo ""
        return 1
    fi
}

# Build documentation
build_docs() {
    log_info "Building documentation..."
    
    cd docs
    
    if [ ! -f "book.toml" ]; then
        log_error "book.toml not found in docs directory"
        return 1
    fi
    
    mdbook build
    
    if [ -d "book" ]; then
        log_success "Documentation built successfully!"
        log_info "Output directory: docs/book"
        return 0
    else
        log_error "Build failed - book directory not created"
        return 1
    fi
}

# Serve documentation locally
serve_docs() {
    log_info "Starting mdBook server..."
    
    cd docs
    mdbook serve --open
}

# Clean build artifacts
clean_docs() {
    log_info "Cleaning build artifacts..."
    
    if [ -d "docs/book" ]; then
        rm -rf docs/book
        log_success "Cleaned docs/book directory"
    else
        log_info "No build artifacts to clean"
    fi
}

# Deploy to GitHub Pages using git subtree
deploy_to_pages() {
    log_info "Deploying to GitHub Pages..."
    
    # Check if we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        log_error "Not a git repository"
        return 1
    fi
    
    # Check if docs/book exists
    if [ ! -d "docs/book" ]; then
        log_info "Building documentation before deploy..."
        build_docs || return 1
    fi
    
    # Create .nojekyll file if it doesn't exist
    touch docs/book/.nojekyll
    
    log_info "Pushing to gh-pages branch..."
    
    # Using git subtree push
    git subtree push --prefix docs/book origin gh-pages
    
    if [ $? -eq 0 ]; then
        log_success "Documentation deployed to GitHub Pages!"
        log_info "View at: https://krosovok52.github.io/media-sessions/"
        return 0
    else
        log_error "Failed to push to gh-pages"
        return 1
    fi
}

# Deploy using GitHub Actions (recommended)
deploy_via_actions() {
    log_info "Documentation will be auto-deployed via GitHub Actions"
    log_info "Simply push your changes to the main branch:"
    echo ""
    echo "  git add docs/"
    echo "  git commit -m 'docs: update documentation'"
    echo "  git push"
    echo ""
    log_info "GitHub Actions will automatically build and deploy to GitHub Pages"
}

# Show help
show_help() {
    echo "Media Sessions Documentation Publisher"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  build      Build documentation (default)"
    echo "  serve      Build and serve locally with auto-reload"
    echo "  deploy     Deploy to GitHub Pages (git subtree)"
    echo "  push       Push to main branch (triggers GitHub Actions deploy)"
    echo "  clean      Clean build artifacts"
    echo "  check      Check mdBook installation"
    echo "  help       Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 build           # Build documentation"
    echo "  $0 serve           # Serve locally"
    echo "  $0 deploy          # Deploy to gh-pages branch"
    echo "  $0 push            # Push to main (auto-deploy via Actions)"
    echo "  $0 clean && $0 build  # Clean rebuild"
    echo ""
}

# Main script
main() {
    case "${1:-build}" in
        build)
            check_mdbook && build_docs
            ;;
        serve)
            check_mdbook && serve_docs
            ;;
        deploy)
            check_mdbook && deploy_to_pages
            ;;
        push)
            log_info "Pushing to main branch..."
            git add docs/
            git commit -m "docs: update documentation" || log_warning "No changes to commit"
            git push
            deploy_via_actions
            ;;
        clean)
            clean_docs
            ;;
        check)
            check_mdbook
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            log_error "Unknown command: $1"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
