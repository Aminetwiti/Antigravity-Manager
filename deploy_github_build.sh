#!/bin/bash
# Script de déploiement - Fix Docker Compose pour utiliser le code GitHub

echo "🔧 Fix Docker Compose - Utiliser le code GitHub au lieu du Docker Hub"
echo ""

# Git add
echo "📦 Staging des fichiers..."
git add docker/docker-compose.yml

# Git commit
echo "💾 Commit..."
git commit -m "fix: Use local build instead of Docker Hub image in docker-compose

Change image from lbjlaq/antigravity-manager:latest to antigravity-manager:local
This ensures Coolify builds from GitHub repo (Aminetwiti/Antigravity-Manager)
instead of pulling old image from Docker Hub without OpenAI button.

Build process:
- Builds from source code in GitHub
- Includes latest OpenAI account features
- Respects CACHEBUST for frontend rebuilds"

# Git push
echo "🚀 Push vers GitHub..."
git push origin main

echo ""
echo "✅ Déploiement terminé!"
echo ""
echo "📌 Dans Coolify:"
echo "   1. Redéployer (il va maintenant BUILD au lieu de PULL)"
echo "   2. Attendre 10-15 minutes pour le build complet"
echo "   3. Chercher dans les logs: 'npm install', 'vite build', 'cargo build'"
echo ""
echo "🎯 Résultat: Bouton [🤖 Add OpenAI Account] visible dans /accounts"
