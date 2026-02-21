# 🚀 Initium - Dashboard Intelligent
Un tableau de bord intelligent multi-plateforme pour lancer vos applications et sites web préférés au démarrage de votre ordinateur.

## 🎯 Caractéristiques
- ✅ Interface élégante et responsive avec grille d'icônes
- ✅ Grille d'icônes pour lancer applications et sites web
- ✅ Multi-plateforme: Linux, Windows, macOS
- ✅ Gestion de configuration via JSON
- ✅ Import/Export de configurations
- ✅ Upload d'icônes personnalisées
- ✅ Architecture modulaire et extensible

## 🛠️ Technologie
- **Backend:** Rust 1.70+ avec Tauri
- **Frontend:** React avec CSS moderne
- **Desktop:** Tauri 2.x
- **Build:** Cargo + npm

## 📦 Installation

### Linux Mint / Ubuntu / Debian

1. **Téléchargez le package:**
   - Allez sur [Releases](https://github.com/Bermotard/initium/releases)
   - Téléchargez `initium_0.1.0-alpha_amd64.deb`

2. **Double-cliquez sur le fichier .deb**
   - Ou installez via terminal:
```bash
   sudo dpkg -i initium_0.1.0-alpha_amd64.deb
```

3. **Lancez Initium:**
   - Via menu Applications (cherchez "Initium Dashboard")
   - Ou terminal: `initium`

### Windows (Coming Soon)
```bash
initium_0.1.0-alpha_x64.msi
```

### macOS (Coming Soon)
```bash
Initium_0.1.0-alpha_amd64.dmgcat >> README.md << 'EOF'

## 🔧 Générer le Package .deb

### Prérequis
```bash
sudo apt install dpkg-deb imagemagick
```

## 🔨 Développement

### Prérequis
- Rust 1.70+ ([Install](https://rustup.rs/))
- Node.js 18+ ([Install](https://nodejs.org/))
- Cargo

### Quick Start
```bash
git clone https://github.com/Bermotard/initium.git
cd initium

# Build frontend
cd frontend
npm install
npm run build

# Build backend
cd ../src-tauri
cargo tauri build
```

### Development Mode
```bash
# Terminal 1: Frontend dev server
cd frontend
npm run dev

# Terminal 2: Tauri dev
cd src-tauri
cargo tauri dev
```

### Commands
```bash
cargo test              # Run unit tests
cargo tarpaulin        # Check code coverage
cargo clippy           # Lint
cargo fmt              # Format code
```

## 📚 Documentation
- [USER_GUIDE.md](docs/USER_GUIDE.md) - Guide utilisateur
- [INSTALLATION.md](docs/INSTALLATION.md) - Instructions détaillées
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Architecture technique
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
- **Issues:** https://github.com/Bermotard/initium/issues
- **Discussions:** https://github.com/Bermotard/initium/discussions

---
**Initium v0.1.0-alpha** | [Changelog](CHANGELOG.md)

## 🔧 Générer le Package .deb

### Prérequis
```bash
sudo apt install dpkg-deb imagemagick
```

### Build du .deb
```bash
# 1. Build le binaire Linux
cd src-tauri
cargo tauri build

# 2. Créer la structure du paquet
mkdir -p packaging/deb/{DEBIAN,usr/bin,usr/share/applications,usr/share/icons/hicolor/256x256/apps}

# 3. Copier les fichiers
cp target/release/initium packaging/deb/usr/bin/
chmod 755 packaging/deb/usr/bin/initium

# 4. Créer les scripts
cat > packaging/deb/DEBIAN/control << 'CONTROL'
Package: initium
Version: 0.1.0-alpha
Architecture: amd64
Maintainer: Bernard <bernard@initium.dev>
Description: Initium Dashboard - Application Launcher
 Manage and launch your applications from a beautiful dashboard.
Homepage: https://github.com/Bermotard/initium
CONTROL

cat > packaging/deb/DEBIAN/postinst << 'POSTINST'
#!/bin/bash
set -e
update-desktop-database /usr/share/applications 2>/dev/null || true
gtk-update-icon-cache /usr/share/icons/hicolor 2>/dev/null || true
POSTINST
chmod 755 packaging/deb/DEBIAN/postinst

cat > packaging/deb/DEBIAN/prerm << 'PRERM'
#!/bin/bash
set -e
exit 0
PRERM
chmod 755 packaging/deb/DEBIAN/prerm

cat > packaging/deb/usr/share/applications/initium.desktop << 'DESKTOP'
[Desktop Entry]
Version=1.0
Type=Application
Name=Initium Dashboard
Comment=Application Launcher
Exec=/usr/bin/initium
Icon=initium
Categories=Utility;
Terminal=false
DESKTOP

# 5. Créer l'icône
convert -size 256x256 xc:blue -fill white -gravity center -pointsize 40 -annotate +0+0 "I" packaging/deb/usr/share/icons/hicolor/256x256/apps/initium.png

# 6. Build le .deb
dpkg-deb --build packaging/deb initium_0.1.0-alpha_amd64.deb

# 7. Tester l'installation
sudo dpkg -i initium_0.1.0-alpha_amd64.deb
initium
```

