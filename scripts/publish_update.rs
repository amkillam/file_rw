//! ```cargo
//! [dependencies]
//! clap = 4.4.11
//! ```

use clap::{App, Arg};
use std::fs::read_to_string;
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

fn get_current_version() -> String {
    let toml_file = std::fs::read_to_string("Cargo.toml").expect("failed to read Cargo.toml");
    let version_line = toml_file
        .lines()
        .find(|line| line.contains("version ="))
        .unwrap();
    let version = version_line.split("\"").nth(1).unwrap();
    version.to_string()
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
        format!("version = {}", old_version),
        format!("version = {}", new_version),
    );
    std::fs::write("Cargo.toml", new_toml_file).expect("failed to write Cargo.toml");
}

fn parse_args() -> String {
    let matches = App::new("publish_update")
        .version("1.0")
        .author("Adam Killam <adammkillam@gmail.com>")
        .about("Publishes a new version of the crate")
        .arg(
            Arg::with_name("release_type")
                .help("major, minor, or patch")
                .required(true)
                .index(1),
        )
        .get_matches();
    return matches
        .value_of("release_type")
        .unwrap_or_else(|| panic!("Release type is required"))
        .to_string();
}

fn ensure_tests_pass() {
    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("failed to execute cargo test");
    if output.stdout.contains("FAILED") {
        println!("Tests failed. Please fix before publishing.");
        std::process::exit(1);
    }
}

fn ensure_crate_builds() {
    let output = Command::new("cargo")
        .arg("build")
        .output()
        .expect("failed to execute cargo build");
    if output.stdout.contains("error") {
        println!("Crate does not build. Please fix before publishing.");
        std::process::exit(1);
    }
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
}