# Push fixes to GitHub
Write-Host "Pushing changes to GitHub..." -ForegroundColor Cyan

try {
    # Check current status
    Write-Host "`nChecking git status..." -ForegroundColor Yellow
    git status

    # Push to origin/main
    Write-Host "`nPushing to origin main..." -ForegroundColor Yellow
    git push origin main

    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n✅ Successfully pushed changes to GitHub!" -ForegroundColor Green
        Write-Host "Coolify will auto-deploy from the main branch." -ForegroundColor Cyan
        Write-Host "`nYour OpenAI button fix is now deployed:" -ForegroundColor Cyan
        Write-Host "- Removed static Tauri imports that crashed the page in web mode" -ForegroundColor White
        Write-Host "- Added dynamic imports with isTauri() check" -ForegroundColor White
        Write-Host "- Fixed OAuth button to only work in desktop app" -ForegroundColor White
    } else {
        Write-Host "`n❌ Push failed with exit code: $LASTEXITCODE" -ForegroundColor Red
        Write-Host "You may need to authenticate with GitHub." -ForegroundColor Yellow
    }
} catch {
    Write-Host "`n❌ Error: $_" -ForegroundColor Red
}
