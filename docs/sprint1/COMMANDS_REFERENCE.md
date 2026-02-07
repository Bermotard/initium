# 🔧 COMMANDES DE RÉFÉRENCE - SPRINT 1

## Installation Initiale

### Installer Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Vérifier l'installation
```bash
rustc --version    # Rust compiler
cargo --version    # Cargo package manager
rustup --version   # Rust installer
```

### Installer VS Code
```bash
# Télécharger depuis: https://code.visualstudio.com/
# Ou via package manager:
sudo apt install code      # Linux
brew install visual-studio-code  # macOS
```

### Installer Extensions VS Code
1. Ouvrir VS Code
2. Extensions (Ctrl+Shift+X)
3. Chercher et installer:
   - `rust-analyzer` (pour support Rust)
   - `CodeLLDB` (pour debugger)
   - `Better TOML` (pour Cargo.toml)
   - `Crates` (pour gérer dépendances)
   - `Error Lens` (pour erreurs inline)

---

## Commandes Projet

### Premiers Pas
```bash
cd /home/bernard/initium-dashboard

# Ouvrir VS Code
code .

# Vérifier installation
rustc --version
cargo --version
```

---

## Cargo - Build & Compilation

### Vérifier compilation (rapide)
```bash
cargo check
```

### Compiler debug
```bash
cargo build
```

### Compiler release (optimisé, lent)
```bash
cargo build --release
```

### Compiler un binaire spécifique
```bash
cargo build --bin initium
```

### Clean (supprimer build)
```bash
cargo clean
```

---

## Cargo - Exécution

### Exécuter avec debug
```bash
cargo run
```

### Exécuter release
```bash
cargo run --release
```

### Passer des arguments
```bash
cargo run -- --arg value
```

---

## Cargo - Tests

### Lancer tous les tests
```bash
cargo test
```

### Tests avec output
```bash
cargo test -- --nocapture
```

### Tests d'un module spécifique
```bash
cargo test launcher::  # Tests module launcher
```

### Tests single-threaded
```bash
cargo test -- --test-threads=1
```

### Tests avec debug info
```bash
RUST_BACKTRACE=1 cargo test
```

### Coverage (nécessite tarpaulin)
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

---

## Cargo - Qualité Code

### Lint (détecter erreurs)
```bash
cargo clippy
```

### Clippy avec warnings stricts
```bash
cargo clippy -- -D warnings
```

### Format code
```bash
cargo fmt
```

### Vérifier format sans modifier
```bash
cargo fmt -- --check
```

### Format spécifique
```bash
cargo fmt -- --edition 2021
```

---

## Cargo - Documentation

### Générer docs
```bash
cargo doc
```

### Générer docs + ouvrir browser
```bash
cargo doc --open
```

### Docs sans dépendances
```bash
cargo doc --no-deps
```

---

## Cargo - Dépendances

### Voir l'arbre des dépendances
```bash
cargo tree
```

### Arbre complet (inclut dev deps)
```bash
cargo tree --all-features
```

### Mettre à jour dépendances
```bash
cargo update
```

### Audit sécurité
```bash
cargo audit
```

### Outdated packages
```bash
cargo outdated
```

### Chercher une crate
```bash
cargo search nom_de_crate
```

---

## Cargo - Benchmarks

### Lancer benchmarks
```bash
cargo bench
```

### Benchmark spécifique
```bash
cargo bench bench_name
```

---

## Git - Branches

### Voir branches locales
```bash
git branch
```

### Voir toutes les branches (inclut remote)
```bash
git branch -a
```

### Créer branche
```bash
git branch feature/DEV-002-structure
```

### Créer + checkout branche
```bash
git checkout -b feature/DEV-002-structure
```

### Changer de branche
```bash
git checkout main
git checkout feature/DEV-002-structure
```

### Renommer branche
```bash
git branch -m old_name new_name
```

### Supprimer branche locale
```bash
git branch -d feature/DEV-002-structure
```

### Supprimer branche force
```bash
git branch -D feature/DEV-002-structure
```

---

## Git - Commits

### Status
```bash
git status
```

### Ajouter tous les fichiers
```bash
git add .
```

### Ajouter fichier spécifique
```bash
git add src/launcher.rs
```

### Ajouter interactif
```bash
git add -p
```

### Commit
```bash
git commit -m "feat(DEV-002): implement module structure"
```

