# 🚀 GUIDE COMPLET - LANCER SPRINT 1 INITIUM

## 💡 IDE RECOMMANDÉ: VS Code

Je vous recommande **VS Code** (Visual Studio Code) pour plusieurs raisons:

✅ **Meilleur support Rust** - Extension rust-analyzer excellente  
✅ **Léger & Rapide** - Démarrage instantané  
✅ **Gratuit & Open Source** - Pas de coûts  
✅ **Excellent écosystème** - Extensions nombreuses  
✅ **Terminal intégré** - Idéal pour cargo commands  
✅ **Debugging** - Extension CodeLLDB très puissante  

### Alternatives:
- **JetBrains RustRover** - Plus complet mais payant
- **Vim/Neovim** - Pour experts, terminal only

---

## 🛠️ SETUP INITIAL (30 minutes)

### ÉTAPE 1: Installer VS Code

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install -y code
```

**macOS:**
```bash
brew install visual-studio-code
```

**Windows:**
Télécharger depuis: https://code.visualstudio.com/

### ÉTAPE 2: Installer Rust (si pas déjà installé)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Vérifier
rustc --version
cargo --version
```

### ÉTAPE 3: Installer Extensions VS Code

Ouvrir VS Code et installer ces extensions:

1. **rust-analyzer** (essentiellement)
   - ID: rust-lang.rust-analyzer
   - Meilleur support Rust, autocomplétion, IntelliSense

2. **CodeLLDB** (debugger)
   - ID: vadimcn.vscode-lldb
   - Pour débugger le code Rust

3. **Better TOML**
   - ID: tamasfe.even-better-toml
   - Syntax highlighting pour Cargo.toml

4. **Crates** (optionnel)
   - ID: serayuzgur.crates
   - Gère les versions des dépendances

5. **Error Lens** (optionnel)
   - ID: usernamehw.errorlens
   - Affiche les erreurs inline

```bash
# Installer extensions via CLI
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension tamasfe.even-better-toml
```

### ÉTAPE 4: Configurer VS Code pour Rust

Créer/modifier le fichier `.vscode/settings.json` dans votre projet:

```json
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "rust-lang.rust-analyzer",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.hover.documentation.enable": true,
  "rust-analyzer.inlayHints.enable": true,
  "files.exclude": {
    "**/target": true
  }
}
```

---

## 🚀 LANCER SPRINT 1 (Pas à Pas)

### ÉTAPE 1: Ouvrir le Projet

```bash
cd /home/bernard/initium-dashboard
code .
```

VS Code va:
1. Analyser le projet Rust
2. Télécharger rust-analyzer
3. Indexer le code

⏳ **Attendre 30-60 secondes** la première fois

### ÉTAPE 2: Vérifier l'Installation

```bash
# Terminal VS Code (Ctrl+`)

# Vérifier Rust
rustc --version
cargo --version

# Vérifier compilation
cargo check

# Lancer les tests
cargo test
```

Vous devez voir:
- ✅ `cargo check` compile sans erreurs
- ✅ `cargo test` passe tous les tests

### ÉTAPE 3: Ouvrir la Documentation Sprint 1

Ouvrir le fichier de doc dans le navigateur:

```bash
# Dans terminal ou direct
firefox /home/bernard/initium-dashboard/docs/Initium_Phase2_Sprint1.html
```

Ou depuis VS Code:
1. Ctrl+O
2. Naviguer vers: `/home/bernard/initium-dashboard/docs/Initium_Phase2_Sprint1.html`
3. Ouvrir dans le navigateur par défaut

### ÉTAPE 4: Créer Branche Git pour Sprint 1

```bash
# Terminal VS Code
cd /home/bernard/initium-dashboard

# Créer branche
git checkout -b sprint/core-engine

# Vérifier
git branch
```

### ÉTAPE 5: Explorer la Structure du Projet

Dans VS Code, explorer ces fichiers:

```
initium-dashboard/
├── src/
│   ├── main.rs              ← Entry point
│   ├── lib.rs               ← Exports modules
│   ├── launcher.rs          ← À développer (DEV-003)
│   ├── config.rs            ← À développer (DEV-004)
│   ├── ui/
│   │   ├── mod.rs
│   │   └── servo_engine.rs  ← À développer (DEV-006)
│   └── system/
│       ├── mod.rs
│       ├── platform.rs      ← À développer (DEV-005)
│       └── autostart.rs     ← À développer (DEV-007)
├── Cargo.toml               ← Dépendances (DEV-001 ✅)
├── docs/
│   └── Initium_Phase2_Sprint1.html  ← À LIRE!
└── config.json              ← Configuration exemple
```

---

## 📋 COMMANDES CLÉS POUR SPRINT 1

### Dans le Terminal VS Code (Ctrl+`)

```bash
# Compilation rapide
cargo check

# Compiler
cargo build

# Tests
cargo test

# Lint (détecte erreurs)
cargo clippy

# Format code
cargo fmt

# Voir la doc API
cargo doc --open

# Créer nouvelle branche
git checkout -b feature/DEV-002-structure

# Commit
git add .
git commit -m "feat(DEV-002): implement module structure"

# Push
git push -u origin feature/DEV-002-structure
```

---

## 🎯 WORKFLOW SPRINT 1 (Détaillé)

### Pour Chaque Tâche (DEV-002, DEV-003, etc.)

