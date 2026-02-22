@echo off
chcp 65001 >nul
echo.
echo ======================================
echo   Media Sessions Documentation Publisher
echo ======================================
echo.

echo [1/5] Checking dependencies...

where git >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] git not found. Install from https://git-scm.com/
    exit /b 1
)
echo [OK] git found

where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] cargo not found. Install from https://rustup.rs/
    exit /b 1
)
echo [OK] cargo found

where mdbook >nul 2>&1
if %errorlevel% neq 0 (
    echo [WARN] mdbook not found. Installing...
    cargo install mdbook
)
echo [OK] mdbook found

echo.
echo [2/5] Building documentation...
cd /d "%~dp0docs"
mdbook build
cd /d "%~dp0"

echo.
echo [3/5] Checking gh-pages branch...
git show-ref --verify --quiet refs/heads/gh-pages
if %errorlevel% neq 0 (
    echo [WARN] gh-pages branch not found. Creating...
    git checkout --orphan gh-pages
    git reset --hard
    git commit --allow-empty -m "Initial commit for GitHub Pages"
    git checkout main
)

echo.
echo [4/5] Publishing to GitHub...
set TEMP_BRANCH=gh-pages-temp-%random%

git worktree add -f %TEMP_BRANCH% gh-pages 2>nul
if %errorlevel% neq 0 (
    git checkout --orphan %TEMP_BRANCH%
    git reset --hard
)

del /q /s %TEMP_BRANCH%\* 2>nul
xcopy /E /I /Y docs\book\* %TEMP_BRANCH%\ >nul
echo. > %TEMP_BRANCH%\.nojekyll

cd %TEMP_BRANCH%
git add .
git commit -m "docs: publish documentation"
git push origin gh-pages --force
cd ..

echo.
echo [5/5] Cleaning up...
git worktree remove -f %TEMP_BRANCH% 2>nul

echo.
echo ======================================
echo   Publication Complete!
echo ======================================
echo.
echo URL: https://krosovok52.github.io/media-sessions/
echo.
pause
