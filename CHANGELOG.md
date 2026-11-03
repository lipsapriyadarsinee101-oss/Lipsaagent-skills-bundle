# Changelog

## 0.2.0

- Codex is now the default installation target. `--agent all` installs into Codex and the shared Agent Skills folder.
- Removed the Claude target and Claude API skill from the installer. Existing installations are not automatically deleted.
- Added ownership protection, reliable failure exit codes, regression tests and recorded installation demos on Windows, Linux and macOS.

### Migration

Use `--agent codex` or `--agent agents` for an explicit target. The default command now installs into `~/.codex/skills`. The bundle contains 50 installable skills. Upstream source directories, attribution and licenses remain unchanged.
