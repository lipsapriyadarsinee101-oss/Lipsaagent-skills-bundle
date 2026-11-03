//! skills-bundle: link the vendored agent skills into each agent's skills folder.
//!
//! Usage:
//!   skills-bundle list
//!   skills-bundle install   [--agent codex|agents|all] [--project] [--copy] [--only NAME,...]
//!   skills-bundle uninstall [--agent ...] [--project] [--only NAME,...]
//!   skills-bundle doctor

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// One installable skill: (bundle, skill folder name, path inside the repo).
struct Skill {
    bundle: &'static str,
    name: &'static str,
    path: &'static str,
}

const fn s(bundle: &'static str, name: &'static str, path: &'static str) -> Skill {
    Skill { bundle, name, path }
}

const SKILLS: &[Skill] = &[
    s("rea", "reverse-engineer-anything", "vendor/rea/.agents/skills/reverse-engineer-anything"),
    s("ponytail", "ponytail", "vendor/ponytail/skills/ponytail"),
    s("ponytail", "ponytail-review", "vendor/ponytail/skills/ponytail-review"),
    s("ponytail", "ponytail-audit", "vendor/ponytail/skills/ponytail-audit"),
    s("ponytail", "ponytail-debt", "vendor/ponytail/skills/ponytail-debt"),
    s("ponytail", "ponytail-gain", "vendor/ponytail/skills/ponytail-gain"),
    s("ponytail", "ponytail-help", "vendor/ponytail/skills/ponytail-help"),
    s("hallmark", "hallmark", "vendor/hallmark/skills/hallmark"),
    s("humanizer", "humanizer", "vendor/humanizer"),
    s("impeccable", "impeccable", "vendor/impeccable/.claude/skills/impeccable"),
    s("ui-ux-pro-max", "ui-ux-pro-max", "vendor/ui-ux-pro-max/.claude/skills/ui-ux-pro-max"),
    s("ui-ux-pro-max", "design", "vendor/ui-ux-pro-max/.claude/skills/design"),
    s("ui-ux-pro-max", "design-system", "vendor/ui-ux-pro-max/.claude/skills/design-system"),
    s("ui-ux-pro-max", "ui-styling", "vendor/ui-ux-pro-max/.claude/skills/ui-styling"),
    s("ui-ux-pro-max", "brand", "vendor/ui-ux-pro-max/.claude/skills/brand"),
    s("ui-ux-pro-max", "banner-design", "vendor/ui-ux-pro-max/.claude/skills/banner-design"),
    s("ui-ux-pro-max", "slides", "vendor/ui-ux-pro-max/.claude/skills/slides"),
    s("rust", "rust-skills", "vendor/rust-skills"),
    s("anthropic", "mcp-builder", "vendor/anthropic-skills/skills/mcp-builder"),
    s("anthropic", "skill-creator", "vendor/anthropic-skills/skills/skill-creator"),
    s("anthropic", "webapp-testing", "vendor/anthropic-skills/skills/webapp-testing"),
    s("vercel", "react-best-practices", "vendor/vercel-agent-skills/skills/react-best-practices"),
    s("vercel", "composition-patterns", "vendor/vercel-agent-skills/skills/composition-patterns"),
    s("vercel", "react-view-transitions", "vendor/vercel-agent-skills/skills/react-view-transitions"),
    s("vercel", "react-native-skills", "vendor/vercel-agent-skills/skills/react-native-skills"),
    s("engineering", "using-agent-skills", "vendor/addy-agent-skills/skills/using-agent-skills"),
    s("engineering", "interview-me", "vendor/addy-agent-skills/skills/interview-me"),
    s("engineering", "idea-refine", "vendor/addy-agent-skills/skills/idea-refine"),
    s("engineering", "spec-driven-development", "vendor/addy-agent-skills/skills/spec-driven-development"),
    s("engineering", "planning-and-task-breakdown", "vendor/addy-agent-skills/skills/planning-and-task-breakdown"),
    s("engineering", "incremental-implementation", "vendor/addy-agent-skills/skills/incremental-implementation"),
    s("engineering", "constraint-driven-development", "vendor/addy-agent-skills/skills/constraint-driven-development"),
    s("engineering", "doubt-driven-development", "vendor/addy-agent-skills/skills/doubt-driven-development"),
    s("engineering", "source-driven-development", "vendor/addy-agent-skills/skills/source-driven-development"),
    s("engineering", "test-driven-development", "vendor/addy-agent-skills/skills/test-driven-development"),
    s("engineering", "context-engineering", "vendor/addy-agent-skills/skills/context-engineering"),
    s("engineering", "api-and-interface-design", "vendor/addy-agent-skills/skills/api-and-interface-design"),
    s("engineering", "frontend-ui-engineering", "vendor/addy-agent-skills/skills/frontend-ui-engineering"),
    s("engineering", "browser-testing-with-devtools", "vendor/addy-agent-skills/skills/browser-testing-with-devtools"),
    s("engineering", "debugging-and-error-recovery", "vendor/addy-agent-skills/skills/debugging-and-error-recovery"),
    s("engineering", "code-review-and-quality", "vendor/addy-agent-skills/skills/code-review-and-quality"),
    s("engineering", "code-simplification", "vendor/addy-agent-skills/skills/code-simplification"),
    s("engineering", "security-and-hardening", "vendor/addy-agent-skills/skills/security-and-hardening"),
    s("engineering", "performance-optimization", "vendor/addy-agent-skills/skills/performance-optimization"),
    s("engineering", "observability-and-instrumentation", "vendor/addy-agent-skills/skills/observability-and-instrumentation"),
    s("engineering", "documentation-and-adrs", "vendor/addy-agent-skills/skills/documentation-and-adrs"),
    s("engineering", "git-workflow-and-versioning", "vendor/addy-agent-skills/skills/git-workflow-and-versioning"),
    s("engineering", "ci-cd-and-automation", "vendor/addy-agent-skills/skills/ci-cd-and-automation"),
    s("engineering", "deprecation-and-migration", "vendor/addy-agent-skills/skills/deprecation-and-migration"),
    s("engineering", "shipping-and-launch", "vendor/addy-agent-skills/skills/shipping-and-launch"),
];