```
1. LIRE
   └─ Ouvrir Initium_Phase2_Sprint1.html
   └─ Lire la section de la tâche (DEV-002, DEV-003, etc.)

2. CRÉER BRANCHE
   └─ git checkout -b feature/DEV-XXX-description

3. CODER
   └─ Ouvrir le fichier à modifier dans VS Code
   └─ Utiliser le code example de la doc
   └─ VS Code affiche erreurs/warnings en temps réel

4. TESTER
   └─ cargo check (rapide, vérifie compilation)
   └─ cargo clippy (détecte les warnings)
   └─ cargo test (lance les tests)

5. FORMATER
   └─ cargo fmt (formate automatiquement)
   └─ Ou Shift+Alt+F dans VS Code

6. COMMIT
   └─ git add .
   └─ git commit -m "feat(DEV-XXX): description"

7. PUSH
   └─ git push -u origin feature/DEV-XXX-description

8. GITHUB
   └─ Aller sur GitHub
   └─ Créer Pull Request
   └─ Review + Merge dans main
```

---

## 🔧 SHORTCUTS VS CODE UTILES

| Action | Raccourci |
|--------|-----------|
| Format document | `Shift + Alt + F` |
| Find in file | `Ctrl + F` |
| Replace | `Ctrl + H` |
| Go to file | `Ctrl + P` |
| Go to line | `Ctrl + G` |
| Terminal | `Ctrl + `` |
| Command palette | `Ctrl + Shift + P` |
| Run task | `Ctrl + Shift + B` |
| Debug | `F5` |

---

## 📊 PROGRESSION SPRINT 1

### Semaine 1
- [ ] DEV-001: ✅ Cargo.toml (déjà fait)
- [ ] DEV-002: Structure modules Rust (1-2 jours)
- [ ] DEV-003: Impl launcher.rs (2-3 jours)
- [ ] DEV-004: Parser config.rs (2-3 jours)

### Semaine 2
- [ ] DEV-005: Platform abstractions (2-3 jours)
- [ ] DEV-006: Menus système (1-2 jours)
- [ ] DEV-007: Autostart manager (1-2 jours)
- [ ] DEV-008: Tests unitaires (1-2 jours)

---

## ✅ PREMIER DEV-002: Structure Modules

Pour démarrer immédiatement, voici DEV-002:

### Fichiers à modifier:

**1. `src/lib.rs` - Ajouter les exports:**
```rust
pub mod launcher;
pub mod config;
pub mod system;
pub mod ui;
```

**2. `src/ui/mod.rs` - Créer le module UI:**
```rust
pub mod servo_engine;
```

**3. `src/system/mod.rs` - Créer le module système:**
```rust
pub mod platform;
pub mod autostart;
```

### Après les modifications:
```bash
cargo check  # Doit compiler sans erreur ✅
cargo test   # Tests doivent passer ✅
```

---

## 🆘 TROUBLESHOOTING

### Problem: "cargo not found"
```bash
# Solution: Ajouter Rust au PATH
source $HOME/.cargo/env
```

### Problem: "rust-analyzer not installed"
```bash
# Solution: Installer l'extension dans VS Code
code --install-extension rust-lang.rust-analyzer
```

### Problem: "Cargo.lock conflict"
```bash
# Solution: Commit le Cargo.lock
git add Cargo.lock
git commit -m "chore: add Cargo.lock"
```

### Problem: "Module not found"
- Vérifier que le module est déclaré dans `mod.rs`
- Vérifier la structure des répertoires

### Problem: "Clippy warnings"
- Lire le message d'erreur
- Utiliser la suggestion proposée
- Relancer: `cargo clippy`

---

## 📞 RESSOURCES

### Documentation Officielle
- **Rust Book:** https://doc.rust-lang.org/book/
- **Cargo:** https://doc.rust-lang.org/cargo/
- **Rust API:** https://docs.rs/

### Votre Projet
- **GitHub:** https://github.com/Bermotard/initium
- **Docs locales:** `/home/bernard/initium-dashboard/docs/`
- **Sprint 1:** `Initium_Phase2_Sprint1.html`

### VS Code
- **Official:** https://code.visualstudio.com/
- **Shortcuts:** https://code.visualstudio.com/docs/editor/codebasics
- **rust-analyzer:** https://rust-analyzer.github.io/

---

## 🎯 CHECKLIST AVANT DE DÉMARRER

- [ ] VS Code installé
- [ ] Rust installé (`rustc --version` OK)
- [ ] Extensions Rust installées
- [ ] Projet ouvert dans VS Code
- [ ] `cargo check` compile ✅
- [ ] `cargo test` passe ✅
- [ ] Documentation Sprint 1 ouverte
- [ ] Branche `sprint/core-engine` créée
- [ ] Prêt à coder! 🚀

---

## 🚀 DÉMARRAGE IMMÉDIAT

```bash
# 1. Ouvrir le projet
cd /home/bernard/initium-dashboard
code .

# 2. Ouvrir terminal (Ctrl+`)

# 3. Vérifier
cargo check
cargo test

# 4. Créer branche
git checkout -b feature/DEV-002-structure-modules

# 5. Lire la doc
firefox /home/bernard/initium-dashboard/docs/Initium_Phase2_Sprint1.html

# 6. Coder DEV-002
# Modifier src/lib.rs, src/ui/mod.rs, src/system/mod.rs

# 7. Vérifier
cargo check
cargo clippy
cargo test

# 8. Commit
git add .
git commit -m "feat(DEV-002): implement module structure"

# 9. Push
git push -u origin feature/DEV-002-structure-modules

# 10. GitHub: Créer PR et merger
```

---

## 🎉 VOUS ÊTES PRÊT!

Vous avez maintenant:
- ✅ IDE optimal (VS Code)
- ✅ Environment configuré (Rust + Extensions)
- ✅ Projet ouvert et prêt
- ✅ Documentation accessible
- ✅ Git workflow clarifié
- ✅ Commandes de référence

**Commencez par DEV-002, c'est simple et rapide!**

**Bonne chance! 🚀**
