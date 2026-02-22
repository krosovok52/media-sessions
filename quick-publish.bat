@echo off
chcp 65001 >nul
echo.
echo ======================================
echo   Quick Publish to GitHub
echo ======================================
echo.

echo [1/3] Adding files...
git add .

echo [2/3] Committing...
git commit -m "docs: publish documentation"

echo [3/3] Pushing to GitHub...
git push origin main

echo.
echo ======================================
echo   Done!
echo ======================================
echo.
echo GitHub Actions will build and publish documentation.
echo Visit: https://github.com/krosovok52/media-sessions/actions
echo.
echo Documentation URL: https://krosovok52.github.io/media-sessions/
echo.
pause
