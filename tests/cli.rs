use std::{fs, path::Path, process::Command};

fn invoke(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_skills-bundle"))
        .current_dir(root).args(args).output().unwrap()
}

#[test]
fn copy_install_uninstall_for_every_agent() {
    let root = std::env::temp_dir().join(format!("bundle-cli-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let result = invoke(&root, &["install", "--project", "--agent", "all", "--copy", "--only", "humanizer"]);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    for agent in [".claude", ".codex", ".agents"] {
        assert!(root.join(agent).join("skills/humanizer/SKILL.md").is_file());
    }
    assert!(invoke(&root, &["install", "--project", "--agent", "all", "--copy", "--only", "humanizer"]).status.success());
    assert!(invoke(&root, &["uninstall", "--project", "--agent", "all", "--only", "humanizer"]).status.success());
    for agent in [".claude", ".codex", ".agents"] {
        assert!(!root.join(agent).join("skills/humanizer").exists());
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn conflicts_and_invalid_selection_fail() {
    let root = std::env::temp_dir().join(format!("bundle-conflict-{}", std::process::id()));
    let dst = root.join(".codex/skills/humanizer");
    fs::create_dir_all(&dst).unwrap();
    fs::write(dst.join("personal.txt"), "keep me").unwrap();
    assert!(!invoke(&root, &["install", "--project", "--agent", "codex", "--copy", "--only", "humanizer"]).status.success());
    assert_eq!(fs::read_to_string(dst.join("personal.txt")).unwrap(), "keep me");
    assert!(!invoke(&root, &["install", "--project", "--only", "does-not-exist"]).status.success());
    assert!(!invoke(&root, &["install", "--agent", "does-not-exist"]).status.success());
    fs::remove_dir_all(root).unwrap();
}
