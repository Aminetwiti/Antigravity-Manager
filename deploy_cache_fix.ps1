# Script to deploy Docker cache fix and French translations
Write-Host "🔧 Deploying Docker Cache Fix + French Translations" -ForegroundColor Cyan
Write-Host ""

# Navigate to project directory
Set-Location "C:\Users\amine\Antigravity-Manager"

# Add modified files
Write-Host "📦 Staging files..." -ForegroundColor Yellow
git add docker/Dockerfile
git add src/locales/fr.json
git add src/i18n.ts

# Commit the changes
Write-Host ""
Write-Host "💾 Committing changes..." -ForegroundColor Yellow
git commit -m "fix: Docker cache + French translations

- Move CACHEBUST before COPY to invalidate Docker cache properly
- Add French translations for OpenAI account features
- Register French locale in i18n configuration"

# Push to GitHub
Write-Host ""
Write-Host "🚀 Pushing to GitHub..." -ForegroundColor Yellow
git push origin main

Write-Host ""
Write-Host "✅ Deployment complete!" -ForegroundColor Green
Write-Host ""
Write-Host "📌 Next steps in Coolify:" -ForegroundColor Cyan
Write-Host "   1. Go to Application Settings → Build" -ForegroundColor White
Write-Host "   2. Add build argument: CACHEBUST=2" -ForegroundColor White
Write-Host "   3. Save and Redeploy" -ForegroundColor White
Write-Host ""
Write-Host "   OR use Force Rebuild & Redeploy (no argument needed)" -ForegroundColor Gray
