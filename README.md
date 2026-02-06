# 🚀 Initium - Dashboard Intelligent

Un tableau de bord intelligent multi-plateforme pour lancer vos applications et sites web préférés au démarrage de votre ordinateur.

## 🎯 Caractéristiques

- ✅ Interface élégante et responsive avec Servo
- ✅ Grille d'icônes pour lancer applications et sites web
- ✅ Multi-plateforme: Linux, Windows, macOS
- ✅ Gestion de configuration via JSON
- ✅ Autostart automatique au démarrage du système
- ✅ Import/Export de configurations
- ✅ Architecture modulaire et extensible

## 🛠️ Technologie

- **Backend:** Rust 1.70+ avec Servo
- **Frontend:** HTML/CSS/JavaScript Vanilla
- **API:** REST interne (Hyper)
- **Build:** Cargo

## 📦 Installation

Voir [Installation Guide](docs/INSTALLATION.md)

## 🔨 Développement

### Prérequis
- Rust 1.70+ ([Install](https://rustup.rs/))
- Cargo

### Quick Start
```bash
git clone https://github.com/initium-dashboard/initium.git
cd initium
cargo build --release
cargo test
```

### Commands
```bash
cargo build              # Build debug
cargo build --release   # Build production
cargo test              # Run tests
cargo clippy            # Lint
cargo fmt               # Format code
cargo doc --open        # Generate docs
```

## 📚 Documentation

- [USER_GUIDE.md](docs/USER_GUIDE.md) - Guide utilisateur
- [INSTALLATION.md](docs/INSTALLATION.md) - Instructions installation
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Architecture technique
- [API.md](docs/API.md) - API REST documentation
- [CONTRIBUTING.md](CONTRIBUTING.md) - Guide contribution

## 🤝 Contributions

Les contributions sont bienvenues! 
- Fork le repository
- Create une branche: `git checkout -b feature/my-feature`
- Commit: `git commit -am 'feat: add feature'`
- Push: `git push origin feature/my-feature`
- Pull Request

Consultez [CONTRIBUTING.md](CONTRIBUTING.md) pour plus de détails.

## 📄 Licence

MIT License - voir [LICENSE](LICENSE)

## 📞 Support

- **Issues:** https://github.com/initium-dashboard/initium/issues
- **Discussions:** https://github.com/initium-dashboard/initium/discussions

---

**Initium v0.1.0-alpha** | [Changelog](CHANGELOG.md)
