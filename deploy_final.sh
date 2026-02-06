#!/bin/bash
# Script de déploiement - Docker Cache Fix + Traductions FR

echo "🔧 Déploiement des corrections OpenAI..."
echo ""

# Git add
echo "📦 Staging des fichiers..."
git add docker/Dockerfile
git add src/locales/fr.json
git add src/i18n.ts

# Git commit
echo "💾 Commit..."
git commit -m "fix: Docker cache + traductions françaises OpenAI

- Déplacer CACHEBUST avant COPY pour invalider le cache Docker correctement
- Ajouter traductions françaises pour les fonctionnalités OpenAI
- Enregistrer locale française dans i18n"

# Git push
echo "🚀 Push vers GitHub..."
git push origin main

echo ""
echo "✅ Déploiement terminé!"
echo ""
echo "📌 Prochaines étapes dans Coolify:"
echo "   1. Ouvrir Application Settings → Build"
echo "   2. Ajouter build argument: CACHEBUST=2"
echo "   3. Sauvegarder et Redéployer"
echo ""
echo "⏳ Attendre 5-10 minutes pour le rebuild complet"
echo "🔄 Vider cache navigateur: Ctrl+Shift+R"
echo "👀 Vérifier page /accounts pour voir [+] [🤖] côte à côte"
