# Lipsa Agent Skills Bundle

One repo for six agent-skill projects, pulled in as git submodules, with a small Rust CLI that installs them into Claude Code, Codex and other Agent Skills hosts.

| Bundle | What it does | Skills | Upstream | License |
|---|---|---|---|---|
| **REA** | Reverse Engineer Anything: investigate an app or binary and rebuild a feature | `reverse-engineer-anything` | [morluto/rea](https://github.com/morluto/rea) | MIT |
| **Ponytail** | Keeps the agent from over-building: reuse code, stdlib and platform features first | `ponytail`, `ponytail-review`, `ponytail-audit`, `ponytail-debt`, `ponytail-gain`, `ponytail-help` | [DietrichGebert/ponytail](https://github.com/DietrichGebert/ponytail) | MIT |
| **Hallmark** | Anti-AI-slop UI: themes and pre-ship gates for pages that look designed | `hallmark` | [Nutlope/hallmark](https://github.com/Nutlope/hallmark) | MIT |
| **Humanizer** | Removes signs of AI-generated writing from text | `humanizer` | [blader/humanizer](https://github.com/blader/humanizer) | MIT |
| **Impeccable** | Design vocabulary and commands: audit, critique, polish, distill… | `impeccable` | [pbakaus/impeccable](https://github.com/pbakaus/impeccable) | Apache 2.0 |
| **UI UX Pro Max** | Design intelligence: styles, palettes, typography, stack guidelines | `ui-ux-pro-max`, `design`, `design-system`, `ui-styling`, `brand`, `banner-design`, `slides` | [nextlevelbuilder/ui-ux-pro-max-skill](https://github.com/nextlevelbuilder/ui-ux-pro-max-skill) | MIT |

## Install

Needs git and [Rust](https://rust-lang.org/tools/install).

```bash
git clone --recurse-submodules --shallow-submodules https://github.com/lipsapriyadarsinee101-oss/Lipsaagent-skills-bundle.git
cd Lipsaagent-skills-bundle
cargo run --release -- install            # all 17 skills into ~/.claude/skills
```

Restart your agent afterwards so it picks the skills up.

Installs are symlinks into this clone, so `git pull` updates every agent at once. Keep the clone where it is (or reinstall with `--copy`).

### Options

```bash
cargo run --release -- list                                  # what's in the bundle
cargo run --release -- install --agent all                   # Claude Code, Codex and ~/.agents/skills
cargo run --release -- install --agent codex
cargo run --release -- install --only hallmark,humanizer     # by bundle or skill name
cargo run --release -- install --project                     # into ./.claude/skills of the current folder
cargo run --release -- install --copy                        # copy instead of symlink (default on Windows)
cargo run --release -- uninstall --agent all                 # removes only what this tool installed
cargo run --release -- doctor                                # check submodules and installs
```

| `--agent` | Folder | Used by |
|---|---|---|
| `claude` (default) | `~/.claude/skills` | Claude Code (OpenCode also reads it) |
| `codex` | `~/.codex/skills` | OpenAI Codex |
| `agents` | `~/.agents/skills` | OpenCode, Antigravity, Gemini CLI and other Agent Skills hosts |

The installer never overwrites a skill folder you made yourself; it reports it and moves on.

### Extra setup for REA

The REA skill drives REA's MCP server, which wraps Ghidra/Hopper. Install that once:

```bash
npx -y rea-agents setup
```

## Updating

```bash
git pull
git submodule update --remote --depth 1      # move every skill to its latest upstream commit
cargo run --release -- doctor
```

Commit the updated submodule pointers if you want the bundle to remember the new versions.

## Adding another skill repo

```bash
git submodule add --depth 1 https://github.com/OWNER/REPO.git vendor/NAME
```

Then add a line for each skill folder to `SKILLS` in `src/main.rs` and run `cargo test`. The tests fail if any listed path is missing a `SKILL.md` or two skills share a name.

## Layout

```
vendor/          the six upstream repos (git submodules, untouched)
src/main.rs      the installer (std only, no dependencies)
.github/         CI: builds, tests and checks every skill path
```

## Credits and licenses

Every skill belongs to its authors and keeps its own license inside its submodule (`vendor/*/LICENSE`, plus `vendor/impeccable/NOTICE.md` and `vendor/rea/THIRD_PARTY_NOTICES.md`). This repo does not modify them. The installer code in `src/` is MIT, see [LICENSE](LICENSE).
