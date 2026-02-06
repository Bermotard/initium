# Guide de Contribution

Merci de votre intérêt pour Initium!

## GitHub Flow

1. Fork le repository
2. Créez une branche: `git checkout -b feature/ma-feature`
3. Commit: `git commit -am 'feat: description'`
4. Push: `git push origin feature/ma-feature`
5. Pull Request

## Convention Commits

- `feat:` nouvelle fonctionnalité
- `fix:` correction de bug
- `docs:` documentation
- `test:` tests
- `chore:` maintenance
- `refactor:` refactorisation

## Quality Requirements

```bash
cargo test          # Tests
cargo clippy        # Lint (0 warnings)
cargo fmt           # Format (checked)
```

Coverage: ≥80%

Merci!
