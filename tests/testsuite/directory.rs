use ansi_term::Color;
use dirs::home_dir;
use git2::Repository;
use std::fs;
use std::io;
use std::path::Path;
use tempfile::TempDir;

use crate::common::{self, TestCommand};

#[test]
fn home_directory() -> io::Result<()> {
    let output = common::render_module("directory")
        .arg("--path=~")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("~"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("~/starship/engine"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine/schematics");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("starship/engine/schematics")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn root_directory() -> io::Result<()> {
    let output = common::render_module("directory")
        .arg("--path=/")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("/"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[cfg(not(target_os = "windows"))]
fn directory_in_root() -> io::Result<()> {
    let output = common::render_module("directory")
        .arg("--path=/etc")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("/etc"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
fn directory_in_root() -> io::Result<()> {
    let output = common::render_module("dir").arg("--path=C:\\").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("/c"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_root() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("starship/thrusters/rocket")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_config_large() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            truncation_length = 100
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("/tmp/starship/thrusters/rocket")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_config_small() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            truncation_length = 2
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("thrusters/rocket"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn git_repo_root() -> io::Result<()> {
    // TODO: Investigate why git repo related tests fail when the tempdir is within /tmp/...
    // Temporarily making the tempdir within $HOME
    // #[ignore] can be removed after this TODO is addressed
    let tmp_dir = TempDir::new_in(dirs::home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    fs::create_dir(&repo_dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("rocket-controls"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(dirs::home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("rocket-controls/src"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(dirs::home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("src/meters/fuel-gauge"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn directory_in_git_repo_truncate_to_repo_false() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(dirs::home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // Don't truncate the path at all.
            truncation_length = 5
            truncate_to_repo = false
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("above-repo/rocket-controls/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn directory_in_git_repo_truncate_to_repo_true() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(dirs::home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // `truncate_to_repo = true` should display the truncated path
            truncation_length = 5
            truncate_to_repo = true
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("rocket-controls/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    Ok(())
}
