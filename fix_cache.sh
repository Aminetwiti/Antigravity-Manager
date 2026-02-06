#!/bin/bash
# Force rebuild after cache issue

echo "🔄 Forcing cache invalidation and rebuild..."
echo ""

cd "$(dirname "$0")"

# Stage updated Dockerfile
git add docker/Dockerfile

# Commit with cache bust
git commit -m "fix: Add CACHEBUST arg to Dockerfile to force frontend rebuild

Docker was caching old layers and not rebuilding the frontend with OpenAI button.
This adds a CACHEBUST argument that Coolify can increment to invalidate cache."

# Push
git push origin main

echo ""
echo "✅ Pushed cache fix!"
echo ""
echo "🔧 Next steps in Coolify dashboard:"
echo "   1. Go to your application settings"
echo "   2. Click 'Redeploy' or 'Force Rebuild'"
echo "   3. OR wait for auto-deploy (~2 min)"
echo ""
echo "⚠️  If button still doesn't appear:"
echo "   • In Coolify: Add build argument CACHEBUST=2 (or 3, 4...)"
echo "   • This forces Docker to rebuild from scratch"
echo ""