/// Agent name -> skills dir relative to home (global) or project root (--project).
const AGENTS: &[(&str, &str)] = &[
    ("codex", ".codex/skills"),
    ("agents", ".agents/skills"), // OpenCode, Antigravity, Gemini and other Agent Skills hosts
];

struct Opts {
    agents: Vec<&'static str>,
    project: bool,
    copy: bool,
    only: Option<Vec<String>>,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    let result = parse_opts(&args[args.len().min(1)..]).and_then(|o| match cmd {
        "list" => Ok(list()),
        "install" => run(&o, true),
        "uninstall" => run(&o, false),
        "doctor" => doctor(),
        _ => Ok(help()),
    });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn help() {
    println!(
        "skills-bundle - install the vendored agent skills\n\n\
         commands:\n  list        show every skill in the bundle\n  \
         install     link skills into agent folders\n  \
         uninstall   remove links this tool created\n  \
         doctor      check submodules and installed links\n\n\
         options:\n  --agent codex|agents|all   target agent (default: codex)\n  \
         --project   install into ./<agent dir> instead of your home folder\n  \
         --copy      copy files instead of symlinking (default on Windows)\n  \
         --only a,b  only these bundles or skill names (e.g. --only hallmark,ponytail)"
    );
}

fn parse_opts(args: &[String]) -> Result<Opts, String> {
    let mut o = Opts { agents: vec!["codex"], project: false, copy: cfg!(windows), only: None };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--project" => o.project = true,
            "--copy" => o.copy = true,
            "--agent" => {
                let v = it.next().ok_or("--agent needs a value")?;
                o.agents = if v == "all" {
                    AGENTS.iter().map(|(n, _)| *n).collect()
                } else {
                    v.split(',')
                        .map(|n| AGENTS.iter().find(|(k, _)| *k == n).map(|(k, _)| *k)
                            .ok_or_else(|| format!("unknown agent '{n}' (codex, agents, all)")))
                        .collect::<Result<_, _>>()?
                };
            }
            "--only" => {
                let v = it.next().ok_or("--only needs a value")?;
                o.only = Some(v.split(',').map(str::to_owned).collect());
            }
            other => return Err(format!("unknown option '{other}'")),
        }
    }
    Ok(o)
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn home() -> Result<PathBuf, String> {
    env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")).map(PathBuf::from)
        .ok_or_else(|| "could not find your home folder".into())
}

