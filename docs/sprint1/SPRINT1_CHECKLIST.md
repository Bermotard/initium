# ✅ SPRINT 1 - CORE ENGINE RUST - CHECKLIST COMPLÈTE

## 🎯 OBJECTIF SPRINT 1
Implémenter l'architecture core Rust et la logique de lancement multi-plateforme

**Durée:** 2 semaines  
**Tâches:** 8 (DEV-001 à DEV-008)  
**User Stories:** US-001, US-003, US-004

---

## 📋 TÂCHES SPRINT 1

### ✅ DEV-001: Config Cargo.toml
- [x] Dépendances ajoutées
- [x] Fichier prêt
- [x] `cargo check` OK
**Status:** ✅ COMPLÉTÉ

---

### 🔄 DEV-002: Structure Modules Rust (DÉBUT)

**Fichiers à créer/modifier:**
- [ ] `src/lib.rs` - Exports les modules
- [ ] `src/ui/mod.rs` - Module UI
- [ ] `src/system/mod.rs` - Module système

**Code à ajouter:**

```rust
// src/lib.rs - Library exports
pub mod launcher;
pub mod config;
pub mod system;
pub mod ui;
```

```rust
// src/ui/mod.rs
pub mod servo_engine;

// src/system/mod.rs
pub mod platform;
pub mod autostart;
```

**Vérification:**
```bash
cargo check  # Doit compiler sans erreur
```

**Critères d'acceptation:**
- [ ] Modules compilent sans erreurs
- [ ] Aucun warning
- [ ] `cargo check` OK
- [ ] Structure logique

---

### ⏳ DEV-003: Impl launcher.rs

**Fichier:** `src/launcher.rs`

**À implémenter:**
- [ ] Enum `LaunchType` (Web, App)
- [ ] Struct `Launcher` (id, name, type, target, icon)
- [ ] Fonction `execute_launcher()`
- [ ] Fonction `open_url()` (multi-OS)
- [ ] Fonction `execute_binary()` (multi-OS)

**Code de base fourni dans la doc**

**Tests à écrire:**
```rust
#[test]
fn test_launcher_creation() { }

#[tokio::test]
async fn test_execute_launcher_web() { }
```

---

### ⏳ DEV-004: Parser config.rs

**Fichier:** `src/config.rs`

**À implémenter:**
- [ ] Struct `Config`
- [ ] Impl `Config::load()`
- [ ] Impl `Config::save()`
- [ ] Impl `Config::add_launcher()`
- [ ] Impl `Config::remove_launcher()`

**Tester avec:** `/home/bernard/initium-dashboard/config.json`

---

### ⏳ DEV-005: Platform Abstractions

**Fichier:** `src/system/platform.rs`

**À implémenter:**
- [ ] Trait `PlatformManager`
- [ ] Impl pour Linux
- [ ] Impl pour Windows
- [ ] Impl pour macOS

**Utiliser:** `#[cfg(target_os = "...")]`

---

### ⏳ DEV-006: Menus Système

**Fichier:** `src/ui/menu.rs` (à créer)

**À implémenter:**
- [ ] Struct `MenuManager`
- [ ] Menu Fichier (Imprimer, Quitter)
- [ ] Menu Édition (Ajouter, Supprimer)
- [ ] Native pour chaque OS

---

### ⏳ DEV-007: Autostart Manager

**Fichier:** `src/system/autostart.rs`

**À implémenter:**
- [ ] Struct `AutostartManager`
- [ ] Impl Linux (XDG)
- [ ] Impl Windows (Registry)
- [ ] Impl macOS (LaunchAgents)

---

### ⏳ DEV-008: Tests Unitaires

**Fichier:** Tests dans chaque module

**À tester:**
- [ ] Config load/save
- [ ] Launcher creation
- [ ] Platform abstractions
- [ ] Autostart registration
- [ ] Coverage ≥80%

**Commande:**
```bash
cargo test
cargo tarpaulin  # Coverage
```

---

## 🔨 WORKFLOW DÉVELOPPEMENT

