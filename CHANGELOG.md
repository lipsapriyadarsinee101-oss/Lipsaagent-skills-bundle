# Changelog

## Unreleased

- Added the read-only Rust `verify` command to detect missing or changed installed files, extra files and unexpected links, with project/agent/skill filters and a failing exit code for differences.

## 0.2.0

- Codex is now the default installation target. `--agent all` installs into Codex and the shared Agent Skills folder.
- Removed the Claude Code installation target. All 51 skills, including the Claude API development skill, remain available for Codex and shared Agent Skills hosts. Existing installations are not automatically deleted.
- Added ownership protection, reliable failure exit codes, regression tests and recorded installation demos on Windows, Linux and macOS.

### Migration

Use `--agent codex` or `--agent agents` for an explicit target. The default command now installs into `~/.codex/skills`. The bundle contains 51 installable skills. Upstream source directories, attribution and licenses remain unchanged.
