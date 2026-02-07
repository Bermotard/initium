# 🚀 SERVO DEPUIS GITHUB - GUIDE COMPLET

## ✅ OBJECTIF
Intégrer Servo directement depuis GitHub pour avoir le vrai moteur de rendu.

---

## 📋 ÉTAPE 1: Modifier Cargo.toml

**Ouvrir:** `/home/bernard/initium-dashboard/Cargo.toml`

**CHERCHER:**
```toml
# UI & Rendering
servo = "0.0.1"
```

**REMPLACER PAR:**
```toml
# UI & Rendering
servo = { git = "https://github.com/servo/servo.git", branch = "main" }
```

**SAUVEGARDER** (Ctrl+S)

---

## 📋 ÉTAPE 2: Nettoyer et Mettre à Jour

**Terminal:**
```bash
cd /home/bernard/initium-dashboard

# Nettoyer les anciens builds
cargo clean

# Mettre à jour les dépendances (va cloner Servo de GitHub)
cargo update

# Cette étape peut prendre un peu de temps (Servo est gros)
```

---

## 📋 ÉTAPE 3: Vérifier

```bash
# Vérifier la compilation
cargo check
```

**Attendez:** Cargo va télécharger et compiler Servo depuis GitHub. C'est peut-être long (Servo est un gros projet).

---

## 🚨 PROBLÈMES POTENTIELS

### Problème 1: "error: failed to clone repository"
**Cause:** Pas d'accès à GitHub ou problème réseau

**Solution:**
```bash
# Vérifier la connexion
ping github.com

# Ou vérifier SSH
git ls-remote https://github.com/servo/servo.git HEAD
```

### Problème 2: "error: failed to compile"
**Cause:** Servo nécessite peut-être Rust nightly

**Solution:**
```bash
# Installer Rust nightly
rustup install nightly

# Compiler avec nightly
cargo +nightly check

# Ou ajouter dans rust-toolchain.toml:
echo 'nightly' > rust-toolchain.toml
cargo check
```

### Problème 3: "Servo has unmet dependencies"
**Cause:** Dépendances système manquantes

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install -y \
  libssl-dev \
  libxcb-render0-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  libxkbcommon-dev \
  python3 \
  autoconf \
  libtool \
  pkg-config
```

**macOS:**
```bash
brew install autoconf libtool pkg-config
```

**Windows:**
```bash
# Installer Visual Studio Community avec les outils C++
# Ou utiliser MSVC via: rustup show
```

### Problème 4: Compilation très longue
**Normal!** Servo est gros. Cela peut prendre 30-60 minutes la première fois.

**Solution:** Soyez patient ou utiliser `-j1` pour compiler avec 1 thread (plus lent mais moins gourmand):
```bash
cargo check -j1
```

---

## ✅ APRÈS SUCCÈS

Une fois `cargo check` qui passe:

```bash
# Tester
cargo test

# Voir si ça compile réellement
cargo build --release

# Voir la taille
ls -lh target/release/initium*
```

---

## 📝 PROCHAINES ÉTAPES (Sprint 1)

Si Servo compile avec succès:

```bash
# Commiter le changement
git add Cargo.toml Cargo.lock
git commit -m "feat: integrate servo from GitHub"
git push origin main

# Commencer Sprint 1
git checkout -b feature/DEV-002-structure-modules
code .
```

---

## 🎯 RÉSUMÉ

| Étape | Action | Temps |
|-------|--------|-------|
| 1 | Modifier Cargo.toml | 1 min |
| 2 | `cargo clean && cargo update` | 5-15 min |
| 3 | `cargo check` | 30-60 min (première fois) |
| 4 | Résoudre les problèmes | ? |
| 5 | Tester | 10-20 min |
| 6 | Commencer Sprint 1 | 🚀 |

---

## 🔗 RÉFÉRENCES

- **Servo GitHub:** https://github.com/servo/servo
- **Servo Book:** https://servo.org/docs/
- **Issues Servo:** https://github.com/servo/servo/issues

---

## 💡 TIPS

1. **Vérifiez Rust version:**
   ```bash
   rustc --version
   cargo --version
   ```

2. **Utilisez un autre terminal pour travailler** pendant que Cargo compile (compilation longue)

3. **Sauvegardez Cargo.lock** dans Git une fois que ça marche:
   ```bash
   git add Cargo.lock
   git commit -m "chore: add Cargo.lock for reproducible builds"
   ```

4. **Si ça prend trop longtemps**, vous pouvez arrêter (Ctrl+C) et revenir plus tard

---

**Bonne chance! Servo est un projet impressionnant, mais la compilation est intense! 🚀**

