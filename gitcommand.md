# Git Branching Cheat Sheet — Akela

Recommended workflow:

```text
main
 └── develop
      ├── feature/*
      ├── bugfix/*
      ├── hotfix/*
      └── experiment/*
```

---

# Branch Purpose

| Branch         | Purpose                      |
| -------------- | ---------------------------- |
| `main`         | Production-ready stable code |
| `develop`      | Active integration branch    |
| `feature/*`    | New features                 |
| `bugfix/*`     | Normal bug fixes             |
| `hotfix/*`     | Emergency production fixes   |
| `experiment/*` | Unsafe testing/prototypes    |

---

# Initial Setup

# 1. Initialize Repo

```bash
git init
```

Add remote:

```bash
git remote add origin <repo-url>
```

---

# 2. Create Main Branch

```bash
git checkout -b main
```

Initial commit:

```bash
git add .
git commit -m "Initial commit"
```

Push:

```bash
git push -u origin main
```

---

# 3. Create Develop Branch

```bash
git checkout -b develop
```

Push:

```bash
git push -u origin develop
```

---

# Daily Workflow

# Always Start From Develop

Update develop first:

```bash
git checkout develop
git pull origin develop
```

---

# Create Feature Branch

Example:

```bash
git checkout -b feature/capsule-overlay
```

OR:

```bash
git switch -c feature/capsule-overlay
```

---

# Work Normally

Check changes:

```bash
git status
```

Add files:

```bash
git add .
```

Commit:

```bash
git commit -m "Add capsule overlay animations"
```

---

# Push Feature Branch

```bash
git push -u origin feature/capsule-overlay
```

---

# Merge Back Into Develop

After PR/review:

```bash
git checkout develop
git pull origin develop
git merge feature/capsule-overlay
```

Push:

```bash
git push origin develop
```

Delete local feature branch:

```bash
git branch -d feature/capsule-overlay
```

Delete remote branch:

```bash
git push origin --delete feature/capsule-overlay
```

---

# Bugfix Flow

Create:

```bash
git checkout develop
git pull
git checkout -b bugfix/audio-desync
```

Commit:

```bash
git commit -m "Fix audio stream desync"
```

Merge back to develop.

---

# Hotfix Flow (Production Emergency)

# Important:

Hotfix starts from `main`, NOT develop.

```bash
git checkout main
git pull origin main
git checkout -b hotfix/crash-on-launch
```

Fix issue.

Commit:

```bash
git commit -m "Fix startup crash"
```

Merge into main:

```bash
git checkout main
git merge hotfix/crash-on-launch
git push origin main
```

Also merge into develop:

```bash
git checkout develop
git merge hotfix/crash-on-launch
git push origin develop
```

Delete hotfix branch.

---

# Before Starting Work Every Day

Always:

```bash
git checkout develop
git pull origin develop
```

Then create fresh feature branch.

---

# If Something Is Wrong

# 1. Check Current Branch

```bash
git branch
```

Current branch has `*`.

---

# 2. Check Changes

```bash
git status
```

---

# 3. View Commit History

Compact:

```bash
git log --oneline --graph --all
```

Very important command.

---

# 4. Undo Unstaged Changes

Discard file changes:

```bash
git restore filename
```

Discard all:

```bash
git restore .
```

---

# 5. Unstage Files

```bash
git restore --staged .
```

---

# 6. Undo Last Commit (Keep Files)

```bash
git reset --soft HEAD~1
```

---

# 7. Completely Remove Last Commit

DANGEROUS:

```bash
git reset --hard HEAD~1
```

---

# 8. Recover Deleted Work

Git keeps reflog history:

```bash
git reflog
```

Then restore:

```bash
git reset --hard <commit-id>
```

---

# 9. Stash Temporary Work

Save current changes:

```bash
git stash
```

Restore later:

```bash
git stash pop
```

View stashes:

```bash
git stash list
```

---

# 10. Fix Merge Conflicts

When conflict happens:

```bash
git status
```

Open conflicted files.

You will see:

```text
<<<<<<< HEAD
your code
=======
incoming code
>>>>>>> branch
```

Manually fix.

Then:

```bash
git add .
git commit
```

---

# Useful Professional Commands

# View Remote Branches

```bash
git branch -a
```

---

# Rename Branch

```bash
git branch -m old-name new-name
```

---

# Fetch Remote Updates

```bash
git fetch
```

---

# Pull Safely With Rebase

Cleaner history:

```bash
git pull --rebase
```

---

# Compare Changes

Working tree:

```bash
git diff
```

Between branches:

```bash
git diff develop..feature/capsule-overlay
```

---

# Clean Untracked Files

DANGEROUS:

```bash
git clean -fd
```

---

# Recommended Commit Style

Good:

```text
feat: add floating capsule overlay
fix: repair audio engine race condition
refactor: split ai runtime services
style: improve glassmorphism shadows
perf: reduce overlay render latency
```

---

# Recommended Akela Workflow

```text
main
  Stable Releases Only

develop
  Active Development

feature/*
  New UI/AI features

bugfix/*
  Normal fixes

hotfix/*
  Emergency production fixes

experiment/*
  Unsafe prototypes
```

---

# Recommended Protected Branches

Protect on GitHub:

| Branch    | Protection       |
| --------- | ---------------- |
| `main`    | No direct pushes |
| `develop` | PR required      |

---

# Best Practice Rules

# NEVER

```text
Commit directly to main
```

---

# NEVER

```text
Develop on develop branch directly
```

Always create feature branches.

---

# ALWAYS

```text
Pull before starting work
```

---

# ALWAYS

```text
Small commits > giant commits
```

---

# Recommended Git Aliases

Add:

```bash
git config --global alias.st status
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.cm commit
git config --global alias.lg "log --oneline --graph --all"
```

Now:

```bash
git lg
```

becomes powerful history view.

---

# Ideal Real Example

```text
main
 └── develop
      ├── feature/overlay-ui
      ├── feature/voice-engine
      ├── bugfix/window-focus
      ├── experiment/gpu-renderer
      └── hotfix/startup-crash
```

This structure scales well for:

* solo development
* startup teams
* enterprise engineering
* AI systems
* desktop infrastructure apps like Akela
