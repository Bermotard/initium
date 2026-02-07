# 🚀 SPRINT 1 - QUICK START

## ✅ PLAN ADOPTÉ

- **Sprint 1:** Core Engine (launcher, config, platform)
- **Sprint 2:** UI avec Tauri + HTML/CSS/JS

---

## 📋 ÉTAPE 1: Nettoyer Cargo.toml

**Ouvrir:** `/home/bernard/initium-dashboard/Cargo.toml`

**Remplacer TOUT par ceci:**

```toml
[package]
name = "initium"
version = "0.1.0-alpha"
edition = "2021"

[dependencies]
# Core
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
log = "0.4"
env_logger = "0.10"
cfg-if = "1.0"
hyper = { version = "0.14", features = ["full"] }
http = "0.2"
anyhow = "1.0"
thiserror = "1.0"

# Platform
dirs = "5.0"
lazy_static = "1.4"

# Platform-specific
[target.'cfg(target_os = "linux")'.dependencies]
xdg = "2.4"

[target.'cfg(target_os = "windows")'.dependencies]
winreg = "0.50"

[target.'cfg(target_os = "macos")'.dependencies]
plist = "1.5"

# Dev
[dev-dependencies]
tokio-test = "0.4"
```

**Sauvegarder** (Ctrl+S)

---

## 📋 ÉTAPE 2: Nettoyer et Vérifier

**Terminal:**

```bash
cd /home/bernard/initium-dashboard

# Nettoyer
cargo clean

# Vérifier
cargo check
```

**Ça devrait passer rapidement!** ✅

---

## 📋 ÉTAPE 3: Tester

```bash
cargo test
```

**Tout devrait marcher!** ✅

---

## 📋 ÉTAPE 4: Commiter

```bash
git add .
git commit -m "chore: simplify dependencies for Sprint 1 (Core Engine only)"
git push origin main
```

---

## 🚀 ÉTAPE 5: Démarrer Sprint 1

```bash
# Créer branche Sprint 1
git checkout -b sprint/core-engine

# Ouvrir VS Code
code .
```

---

## 📝 SPRINT 1 - TÂCHES

### ✅ DEV-001: Cargo.toml
- ✅ COMPLÉTÉ

### 🔄 DEV-002: Structure modules (COMMENCER ICI)

**Fichiers à modifier:**
- `src/lib.rs` - Exporter les modules
- `src/ui/mod.rs` - Module UI (placeholder)
- `src/system/mod.rs` - Module système

**Code:**

**src/lib.rs:**
```rust
pub mod launcher;
pub mod config;
pub mod system;
pub mod ui;
```

**src/ui/mod.rs:**
```rust
pub mod servo_engine;
```

**src/system/mod.rs:**
```rust
pub mod platform;
pub mod autostart;
```

**Vérifier:**
```bash
cargo check
cargo test
```

---

### ⏳ DEV-003 à DEV-008

Voir la documentation Sprint 1 pour les détails:
`/home/bernard/initium-dashboard/docs/Initium_Phase2_Sprint1.html`

---

## 🎯 WORKFLOW SPRINT 1

**Pour chaque tâche (DEV-002 à DEV-008):**

```bash
# 1. Lire la doc
# Ouvrir: Initium_Phase2_Sprint1.html
# Lire section DEV-XXX

# 2. Créer branche
git checkout -b feature/DEV-XXX-description

# 3. Coder
# Utiliser VS Code
# code .

# 4. Tester
cargo check
cargo clippy
cargo test

# 5. Formatter
cargo fmt

# 6. Commit
git add .
git commit -m "feat(DEV-XXX): description"

# 7. Push
git push -u origin feature/DEV-XXX-description

# 8. GitHub
# Créer Pull Request
# Merger dans main

# 9. Revenir à main
git checkout sprint/core-engine
git pull origin main
```

---

## 🎯 RÉSUMÉ SPRINT 1

| Tâche | Description | Durée |
|-------|-------------|-------|
| DEV-001 | Cargo.toml | ✅ FAIT |
| DEV-002 | Structure modules | 1-2 jours |
| DEV-003 | launcher.rs | 2-3 jours |
| DEV-004 | config.rs | 2-3 jours |
| DEV-005 | Platform abstractions | 2-3 jours |
| DEV-006 | Menus système | 1-2 jours |
| DEV-007 | Autostart manager | 1-2 jours |
| DEV-008 | Tests unitaires | 1-2 jours |

**Total: ~2 semaines**

---

## ✨ PROCHAINES ÉTAPES

1. ✅ Modifier Cargo.toml
2. ✅ `cargo check` et `cargo test`
3. ✅ Commit
4. ✅ Créer branche `sprint/core-engine`
5. 🔄 Commencer DEV-002
6. 📝 Lire la doc Sprint 1
7. 🚀 Coder le Core Engine

---

## 📚 RESSOURCES

- **Sprint 1 Doc:** `/home/bernard/initium-dashboard/docs/Initium_Phase2_Sprint1.html`
- **GitHub Repo:** https://github.com/Bermotard/initium
- **Rust Book:** https://doc.rust-lang.org/book/
- **Cargo:** https://doc.rust-lang.org/cargo/

---

## 🎉 VOUS ÊTES PRÊT!

Cargo.toml va compiler rapidement, plus de problèmes de Servo/Tauri.

**Commencez par DEV-002, c'est simple!** 💪

**Bonne chance! 🚀**

