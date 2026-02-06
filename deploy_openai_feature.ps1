# Script to deploy OpenAI account feature to GitHub
# Run this script to commit and push the OpenAI button to your server

Write-Host "🚀 Deploying OpenAI Account Feature" -ForegroundColor Cyan
Write-Host ""

# Navigate to project directory
Set-Location "C:\Users\amine\Antigravity-Manager"

# Check git status
Write-Host "📋 Checking git status..." -ForegroundColor Yellow
git status

# Add OpenAI-related files
Write-Host ""
Write-Host "📦 Staging OpenAI feature files..." -ForegroundColor Yellow
git add src/pages/Accounts.tsx
git add src/components/accounts/AddOpenAIAccountDialog.tsx
git add src/stores/useAccountStore.ts
git add src/services/accountService.ts
git add src/locales/en.json
git add src/locales/zh.json

# Commit the changes
Write-Host ""
Write-Host "💾 Committing changes..." -ForegroundColor Yellow
git commit -m "feat: Add OpenAI account integration button and dialog to Accounts page

- Add OpenAI account button (Bot icon) next to Add Account button
- Include AddOpenAIAccountDialog component with Web and API tabs
- Add translations for OpenAI account features
- Backend integration already exists (Rust handlers, commands, etc.)
"

# Push to GitHub
Write-Host ""
Write-Host "📤 Pushing to GitHub..." -ForegroundColor Yellow
git push origin main

Write-Host ""
Write-Host "✅ Done! Coolify will now auto-deploy the changes." -ForegroundColor Green
Write-Host ""
Write-Host "📍 After deployment completes:" -ForegroundColor Cyan
Write-Host "   1. Navigate to the Accounts page on your server"
Write-Host "   2. You should see TWO buttons:"
Write-Host "      • '+' Button → Google accounts (OAuth, Refresh Token, Import)"
Write-Host "      • '🤖' Button (green) → OpenAI accounts (Web, API)"
Write-Host ""
Write-Host "⏱️  Deployment may take 5-10 minutes..." -ForegroundColor Yellow
