# Script to build and publish Media Sessions documentation to GitHub Pages
#
# Usage:
#   .\publish-docs.ps1          # Build and deploy
#   .\publish-docs.ps1 -Check   # Check mdBook installation
#   .\publish-docs.ps1 -Clean   # Clean build artifacts
#

param(
    [Parameter(Position=0)]
    [ValidateSet('build', 'serve', 'deploy', 'push', 'clean', 'check', 'help')]
    [string]$Command = 'build'
)

# Functions
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Test-MdBook {
    Write-Info "Checking mdBook installation..."
    
    try {
        $version = mdbook --version 2>&1
        Write-Success "mdBook is installed: $version"
        return $true
    } catch {
        Write-Error "mdBook is not installed"
        Write-Host ""
        Write-Host "Install mdBook using one of these commands:"
        Write-Host "  cargo install mdbook"
        Write-Host "  winget install Rust.mdbook"
        Write-Host ""
        return $false
    }
}

function Build-Docs {
    Write-Info "Building documentation..."
    
    $originalLocation = Get-Location
    Set-Location -Path "$PSScriptRoot\docs"
    
    if (-not (Test-Path "book.toml")) {
        Write-Error "book.toml not found in docs directory"
        Set-Location $originalLocation
        return $false
    }
    
    try {
        mdbook build
        if (Test-Path "book") {
            Write-Success "Documentation built successfully!"
            Write-Info "Output directory: docs/book"
            Set-Location $originalLocation
            return $true
        } else {
            Write-Error "Build failed - book directory not created"
            Set-Location $originalLocation
            return $false
        }
    } catch {
        Write-Error "Build failed: $_"
        Set-Location $originalLocation
        return $false
    }
}

function Serve-Docs {
    Write-Info "Starting mdBook server..."
    
    $originalLocation = Get-Location
    Set-Location -Path "$PSScriptRoot\docs"
    
    try {
        mdbook serve --open
    } catch {
        Write-Error "Serve failed: $_"
    } finally {
        Set-Location $originalLocation
    }
}

function Clean-Docs {
    Write-Info "Cleaning build artifacts..."
    
    if (Test-Path "$PSScriptRoot\docs\book") {
        Remove-Item -Path "$PSScriptRoot\docs\book" -Recurse -Force
        Write-Success "Cleaned docs/book directory"
    } else {
        Write-Info "No build artifacts to clean"
    }
}

function Deploy-ToPages {
    Write-Info "Deploying to GitHub Pages..."
    
    # Check if we're in a git repository
    if (-not (Test-Path ".git")) {
        Write-Error "Not a git repository"
        return $false
    }
    
    # Check if docs/book exists
    if (-not (Test-Path "docs\book")) {
        Write-Info "Building documentation before deploy..."
        if (-not (Build-Docs)) {
            return $false
        }
    }
    
    # Create .nojekyll file
    New-Item -Path "docs\book\.nojekyll" -ItemType File -Force | Out-Null
    
    Write-Info "Pushing to gh-pages branch..."
    
    try {
        git subtree push --prefix docs/book origin gh-pages
        Write-Success "Documentation deployed to GitHub Pages!"
        Write-Info "View at: https://krosovok52.github.io/media-sessions/"
        return $true
    } catch {
        Write-Error "Failed to push to gh-pages: $_"
        return $false
    }
}

function Deploy-ViaActions {
    Write-Info "Documentation will be auto-deployed via GitHub Actions"
    Write-Info "Simply push your changes to the main branch:"
    Write-Host ""
    Write-Host "  git add docs/"
    Write-Host "  git commit -m 'docs: update documentation'"
    Write-Host "  git push"
    Write-Host ""
    Write-Info "GitHub Actions will automatically build and deploy to GitHub Pages"
}

function Show-Help {
    Write-Host "Media Sessions Documentation Publisher"
    Write-Host ""
    Write-Host "Usage: .\publish-docs.ps1 [COMMAND]"
    Write-Host ""
    Write-Host "Commands:"
    Write-Host "  build      Build documentation (default)"
    Write-Host "  serve      Build and serve locally with auto-reload"
    Write-Host "  deploy     Deploy to GitHub Pages (git subtree)"
    Write-Host "  push       Push to main branch (triggers GitHub Actions deploy)"
    Write-Host "  clean      Clean build artifacts"
    Write-Host "  check      Check mdBook installation"
    Write-Host "  help       Show this help message"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\publish-docs.ps1 build           # Build documentation"
    Write-Host "  .\publish-docs.ps1 serve           # Serve locally"
    Write-Host "  .\publish-docs.ps1 deploy          # Deploy to gh-pages branch"
    Write-Host "  .\publish-docs.ps1 push            # Push to main (auto-deploy via Actions)"
    Write-Host "  .\publish-docs.ps1 clean; .\publish-docs.ps1 build  # Clean rebuild"
    Write-Host ""
}

# Main script
switch ($Command) {
    'build' {
        if (Test-MdBook) { Build-Docs }
    }
    'serve' {
        if (Test-MdBook) { Serve-Docs }
    }
    'deploy' {
        if (Test-MdBook) { Deploy-ToPages }
    }
    'push' {
        Write-Info "Pushing to main branch..."
        git add docs/
        $changes = git status --porcelain
        if ($changes) {
            git commit -m "docs: update documentation"
        } else {
            Write-Warning "No changes to commit"
        }
        git push
        Deploy-ViaActions
    }
    'clean' {
        Clean-Docs
    }
    'check' {
        Test-MdBook
    }
    'help' {
        Show-Help
    }
    default {
        Write-Error "Unknown command: $Command"
        Show-Help
        exit 1
    }
}