### Avant de Commencer

```bash
# 1. Cloner le repo (si nécessaire)
git clone https://github.com/Bermotard/initium.git
cd initium-dashboard

# 2. Vérifier l'installation
rustc --version
cargo --version

# 3. Vérifier que ça compile
cargo check
cargo test

# 4. Ouvrir dans VS Code
code .
```

### Pour Chaque Tâche

```bash
# 1. Créer branche
git checkout -b feature/DEV-XXX-description

# 2. Coder la tâche
# Utiliser la documentation Sprint 1 comme guide

# 3. Vérifier
cargo check
cargo fmt
cargo clippy
cargo test

# 4. Commit
git add .
git commit -m "feat(DEV-XXX): description de ce qui a été fait"

# 5. Merger localement (ou PR sur GitHub)
git checkout main
git merge feature/DEV-XXX-description
git push origin main
```

---

## 📚 DOCUMENTATION À CONSULTER

**Ouvrir dans le navigateur:**
```bash
firefox /home/bernard/initium-dashboard/docs/Initium_Phase2_Sprint1.html
```

**Cette doc contient:**
- Tâches détaillées (DEV-001 à DEV-008)
- Code examples complets
- Critères acceptation
- Matrice de test
- Risques et mitigation

---

## 🧪 COMMANDES UTILES

### Développement
```bash
# Compiler
cargo build

# Compiler release
cargo build --release

# Run
cargo run

# Check (rapide)
cargo check

# Format
cargo fmt

# Lint
cargo clippy

# Tests
cargo test

# Tests verbose
cargo test -- --nocapture

# Documentation
cargo doc --open

# Dépendances
cargo tree

# Audit sécurité
cargo audit
```

### Git
```bash
# Status
git status

# Log
git log --oneline

# Diff
git diff

# Voir les branches
git branch -a

# Changer de branche
git checkout main

# Supprimer branche
git branch -d feature/XXX
```

---

## 📊 PROGRESS TRACKING

### Semaine 1
- [ ] DEV-001: ✅ Cargo.toml
- [ ] DEV-002: Structure modules
- [ ] DEV-003: Impl launcher.rs
- [ ] DEV-004: Parser config.rs (moitié)

### Semaine 2
- [ ] DEV-004: Parser config.rs (suite)
- [ ] DEV-005: Platform abstractions
- [ ] DEV-006: Menus système
- [ ] DEV-007: Autostart manager
- [ ] DEV-008: Tests unitaires

---

## ✅ DEFINITION OF DONE

Pour chaque tâche:
- [ ] Code Rust écrit (idiomatique)
- [ ] Format: `cargo fmt` OK
- [ ] Lint: `cargo clippy` 0 warnings
- [ ] Tests: `cargo test` OK
- [ ] Documentation: Rustdoc écrite
- [ ] Code review: Prêt
- [ ] Git: Commit message clair
- [ ] Merge: Dans main

---

## 🎯 CRITÈRES ACCEPTATION SPRINT 1

- [ ] `cargo build --release` OK
- [ ] `cargo test` OK (≥80% coverage)
- [ ] `cargo clippy` 0 warnings
- [ ] `cargo fmt --check` OK
- [ ] Tests multiplateforme: Linux/Windows/macOS
- [ ] Documentation API complète
- [ ] Tous les commits pushés
- [ ] Pull Requests mergées

---

## 📞 RESSOURCES

**Documentation Rust:**
- https://doc.rust-lang.org/book/
- https://doc.rust-lang.org/cargo/

**Initium Docs:**
- Initium_Phase2_Sprint1.html (LOCAL)
- Initium_Phase1_Agile.html (Architecture)

**Tools:**
- VS Code: https://code.visualstudio.com/
- Rust: https://rustup.rs/

---

## 🚀 READY TO CODE!

```bash
cd /home/bernard/initium-dashboard
code .
```

Ouvrez `Initium_Phase2_Sprint1.html` et commencez par DEV-002!

**Bonne chance! 🎉**