fn selected<'a>(o: &'a Opts) -> impl Iterator<Item = &'static Skill> + 'a {
    SKILLS.iter().filter(move |sk| match &o.only {
        None => true,
        Some(list) => list.iter().any(|x| x == sk.bundle || x == sk.name),
    })
}

fn list() {
    let mut last = "";
    for sk in SKILLS {
        if sk.bundle != last {
            println!("\n{}", sk.bundle);
            last = sk.bundle;
        }
        let ok = repo_root().join(sk.path).join("SKILL.md").is_file();
        println!("  {} {}", if ok { "✓" } else { "✗" }, sk.name);
    }
}

fn run(o: &Opts, install: bool) -> Result<(), String> {
    let root = repo_root();
    let base = if o.project { env::current_dir().map_err(|e| e.to_string())? } else { home()? };
    let mut missing = false;
    let mut failures = 0;
    if selected(o).next().is_none() {
        return Err("no skills match --only; run list for valid names".into());
    }
    if let Some(names) = &o.only {
        for name in names {
            if !SKILLS.iter().any(|s| s.bundle == name || s.name == name) {
                return Err(format!("unknown skill or bundle '{name}'"));
            }
        }
    }
    for agent in &o.agents {
        let rel = AGENTS.iter().find(|(n, _)| n == agent).unwrap().1;
        let dir = base.join(rel);
        println!("{agent}: {}", dir.display());
        for sk in selected(o) {
            let src = root.join(sk.path);
            let dst = dir.join(sk.name);
            let r = if install {
                if !src.join("SKILL.md").is_file() {
                    missing = true;
                    println!("  ✗ {} (submodule not checked out)", sk.name);
                    continue;
                }
                fs::create_dir_all(&dir).and_then(|_| place(&src, &dst, o.copy))
            } else {
                remove(&src, &dst)
            };
            match r {
                Ok(msg) => println!("  {} {}", msg, sk.name),
                Err(e) => { failures += 1; println!("  ✗ {} ({e})", sk.name); },
            }
        }
    }
    if missing {
        eprintln!("\nrun: git submodule update --init --depth 1");
    }
    if install && selected(o).any(|s| s.bundle == "rea") {
        println!("\nREA also needs its MCP server for the binary tools: npx -y rea-agents setup");
    }
    if missing || failures > 0 {
        Err(format!("installation incomplete: {failures} operation errors; missing sources: {missing}"))
    } else { Ok(()) }
}

const OWNER_FILE: &str = ".skills-bundle-owner";

fn ownership(src: &Path) -> io::Result<String> {
    Ok(format!("skills-bundle-v1\n{}\n", fs::canonicalize(src)?.display()))
}

fn owned(src: &Path, dst: &Path) -> bool {
    match (ownership(src), fs::read_to_string(dst.join(OWNER_FILE))) {
        (Ok(expected), Ok(actual)) => expected == actual,
        _ => false,
    }
}

/// Refuse to remove a copied skill after local edits or additional user files.
fn same_tree(src: &Path, dst: &Path) -> io::Result<bool> {
    let names = |path: &Path, source: bool| -> io::Result<Vec<std::ffi::OsString>> {
        let mut result = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let name = entry.file_name();
            if name != OWNER_FILE && !(source && (name == ".git" || name == "node_modules")) { result.push(name); }
        }
        result.sort();
        Ok(result)
    };
    let entries = names(src, true)?;
    if entries != names(dst, false)? { return Ok(false); }
    for name in entries {
        let a = src.join(&name); let b = dst.join(&name);
        let at = fs::symlink_metadata(&a)?.file_type();
        let bt = fs::symlink_metadata(&b)?.file_type();
        if at.is_symlink() || bt.is_symlink() { return Ok(false); }
        if at.is_dir() && bt.is_dir() {
            if !same_tree(&a, &b)? { return Ok(false); }
        } else if at.is_file() && bt.is_file() {
            if fs::read(a)? != fs::read(b)? { return Ok(false); }
        } else { return Ok(false); }
    }
    Ok(true)
}

