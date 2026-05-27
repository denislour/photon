# Photon Git Skill

## Commit Convention

```
<type>(<scope>): <description>
```

### Types

| Type       | Purpose                               |
| ---------- | ------------------------------------- |
| `feat`     | New feature                           |
| `fix`      | Bug fix                               |
| `chore`    | Build, deps, config                   |
| `docs`     | Documentation                         |
| `refactor` | Code restructure (no behavior change) |
| `style`    | Formatting, whitespace                |

### Scopes

| Scope  | Area                       |
| ------ | -------------------------- |
| `be`   | Backend (api/)             |
| `fe`   | Frontend (app/)            |
| `root` | Workspace config, Justfile |

### Rules

- One feature per commit
- One-line description (under 72 chars)
- Use imperative mood ("add" not "added")
- No emoji, no markdown in subject line

### Examples

```
feat(be): add POST /api/media upload with R2 storage
feat(fe): add gallery grid with masonry layout
chore(root): add Justfile with dev, build, deploy recipes
fix(be): handle empty file in multipart upload
refactor(fe): extract API client into utils module
```

## Workflow

1. Write code following the skill guides
2. Run `cargo fmt` + `cargo clippy -- -D warnings`
3. Group related changes into one commit
4. Use semantic commit message
