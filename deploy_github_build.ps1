# Script de déploiement - Fix Docker Compose pour utiliser le code GitHub
Write-Host "🔧 Fix Docker Compose - Utiliser le code GitHub au lieu du Docker Hub" -ForegroundColor Cyan
Write-Host ""

# Navigate to project directory
Set-Location "C:\Users\amine\Antigravity-Manager"

# Git add
Write-Host "📦 Staging des fichiers..." -ForegroundColor Yellow
git add docker/docker-compose.yml

# Git commit
Write-Host ""
Write-Host "💾 Commit..." -ForegroundColor Yellow
git commit -m "fix: Use local build instead of Docker Hub image in docker-compose

Change image from lbjlaq/antigravity-manager:latest to antigravity-manager:local
This ensures Coolify builds from GitHub repo (Aminetwiti/Antigravity-Manager)
instead of pulling old image from Docker Hub without OpenAI button.

Build process:
- Builds from source code in GitHub
- Includes latest OpenAI account features
- Respects CACHEBUST for frontend rebuilds"

# Git push
Write-Host ""
Write-Host "🚀 Push vers GitHub..." -ForegroundColor Yellow
git push origin main

Write-Host ""
Write-Host "✅ Déploiement terminé!" -ForegroundColor Green
Write-Host ""
Write-Host "📌 Dans Coolify:" -ForegroundColor Cyan
Write-Host "   1. Redéployer (il va maintenant BUILD au lieu de PULL)" -ForegroundColor White
Write-Host "   2. Attendre 10-15 minutes pour le build complet" -ForegroundColor White
Write-Host "   3. Chercher dans les logs: 'npm install', 'vite build', 'cargo build'" -ForegroundColor White
Write-Host ""
Write-Host "🎯 Résultat: Bouton [🤖 Add OpenAI Account] visible dans /accounts" -ForegroundColor Green