/// Link (or copy) src to dst. Never overwrites a folder the user made themselves.
fn place(src: &Path, dst: &Path, copy: bool) -> io::Result<&'static str> {
    if !src.join("SKILL.md").is_file() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "source SKILL.md missing"));
    }
    if let Ok(target) = fs::read_link(dst) {
        if target == src {
            return Ok("=");
        }
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "link points elsewhere, left untouched"));
    } else if dst.exists() {
        if owned(src, dst) && same_tree(src, dst)? { return Ok("="); }
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "already exists, left untouched"));
    }
    if copy {
        copy_dir(src, dst)?;
        fs::write(dst.join(OWNER_FILE), ownership(src)?)?;
        return Ok("+");
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(src, dst)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(src, dst)?;
    Ok("→")
}

/// Only removes this tool's links or explicitly owned, unmodified copies.
fn remove(src: &Path, dst: &Path) -> io::Result<&'static str> {
    if let Ok(target) = fs::read_link(dst) {
        if target == src {
            remove_link(dst)?;
            return Ok("-");
        }
        return Ok("· (link points elsewhere, kept)");
    }
    if dst.is_dir() && owned(src, dst) && same_tree(src, dst)? {
        fs::remove_dir_all(dst)?;
        return Ok("-");
    }
    Ok("·")
}

/// Windows deletes a folder symlink with remove_dir; Unix with remove_file.
fn remove_link(path: &Path) -> io::Result<()> {
    #[cfg(windows)]
    return fs::remove_dir(path);
    #[cfg(not(windows))]
    return fs::remove_file(path);
}

fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    if fs::symlink_metadata(src)?.file_type().is_symlink() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "source symlinks must be materialized before copying"));
    }
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".git" || name == "node_modules" {
            continue;
        }
        let to = dst.join(&name);
        if entry.file_type()?.is_symlink() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "source contains symlink"));
        } else if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

