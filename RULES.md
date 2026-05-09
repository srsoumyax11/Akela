# Akela Project Development Rules

## 🌿 Branching Strategy
- **`main`**: Production-ready code. Only merged from `develop` via PR.
- **`develop`**: Integration branch for features.
- **`feature/[name]`**: For new features. Branched from `develop`.
- **`fix/[name]`**: For bug fixes. Branched from `develop`.

All branches must be properly tested before merging into the main branch.

## 🚀 Release Process
1. Update the version in `src-tauri/tauri.conf.json`.
2. Commit and push the changes to `develop`.
3. Create a git tag following semantic versioning (e.g., `v0.1.0`):
   ```bash
   git tag -a v0.1.0 -m "Release version 0.1.0"
   git push origin v0.1.0
   ```
4. The GitHub Workflow will automatically:
   - Build the Windows app (MSI and EXE).
   - Create a draft release on GitHub with the artifacts.
5. Review the draft release on GitHub, add release notes, and publish it.
