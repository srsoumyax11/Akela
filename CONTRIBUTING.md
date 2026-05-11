# Contributing to Akela

First off, thank you for considering contributing to Akela! It's people like you who make Akela such a great tool for the community.

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) & [Bun](https://bun.sh/)
- Windows 10/11 (Native APIs used)

### Development Setup
1. Clone the repository: `git clone https://github.com/srsoumyax11/Akela.git`
2. Install dependencies: `bun install`
3. Start the development environment: `bun tauri dev`

## 🌿 Branching Strategy
- `main`: Production-ready code only.
- `develop`: Main development branch.
- `feature/[name]`: New features or enhancements.
- `bugfix/[name]`: Bug fixes.

## 📝 Coding Standards
- **Rust**: Use `cargo fmt` and `cargo clippy`.
- **TypeScript/React**: Use `prettier` and `eslint`.
- Keep components small and focused.
- Ensure all native Win32 calls are properly documented.

## 💬 Commit Style
We use [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` for new features.
- `fix:` for bug fixes.
- `docs:` for documentation changes.
- `refactor:` for code restructuring.
- `perf:` for performance improvements.

## 📬 Pull Request Process
1. Create a new branch from `develop`.
2. Ensure your code builds and passes all lints.
3. Update documentation if necessary.
4. Open a PR against the `develop` branch.
5. Provide a clear description of the changes and any testing performed.

## 🏷️ Issue Labels
- `bug`: Something isn't working.
- `enhancement`: New feature or request.
- `good first issue`: Beginner-friendly tasks.
- `performance`: Latency or resource optimization.
- `audio`: Audio engine specific.
- `overlay`: UI/Overlay specific.

---

By contributing to this project, you agree to abide by the [Code of Conduct](CODE_OF_CONDUCT.md).