fn doctor() -> Result<(), String> {
    let root = repo_root();
    let missing: Vec<_> = SKILLS.iter().filter(|s| !root.join(s.path).join("SKILL.md").is_file()).collect();
    if missing.is_empty() {
        println!("✓ all {} skills present in submodules", SKILLS.len());
    } else {
        for s in &missing {
            println!("✗ {} missing at {}", s.name, s.path);
        }
        println!("run: git submodule update --init --depth 1");
    }
    let home = home()?;
    for (agent, rel) in AGENTS {
        let n = SKILLS.iter().filter(|s| home.join(rel).join(s.name).exists()).count();
        println!("{agent}: {n}/{} installed in ~/{rel}", SKILLS.len());
    }
    if missing.is_empty() { Ok(()) } else { Err("some skills are missing".into()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codex_is_the_default_target() {
        assert_eq!(parse_opts(&[]).unwrap().agents, vec!["codex"]);
    }

    #[test]
    fn all_targets_excludes_removed_host() {
        let args = vec!["--agent".into(), "all".into()];
        assert_eq!(parse_opts(&args).unwrap().agents, vec!["codex", "agents"]);
        assert!(parse_opts(&["--agent".into(), "claude".into()]).is_err());
    }

    #[test]
    fn provider_specific_api_skill_is_not_installed() {
        assert!(!SKILLS.iter().any(|skill| skill.name == "claude-api"));
    }

    #[test]
    fn every_skill_has_a_skill_md() {
        for s in SKILLS {
            assert!(repo_root().join(s.path).join("SKILL.md").is_file(), "{} missing", s.path);
        }
    }

    #[test]
    fn skill_names_are_unique() {
        let mut names: Vec<_> = SKILLS.iter().map(|s| s.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), SKILLS.len());
    }

    #[test]
    fn install_and_uninstall_roundtrip() {
        let tmp = env::temp_dir().join(format!("skills-bundle-test-{}", std::process::id()));
        let src = repo_root().join(SKILLS.iter().find(|s| s.name == "hallmark").unwrap().path);
        let dst = tmp.join("hallmark");
        fs::create_dir_all(&tmp).unwrap();
        #[cfg(unix)]
        {
            assert_eq!(place(&src, &dst, false).unwrap(), "→");
            assert_eq!(place(&src, &dst, false).unwrap(), "=");
            assert_eq!(remove(&src, &dst).unwrap(), "-");
        }
        assert_eq!(place(&src, &dst, true).unwrap(), "+");
        assert_eq!(place(&src, &dst, true).unwrap(), "=");
        assert_eq!(remove(&src, &dst).unwrap(), "-");
        assert!(!dst.exists());
        fs::remove_dir_all(&tmp).ok();
    }

    fn fixture(name: &str) -> (PathBuf, PathBuf, PathBuf) {
        let root = env::temp_dir().join(format!("skills-bundle-{}-{name}", std::process::id()));
        let src = root.join("source"); let dst = root.join("installed");
        fs::create_dir_all(src.join("references")).unwrap();
        fs::write(src.join("SKILL.md"), "---\nname: example\ndescription: test\n---\nExample").unwrap();
        fs::write(src.join("references/guide.md"), "guide").unwrap();
        (root, src, dst)
    }

    #[test]
    fn unowned_matching_skill_is_preserved() {
        let (root, src, dst) = fixture("unowned");
        copy_dir(&src, &dst).unwrap();
        assert!(place(&src, &dst, true).is_err());
        assert_eq!(remove(&src, &dst).unwrap(), "·");
        assert!(dst.join("SKILL.md").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn directories_without_skill_files_are_preserved() {
        let (root, src, dst) = fixture("missing-skill");
        fs::remove_file(src.join("SKILL.md")).unwrap();
        fs::create_dir_all(&dst).unwrap();
        fs::write(dst.join("personal.txt"), "keep").unwrap();
        assert_eq!(remove(&src, &dst).unwrap(), "·");
        assert!(place(&src, &dst, true).is_err());
        assert!(dst.join("personal.txt").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn changed_or_added_files_are_preserved() {
        let (root, src, dst) = fixture("modified");
        place(&src, &dst, true).unwrap();
        fs::write(dst.join("references/guide.md"), "my edits").unwrap();
        assert_eq!(remove(&src, &dst).unwrap(), "·");
        fs::write(dst.join("references/guide.md"), "guide").unwrap();
        fs::write(dst.join("personal.txt"), "keep").unwrap();
        assert_eq!(remove(&src, &dst).unwrap(), "·");
        fs::remove_file(dst.join("personal.txt")).unwrap();
        fs::create_dir_all(dst.join("node_modules")).unwrap();
        fs::write(dst.join("node_modules/personal.txt"), "keep").unwrap();
        assert_eq!(remove(&src, &dst).unwrap(), "·");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn ownership_from_another_source_is_preserved() {
        let (root, src, dst) = fixture("ownership");
        place(&src, &dst, true).unwrap();
        fs::write(dst.join(OWNER_FILE), "different source").unwrap();
        assert_eq!(remove(&src, &dst).unwrap(), "·");
        assert!(dst.exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn source_update_does_not_delete_installed_copy() {
        let (root, src, dst) = fixture("updated-source");
        place(&src, &dst, true).unwrap();
        fs::write(src.join("references/guide.md"), "new upstream version").unwrap();
        assert_eq!(remove(&src, &dst).unwrap(), "·");
        assert!(place(&src, &dst, true).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn unrelated_symlink_is_preserved() {
        let (root, src, dst) = fixture("other-link");
        let other = root.join("other"); fs::create_dir_all(&other).unwrap();
        std::os::unix::fs::symlink(&other, &dst).unwrap();
        assert!(place(&src, &dst, false).is_err());
        assert_eq!(remove(&src, &dst).unwrap(), "· (link points elsewhere, kept)");
        assert_eq!(fs::read_link(&dst).unwrap(), other);
        fs::remove_dir_all(root).unwrap();
    }
}
