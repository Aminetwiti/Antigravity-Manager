# 🎯 Guide de Déploiement Coolify - Fix Bouton OpenAI

Ce guide explique comment résoudre le problème du bouton OpenAI manquant dans Coolify.

## 🐛 Problème Identifié

Coolify utilisait `image: lbjlaq/antigravity-manager:latest` du Docker Hub, qui est l'**ancienne version sans le bouton OpenAI**. Le "Force Rebuild" ne faisait que redémarrer le container sans rebuilder l'image.

## ✅ Solution Appliquée

Le `docker-compose.yml` a été modifié pour **builder localement** depuis le Dockerfile:

```yaml
services:
  antigravity-manager:
    build:
      context: ..
      dockerfile: docker/Dockerfile
      args:
        CACHEBUST: "${CACHEBUST:-1}"
    image: lbjlaq/antigravity-manager:latest
    # reste de la config...
```

## 📦 Déploiement dans Coolify

### Étape 1: Pusher les Modifications

```bash
cd C:\Users\amine\Antigravity-Manager
git add docker/docker-compose.yml docker/.env.example docker/README.md
git commit -m "fix: Configure docker-compose to build locally instead of using Docker Hub image"
git push origin main
```

### Étape 2: Configurer Coolify

1. **Ouvrir Coolify Dashboard**
2. **Aller dans Application Settings**
3. **Section "Build"**:
   - **Ajouter Variable d'Environnement** (Build Time):
     ```
     CACHEBUST=2
     ```
4. **Sauvegarder**

### Étape 3: Force Rebuild

1. **Cliquer sur "Deploy" ou "Redeploy"**
2. **Attendre 10-15 minutes** (premier build complet)
3. **Vérifier les logs** - Vous devriez voir:
   ```
   npm install
   npm run tauri build
   Building frontend...
   Compiling Rust...
   ```

### Étape 4: Vérification

1. **Vider cache navigateur**: `Ctrl + Shift + R`
2. **Recharger** `https://openai.ty-dev.site/accounts`
3. **Chercher**: `[+]` `[🤖 Add OpenAI Account]` `[Refresh All]`

## 🔧 Dépannage

### Si le bouton n'apparaît toujours pas:

1. **Vérifier que le build a vraiment eu lieu**:
   ```bash
   # Dans les logs Coolify, chercher:
   "npm install"
   "Building frontend"
   "vite v7.2.7"
   ```

2. **Incrémenter CACHEBUST**:
   - Changer `CACHEBUST=2` → `CACHEBUST=3` dans Coolify
   - Redéployer

3. **Build manuel en SSH sur le serveur**:
   ```bash
   cd /path/to/coolify/artifacts
   docker compose -f docker/docker-compose.yml build --no-cache
   docker compose -f docker/docker-compose.yml up -d
   ```

## 📊 Différence Avant/Après

### ❌ Avant (Ne Fonctionnait Pas)
```yaml
image: lbjlaq/antigravity-manager:latest
```
→ Télécharge image Docker Hub (ancienne version)
→ `docker compose up -d` redémarre juste le container
→ Pas de rebuild, pas de nouveau code

### ✅ Après (Fonctionne)
```yaml
build:
  context: ..
  dockerfile: docker/Dockerfile
  args:
    CACHEBUST: "${CACHEBUST:-1}"
```
→ Build localement depuis GitHub
→ Compile frontend + backend avec le nouveau code
→ Bouton OpenAI inclus ✨

## 🎉 Résultat Attendu

Après le déploiement, tu devrais voir **3 boutons** dans `/accounts`:

```
╔════════════════════════════════════════════╗
║  [Search...]                               ║
║                                            ║
║  [+ Add Account] [🤖 Add OpenAI] [↻ Refresh]
║   ↑ Google        ↑ OpenAI (NOUVEAU!)    ║
╚════════════════════════════════════════════╝
```

Le bouton 🤖 vert avec "Add OpenAI Account" devrait être visible entre les boutons "+" et "Refresh"!
