//! skills-bundle: link the vendored agent skills into each agent's skills folder.
//!
//! Usage:
//!   skills-bundle list
//!   skills-bundle install   [--agent claude|codex|agents|all] [--project] [--copy] [--only NAME,...]
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
];

/// Agent name -> skills dir relative to home (global) or project root (--project).
const AGENTS: &[(&str, &str)] = &[
    ("claude", ".claude/skills"),
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
         options:\n  --agent claude|codex|agents|all   target agent (default: claude)\n  \
         --project   install into ./<agent dir> instead of your home folder\n  \
         --copy      copy files instead of symlinking (default on Windows)\n  \
         --only a,b  only these bundles or skill names (e.g. --only hallmark,ponytail)"
    );
}

fn parse_opts(args: &[String]) -> Result<Opts, String> {
    let mut o = Opts { agents: vec!["claude"], project: false, copy: cfg!(windows), only: None };
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
                            .ok_or_else(|| format!("unknown agent '{n}' (claude, codex, agents, all)")))
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
                Err(e) => println!("  ✗ {} ({e})", sk.name),
            }
        }
    }
    if missing {
        eprintln!("\nrun: git submodule update --init --depth 1");
    }
    if install && selected(o).any(|s| s.bundle == "rea") {
        println!("\nREA also needs its MCP server for the binary tools: npx -y rea-agents setup");
    }
    Ok(())
}

/// Link (or copy) src to dst. Never overwrites a folder the user made themselves.
fn place(src: &Path, dst: &Path, copy: bool) -> io::Result<&'static str> {
    if let Ok(target) = fs::read_link(dst) {
        if target == src {
            return Ok("=");
        }
        fs::remove_file(dst)?; // stale link from an older checkout
    } else if dst.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "already exists, left untouched"));
    }
    if copy {
        copy_dir(src, dst)?;
        return Ok("+");
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(src, dst)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(src, dst)?;
    Ok("→")
}

/// Only removes links pointing into this repo, or copies marked by a matching SKILL.md.
fn remove(src: &Path, dst: &Path) -> io::Result<&'static str> {
    if let Ok(target) = fs::read_link(dst) {
        if target == src {
            fs::remove_file(dst)?;
            return Ok("-");
        }
        return Ok("· (link points elsewhere, kept)");
    }
    if dst.is_dir() && fs::read(dst.join("SKILL.md")).ok() == fs::read(src.join("SKILL.md")).ok() {
        fs::remove_dir_all(dst)?;
        return Ok("-");
    }
    Ok("·")
}

fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".git" || name == "node_modules" {
            continue;
        }
        let to = dst.join(&name);
        if entry.file_type()?.is_dir() {
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
        let src = repo_root().join(SKILLS[7].path); // hallmark
        let dst = tmp.join("hallmark");
        fs::create_dir_all(&tmp).unwrap();
        assert_eq!(place(&src, &dst, false).unwrap(), "→");
        assert_eq!(place(&src, &dst, false).unwrap(), "=");
        assert_eq!(remove(&src, &dst).unwrap(), "-");
        assert_eq!(place(&src, &dst, true).unwrap(), "+");
        assert_eq!(remove(&src, &dst).unwrap(), "-");
        assert!(!dst.exists());
        fs::remove_dir_all(&tmp).ok();
    }
}
