#!/bin/bash
# Deploy OpenAI account feature to GitHub

echo "🚀 Deploying OpenAI Account Feature"
echo ""

# Navigate to project directory
cd "$(dirname "$0")"

# Configure git user if needed
git config user.email "aminetwiti@gmail.com" || true
git config user.name "Amine" || true

# Check git status
echo "📋 Current status:"
git status --short

# Add OpenAI-related files
echo ""
echo "📦 Staging OpenAI feature files..."
git add src/pages/Accounts.tsx
git add src/components/accounts/AddOpenAIAccountDialog.tsx
git add src/stores/useAccountStore.ts
git add src/services/accountService.ts
git add src/locales/en.json
git add src/locales/zh.json

# Also stage backend files to ensure completeness
git add src-tauri/src/auth/openai_web.rs
git add src-tauri/src/auth/openai_oauth.rs
git add src-tauri/src/commands/openai_accounts.rs
git add src-tauri/src/proxy/clients/chatgpt.rs
git add src-tauri/src/proxy/handlers/openai.rs

# Commit the changes
echo ""
echo "💾 Committing changes..."
git commit -m "feat: Add OpenAI account integration with UI button

- Add OpenAI account button (Bot icon) next to Add Account button in Accounts page
- Include AddOpenAIAccountDialog component with Web and API tabs
- Support for OpenAI Web session tokens and API keys
- Backend integration: Rust handlers, OAuth flow, ChatGPT client
- Multi-provider token manager supporting Google + OpenAI accounts
- Translations for OpenAI account features (EN, ZH)

This enables users to add and manage OpenAI accounts alongside Google accounts."

# Push to GitHub
echo ""
echo "📤 Pushing to GitHub..."
git push origin main

echo ""
echo "✅ Done! Coolify will auto-deploy in ~5-10 minutes."
echo ""
echo "📍 After deployment:"
echo "   Navigate to Accounts page → You'll see 2 buttons:"
echo "   • [+] Google accounts"
echo "   • [🤖] OpenAI accounts (NEW)"
echo ""
