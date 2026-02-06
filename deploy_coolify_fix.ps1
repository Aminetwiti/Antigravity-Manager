# Script de déploiement - Fix Coolify Docker Build
Write-Host "🔧 Déploiement du fix Coolify Docker Build" -ForegroundColor Cyan
Write-Host ""

# Navigate to project directory
Set-Location "C:\Users\amine\Antigravity-Manager"

# Add modified files
Write-Host "📦 Staging des fichiers..." -ForegroundColor Yellow
git add docker/docker-compose.yml
git add docker/.env.example
git add docker/README.md
git add COOLIFY_DEPLOYMENT_GUIDE.md

# Commit the changes
Write-Host ""
Write-Host "💾 Commit des changements..." -ForegroundColor Yellow
git commit -m "fix: Configure docker-compose to build locally for Coolify deployment

BREAKING CHANGE: docker-compose now builds from source instead of pulling Docker Hub image

- Add build configuration to docker-compose.yml with CACHEBUST support
- Create .env.example with build-time variables
- Update docker/README.md with local build instructions
- Add COOLIFY_DEPLOYMENT_GUIDE.md with complete deployment steps

This fixes the issue where Coolify was using the old Docker Hub image
instead of building the latest code with OpenAI account button."

# Push to GitHub
Write-Host ""
Write-Host "🚀 Push vers GitHub..." -ForegroundColor Yellow
git push origin main

Write-Host ""
Write-Host "✅ Push terminé!" -ForegroundColor Green
Write-Host ""
Write-Host "📌 Prochaines étapes dans Coolify:" -ForegroundColor Cyan
Write-Host "   1. Ouvrir Coolify Dashboard" -ForegroundColor White
Write-Host "   2. Application Settings → Build" -ForegroundColor White
Write-Host "   3. Ajouter variable: CACHEBUST=2" -ForegroundColor White
Write-Host "   4. Redéployer et attendre 10-15 minutes" -ForegroundColor White
Write-Host ""
Write-Host "   Les logs devraient maintenant montrer:" -ForegroundColor Gray
Write-Host "   - npm install" -ForegroundColor Gray
Write-Host "   - Building frontend..." -ForegroundColor Gray
Write-Host "   - Compiling Rust backend..." -ForegroundColor Gray
Write-Host ""
Write-Host "📖 Guide complet: COOLIFY_DEPLOYMENT_GUIDE.md" -ForegroundColor Yellow
