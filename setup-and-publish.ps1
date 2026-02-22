# setup-and-publish.ps1 - Script for publishing documentation to GitHub Pages
# Usage: .\setup-and-publish.ps1

[CmdletBinding()]
param(
    [switch]$OpenBrowser
)

function Write-Header {
    param([string]$Text)
    Write-Host ""
    Write-Host "==================================================" -ForegroundColor Cyan
    Write-Host $Text -ForegroundColor Cyan
    Write-Host "==================================================" -ForegroundColor Cyan
    Write-Host ""
}

function Write-Success {
    param([string]$Text)
    Write-Host "[OK] $Text" -ForegroundColor Green
}

function Write-Error-Custom {
    param([string]$Text)
    Write-Host "[ERROR] $Text" -ForegroundColor Red
}

function Write-Warning-Custom {
    param([string]$Text)
    Write-Host "[WARN] $Text" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "   Media Sessions Documentation Publisher" -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""

Write-Header "1. Checking dependencies"

if (!(Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Error-Custom "git not found. Install from: https://git-scm.com/"
    exit 1
}
Write-Success "git found: $(git --version)"

if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error-Custom "cargo not found. Install from: https://rustup.rs/"
    exit 1
}
Write-Success "cargo found: $(cargo --version)"

if (!(Get-Command mdbook -ErrorAction SilentlyContinue)) {
    Write-Warning-Custom "mdbook not found. Installing..."
    cargo install mdbook
    Write-Success "mdbook installed"
} else {
    Write-Success "mdbook found: $(mdbook --version)"
}

Write-Header "2. Checking repository"

if (!(Test-Path ".git")) {
    Write-Error-Custom "Not a git repository"
    exit 1
}
Write-Success "Git repository found"

$originRemote = git remote get-url origin 2>$null
if (!$originRemote) {
    Write-Error-Custom "origin remote not found"
    exit 1
}
Write-Success "Remote: $originRemote"

Write-Header "3. Building documentation"

$docsDir = Join-Path $PSScriptRoot "docs"
Set-Location $docsDir

if (!(Test-Path "book.toml")) {
    Write-Error-Custom "book.toml not found"
    exit 1
}

Write-Success "Building with mdbook..."
mdbook build

if (!(Test-Path "book")) {
    Write-Error-Custom "book/ directory not created"
    exit 1
}

Write-Success "Documentation built in docs/book/"
Set-Location $PSScriptRoot

Write-Header "4. Setting up GitHub Pages"

$ghPagesExists = git show-ref --verify --quiet refs/heads/gh-pages
if ($ghPagesExists) {
    Write-Success "gh-pages branch exists"
} else {
    Write-Warning-Custom "gh-pages branch does not exist. Creating..."
    git checkout --orphan gh-pages
    git reset --hard
    git commit --allow-empty -m "Initial commit for GitHub Pages"
    git checkout main
    Write-Success "gh-pages branch created"
}

Write-Header "5. Publishing documentation"

$tempBranch = "gh-pages-temp-$PID"
Write-Success "Creating temp branch: $tempBranch"

$existingWorktree = git worktree list | Select-String $tempBranch
if ($existingWorktree) {
    git worktree remove -f $tempBranch 2>$null
}

git worktree add -f "$tempBranch" gh-pages 2>$null
if ($LASTEXITCODE -ne 0) {
    git checkout --orphan "$tempBranch"
    git reset --hard
}

$tempPath = Join-Path $PSScriptRoot $tempBranch
Write-Success "Copying documentation files..."
Get-ChildItem -Path $tempPath -Recurse | Remove-Item -Force -Recurse
Copy-Item -Path (Join-Path $docsDir "book\*") -Destination $tempPath -Recurse -Force

New-Item -ItemType File -Path (Join-Path $tempPath ".nojekyll") -Force | Out-Null

Set-Location $tempPath
git add .

$changes = git status --porcelain
if (!$changes) {
    Write-Warning-Custom "No changes to publish"
    Set-Location $PSScriptRoot
    git worktree remove -f $tempBranch 2>$null
    Write-Success "Nothing to publish"
    exit 0
}

git commit -m "docs: update documentation ($(Get-Date -Format 'yyyy-MM-dd HH:mm'))"

Write-Success "Pushing to GitHub..."
git push origin "$tempBranch`:gh-pages" --force

Set-Location $PSScriptRoot
git worktree remove -f $tempBranch 2>$null

Write-Header "Publication Complete!"

Write-Host ""
Write-Success "Documentation successfully published!"
Write-Host ""
Write-Host "Documentation URL:" -ForegroundColor Cyan
Write-Host "   https://krosovok52.github.io/media-sessions/"
Write-Host ""
Write-Host "Branch: gh-pages" -ForegroundColor Cyan
Write-Host "Publishing time: Usually 1-2 minutes" -ForegroundColor Cyan
Write-Host ""
Write-Host "Note: If documentation is not available, check:" -ForegroundColor Yellow
Write-Host "   GitHub -> Settings -> Pages -> Source: gh-pages branch"
Write-Host ""

if ($OpenBrowser) {
    Start-Process "https://krosovok52.github.io/media-sessions/"
} else {
    $response = Read-Host "Open documentation in browser? (y/n)"
    if ($response -eq "y" -or $response -eq "Y") {
        Start-Process "https://krosovok52.github.io/media-sessions/"
    }
}

Write-Success "Done!"
Write-Host ""
