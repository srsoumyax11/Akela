# Akela Rules

## Workflow

1. Read related documentation before starting work
2. Make a short implementation plan
3. Take approval before major changes
4. Check current git branch before starting any work
5. Create/switch to proper branch before implementation
6. Implement feature
7. Test properly
8. Get owner confirmation
9. Make proper git commit
10. Use the Affilate files always as reference

---

## Code Rules

- Keep code modular
- Small reusable components/functions
- Avoid large files
- Do not mix UI and business logic
- Follow existing architecture

---

## Project Tracking

### log.txt
Update briefly after work.

Example:
- Added draggable overlay
- Fixed transparency issue

---

### todo.md
Track:
- skipped tasks
- future improvements
- technical debt

---

## Git Rules

### Branches

- main → stable code
- develop → active development
- feature/* → new features
- bugfix/* → fixes

---

### Branch Checking

Before writing code:
- check current branch first
- never work directly on main
- use proper feature/bugfix branch

Example:
feature/overlay-prototype

---

### Commit Format

Examples:
- feat: add overlay dragging
- fix: resolve overlay flicker
- docs: update roadmap

---

## Documentation

When changing architecture or flow:
- update related docs
- update roadmap if needed
- update todo.md if work is postponed