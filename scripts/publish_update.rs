#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! clap = "4.4.11"
//! ```

use clap::Arg;
use std::process::Command;

fn ensure_git_clean() {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("failed to execute git status --porcelain");
    if !output.stdout.is_empty() {
        println!("Git status is not clean. Please commit all changes before publishing.");
        std::process::exit(1);
    }
}

fn gen_new_version(old_version: &str, release_type: &str) -> String {
    let mut version_parts = old_version
        .split(".")
        .map(|part| part.parse::<u32>().unwrap());
    let mut major = version_parts.next().unwrap();
    let mut minor = version_parts.next().unwrap();
    let mut patch = version_parts.next().unwrap();

    if release_type == "major" {
        major += 1;
        minor = 0;
        patch = 0;
    } else if release_type == "minor" {
        minor += 1;
        patch = 0;
    } else if release_type == "patch" {
        patch += 1;
    } else {
        println!("Unknown release type: {}", release_type);
        std::process::exit(1);
    }

    format!("{}.{}.{}", major, minor, patch)
}

fn update_readme_version(old_version: &str, new_version: &str) {
    let readme_file = std::fs::read_to_string("README.md").expect("failed to read README.md");
    let new_readme_file = readme_file.replace(old_version, new_version);
    std::fs::write("README.md", new_readme_file).expect("failed to write README.md");
}

fn update_cargo_version(old_version: &str, new_version: &str) {
    let toml_file = std::fs::read_to_string("Cargo.toml").expect("failed to read Cargo.toml");
    let new_toml_file = toml_file.replace(
        format!("version = \"{}\"", old_version).as_str(),
        format!("version = \"{}\"", new_version).as_str(),
    );
    std::fs::write("Cargo.toml", new_toml_file).expect("failed to write Cargo.toml");
     Command::new("cargo")
        .arg("build")
        .output()
        .expect("failed to build updated crate"); //To ensure Cargo.lock version is updated as well - easier than manual parsing
}

fn parse_args() -> String {
    let matches = clap::Command::new("publish_update")
        .version("1.0")
        .author("Adam Killam <adammkillam@gmail.com>")
        .about("Publishes a new version of the crate")
        .arg(
            Arg::new("release_type")
                .long("release-type")
                .short('r')
                .help("The type of release to publish - major, minor, or patch")
                .required(true),
        )
        .get_matches();
    return matches
        .get_one::<String>("release_type")
        .unwrap_or_else(|| panic!("Release type is required"))
        .to_string();
}

fn main() {
    ensure_git_clean();

    let release_type = parse_args();
    let current_version = std::env::var("CARGO_MAKE_PROJECT_VERSION").unwrap();
    let new_version = gen_new_version(&current_version, &release_type);

    println!("Current version: {}", current_version);
    println!("New version: {}", new_version);

    update_readme_version(&current_version, &new_version);
    update_cargo_version(&current_version, &new_version);

    Command::new("git")
        .arg("add")
        .arg("README.md")
        .arg("Cargo.toml")
        .arg("Cargo.lock")
        .output()
        .expect("failed to execute git add README.md Cargo.toml Cargo.lock");
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("Release {}", new_version))
        .output()
        .expect("failed to execute git commit -m");
    Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(format!("v{}", new_version))
        .arg("-m")
        .arg(format!("Release {}", new_version))
        .output()
        .expect("failed to execute git tag -a");
    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .arg("--tags")
        .output()
        .expect("failed to execute git push origin master --tags");
    Command::new("cargo")
        .arg("publish")
        .output()
        .expect("failed to execute cargo publish");

    println!("Successfully published version {}", new_version);
}
