# Simple publish script - just run it!
# .\publish-simple.ps1

Write-Host "Building documentation..." -ForegroundColor Cyan
cd $PSScriptRoot\docs
mdbook build
cd $PSScriptRoot

Write-Host "Publishing to GitHub..." -ForegroundColor Cyan

# Simple publish without worktree
git checkout gh-pages 2>$null
if ($LASTEXITCODE -ne 0) {
    git checkout --orphan gh-pages
    git reset --hard
}

git rm -rf . 2>$null
Copy-Item -Path "docs\book\*" -Destination "." -Recurse -Force
git add .
git commit -m "docs: publish"
git push origin gh-pages --force
git checkout main

Write-Host "Done! Visit: https://krosovok52.github.io/media-sessions/" -ForegroundColor Green