### Commit amendé (ajouter au dernier commit)
```bash
git commit --amend
git commit --amend --no-edit
```

### Voir dernier commit
```bash
git show
```

---

## Git - Histoire

### Log oneline
```bash
git log --oneline
```

### Log avec graph
```bash
git log --oneline --graph --all
```

### Log stat (fichiers changés)
```bash
git log --stat
```

### Log avec diffs
```bash
git log -p
```

### Log d'un fichier
```bash
git log src/launcher.rs
```

### Voir qui a changé quoi
```bash
git blame src/launcher.rs
```

---

## Git - Diffs

### Diff non staged
```bash
git diff
```

### Diff staged
```bash
git diff --cached
```

### Diff entre branches
```bash
git diff main feature/DEV-002
```

### Diff fichier spécifique
```bash
git diff src/launcher.rs
```

---

## Git - Remote

### Voir remotes
```bash
git remote -v
```

### Ajouter remote
```bash
git remote add origin https://github.com/Bermotard/initium.git
```

### Changer URL remote
```bash
git remote set-url origin https://github.com/Bermotard/initium.git
```

### Voir info remote
```bash
git remote show origin
```

---

## Git - Push & Pull

### Push branche
```bash
git push origin feature/DEV-002-structure
```

### Push avec tracking
```bash
git push -u origin feature/DEV-002-structure
```

### Push force (dangereux!)
```bash
git push -f origin feature/DEV-002-structure
```

### Pull changes
```bash
git pull origin main
```

### Fetch (sans merge)
```bash
git fetch origin
```

---

## Git - Merge & Rebase

### Merge branche
```bash
git checkout main
git merge feature/DEV-002-structure
```

### Merge sans fast-forward
```bash
git merge --no-ff feature/DEV-002-structure
```

### Rebase (alternative à merge)
```bash
git rebase main
```

### Abort merge en conflit
```bash
git merge --abort
```

---

## Rust - Diagnostic

### Backtrace complet
```bash
RUST_BACKTRACE=full cargo test
```

### Debug build avec symbols
```bash
cargo build --debug-info=full
```

---

## VS Code - Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Format document | `Shift+Alt+F` |
| Find | `Ctrl+F` |
| Replace | `Ctrl+H` |
| Go to file | `Ctrl+P` |
| Go to line | `Ctrl+G` |
| Terminal | `Ctrl+`` |
| Command palette | `Ctrl+Shift+P` |
| Run test | `Ctrl+Shift+U` |
| Debug | `F5` |

---

## Workflow Complet Sprint 1

```bash
# 1. Ouvrir le projet
cd /home/bernard/initium-dashboard
code .

# 2. Vérifier
cargo check
cargo test

# 3. Créer branche pour DEV-002
git checkout -b feature/DEV-002-structure-modules

# 4. Développer (ajouter le code)
# ... utiliser VS Code ...

# 5. Vérifier qualité
cargo fmt
cargo clippy
cargo test

# 6. Commit
git add .
git commit -m "feat(DEV-002): implement module structure"

# 7. Push
git push -u origin feature/DEV-002-structure-modules

# 8. Sur GitHub: Créer Pull Request
# 9. Après review: Merge dans main

# 10. Revenir à main
git checkout main
git pull origin main

# 11. Supprimer branche locale
git branch -d feature/DEV-002-structure-modules
```

---

## Fichiers Importants

```
/home/bernard/initium-dashboard/
├── Cargo.toml              # Dépendances
├── src/
│   ├── lib.rs              # Exports
│   ├── main.rs             # Entry point
│   ├── launcher.rs         # À implémenter (DEV-003)
│   ├── config.rs           # À implémenter (DEV-004)
│   ├── ui/servo_engine.rs  # À implémenter (DEV-006)
│   └── system/
│       ├── platform.rs     # À implémenter (DEV-005)
│       └── autostart.rs    # À implémenter (DEV-007)
├── tests/                  # Tests (DEV-008)
├── docs/
│   └── Initium_Phase2_Sprint1.html  # À LIRE!
└── config.json             # Configuration exemple
```

---

## Resources

- Rust Book: https://doc.rust-lang.org/book/
- Cargo: https://doc.rust-lang.org/cargo/
- Initium GitHub: https://github.com/Bermotard/initium
- VS Code: https://code.visualstudio.com/

---

**Bon développement! 🚀**
