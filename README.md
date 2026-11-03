# Lipsa Agent Skills Bundle

One repo for ten agent-skill projects, pulled in as git submodules, with a small Rust CLI that installs them into Codex and other Agent Skills hosts.

| Bundle | What it does | Skills | Upstream | License |
|---|---|---|---|---|
| **REA** | Reverse Engineer Anything: investigate an app or binary and rebuild a feature | `reverse-engineer-anything` | [morluto/rea](https://github.com/morluto/rea) | MIT |
| **Ponytail** | Keeps the agent from over-building: reuse code, stdlib and platform features first | `ponytail`, `ponytail-review`, `ponytail-audit`, `ponytail-debt`, `ponytail-gain`, `ponytail-help` | [DietrichGebert/ponytail](https://github.com/DietrichGebert/ponytail) | MIT |
| **Hallmark** | Anti-AI-slop UI: themes and pre-ship gates for pages that look designed | `hallmark` | [Nutlope/hallmark](https://github.com/Nutlope/hallmark) | MIT |
| **Humanizer** | Removes signs of AI-generated writing from text | `humanizer` | [blader/humanizer](https://github.com/blader/humanizer) | MIT |
| **Impeccable** | Design vocabulary and commands: audit, critique, polish, distill… | `impeccable` | [pbakaus/impeccable](https://github.com/pbakaus/impeccable) | Apache 2.0 |
| **UI UX Pro Max** | Design intelligence: styles, palettes, typography, stack guidelines | `ui-ux-pro-max`, `design`, `design-system`, `ui-styling`, `brand`, `banner-design`, `slides` | [nextlevelbuilder/ui-ux-pro-max-skill](https://github.com/nextlevelbuilder/ui-ux-pro-max-skill) | MIT |
| **Rust Skills** | 265 Rust rules: ownership, errors, async, unsafe, API design, performance | `rust-skills` | [leonardomso/rust-skills](https://github.com/leonardomso/rust-skills) | MIT |
| **Engineering** | Addy Osmani's full dev lifecycle: spec, plan, TDD, debug, review, secure, ship | 25 skills, e.g. `spec-driven-development`, `test-driven-development`, `security-and-hardening` (`list` shows all) | [addyosmani/agent-skills](https://github.com/addyosmani/agent-skills) | MIT |
| **Anthropic** | Official skills for building MCP servers, new skills and web app tests | `mcp-builder`, `skill-creator`, `webapp-testing` | [anthropics/skills](https://github.com/anthropics/skills) | Apache 2.0 |
| **Vercel** | React and React Native performance and composition guidance | `react-best-practices`, `composition-patterns`, `react-view-transitions`, `react-native-skills` | [vercel-labs/agent-skills](https://github.com/vercel-labs/agent-skills) | MIT |

## Install

**Windows, easiest:** download [`setup-windows.bat`](setup-windows.bat) and double-click it. It downloads the bundle into your user folder (or updates it) and installs every skill. Run it again any time to update.

Needs git and [Rust](https://rust-lang.org/tools/install).

```bash
git clone --recurse-submodules --shallow-submodules https://github.com/lipsapriyadarsinee101-oss/Lipsaagent-skills-bundle.git
cd Lipsaagent-skills-bundle
cargo run --release -- install            # all 50 skills into ~/.codex/skills
```

Restart your agent afterwards so it picks the skills up.

Installs are symlinks into this clone, so `git pull` updates every agent at once. Keep the clone where it is (or reinstall with `--copy`).

### Options

```bash
cargo run --release -- list                                  # what's in the bundle
cargo run --release -- install --agent all                   # Codex and ~/.agents/skills
cargo run --release -- install --agent codex
cargo run --release -- install --only hallmark,humanizer     # by bundle or skill name
cargo run --release -- install --project                     # into ./.codex/skills of the current folder
cargo run --release -- install --copy                        # copy instead of symlink (default on Windows)
cargo run --release -- uninstall --agent all                 # removes only what this tool installed
cargo run --release -- doctor                                # check submodules and installs
```

| `--agent` | Folder | Used by |
|---|---|---|
| `codex` (default) | `~/.codex/skills` | OpenAI Codex |
| `agents` | `~/.agents/skills` | OpenCode, Antigravity, Gemini CLI and other Agent Skills hosts |

The installer never overwrites an existing folder or a link pointing elsewhere. Copied installs carry an ownership marker. Reinstalling an unchanged owned copy is a no-op. Uninstall removes only an owned copy that still matches the source tree; local edits and additional files are preserved. Existing copies made by older versions have no marker and are preserved. Conflicts, unknown selectors and missing sources return a failing exit code.

### Recorded installation demonstration

`cargo test` includes CLI tests that install, repeat-install and uninstall into both supported agent project folders. They also check the default target, removed options, conflict handling, local-file preservation and invalid selectors. Unix tests additionally exercise symlinks. Windows tests use copy mode without requiring administrator privileges.

Record the demonstration yourself (Python 3 required):

```bash
cargo build
python scripts/demo_install.py --binary target/debug/skills-bundle --output demo-transcript.txt
# Windows: use target/debug/skills-bundle.exe
```

The demonstration uses a temporary project, checks the files at every stage and leaves global installations untouched. CI runs it on Windows, Linux and macOS and uploads a transcript per operating system. These checks verify installation into supported folders; they do not verify that each agent application executes every upstream skill. REA's MCP server remains a separate optional setup.

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
vendor/          the ten upstream repos (git submodules, untouched)
src/main.rs      the installer (std only, no dependencies)
.github/         CI: builds, tests and checks every skill path
```

## Credits and licenses

Every skill belongs to its authors and keeps its own license inside its submodule (`vendor/*/LICENSE`, per-skill `LICENSE.txt` in `vendor/anthropic-skills`, the `license:` field in each Vercel `SKILL.md`, plus `vendor/impeccable/NOTICE.md` and `vendor/rea/THIRD_PARTY_NOTICES.md`). This repo does not modify them. Only openly licensed skills are wired into the installer: Anthropic's docx/pdf/pptx/xlsx and Vercel's unlicensed skills stay in their submodules but are not installed. `frontend-design` is left out because Hallmark and Impeccable already build on it. The installer code in `src/` is MIT, see [LICENSE](LICENSE).
