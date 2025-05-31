// SPDX-License-Identifier: ISC
// Copyright (c) 2024 Ben Giles and contributors
// Permission to use, copy, modify, and/or distribute this file for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
// See the LICENSE file in the project root for full license text.

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, Stdio};

const CARGO_CLIPPY_ARGS: &[&str] =
    &["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"];

// Helper to get $CARGO_HOME/bin or $HOME/.cargo/bin or $USERPROFILE/.cargo/bin
fn get_cargo_home_bin(tool_name: &str) -> String {
    if let Ok(cargo_home) = env::var("CARGO_HOME") {
        return Path::new(&cargo_home).join("bin").join(tool_name).to_string_lossy().into_owned();
    }
    if let Ok(home) = env::var("HOME") {
        let path = Path::new(&home).join(".cargo").join("bin").join(tool_name);
        if path.exists() {
            return path.to_string_lossy().into_owned();
        }
    }
    if let Ok(user_profile) = env::var("USERPROFILE") {
        let path = Path::new(&user_profile).join(".cargo").join("bin").join(tool_name);
        if path.exists() {
            return path.to_string_lossy().into_owned();
        }
    }
    // Fallback or error if not found, though xtask might rely on it being in PATH if these fail
    eprintln!(
        "Warning: Could not determine cargo binary path for {} via CARGO_HOME, HOME, or USERPROFILE. Assuming it's in PATH.",
        tool_name
    );
    tool_name.to_string() // Default to tool_name, assuming it's in PATH
}

fn run_cargo_with_input(
    cargo_bin_name: &str,
    cargo_args: &[&str],
    stdin_content: &str,
    project_root: &Path,
) -> bool {
    let mut cmd_instance = Command::new("cargo");
    cmd_instance.arg("run");
    cmd_instance.arg("--bin");
    cmd_instance.arg(cargo_bin_name);
    cmd_instance.arg("--manifest-path");
    cmd_instance.arg(project_root.join("Cargo.toml"));
    if !cargo_args.is_empty() {
        cmd_instance.arg("--");
        cmd_instance.args(cargo_args);
    }

    cmd_instance.stdin(Stdio::piped());
    cmd_instance.stdout(Stdio::inherit());
    cmd_instance.stderr(Stdio::inherit());
    cmd_instance.current_dir(project_root);

    println!(
        "Running command (piping stdin): cargo run --bin {} {} {}",
        cargo_bin_name,
        if cargo_args.is_empty() { "" } else { "--" },
        cargo_args.join(" ")
    );

    let mut child = cmd_instance
        .spawn()
        .expect(&format!("Failed to spawn command: cargo run --bin {}", cargo_bin_name));

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(stdin_content.as_bytes()).expect("Failed to write to command stdin");
    }

    let status = child.wait().expect("Failed to wait on command");
    status.success()
}

fn run_command(command: &str, args: &[&str], working_dir: Option<&Path>) {
    let mut cmd_instance = Command::new(command);
    cmd_instance.args(args);
    if let Some(dir) = working_dir {
        cmd_instance.current_dir(dir);
    }

    println!("Running command: {} {}", command, args.join(" "));

    let status = cmd_instance
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect(&format!("Failed to execute command: {}", command));

    if !status.success() {
        eprintln!("Command {} {} failed with status: {}", command, args.join(" "), status);
        exit(1);
    }
}

fn validate_mistake_file(file_path: &Path, project_root: &Path) -> bool {
    println!("Validating mistake file: {:?}", file_path);
    if !file_path.exists() {
        eprintln!("Error: Mistake file not found: {:?}", file_path);
        return false;
    }
    let content_str = fs::read_to_string(file_path)
        .expect(&format!("Failed to read mistake file: {:?}", file_path));

    if content_str.trim().is_empty() {
        println!("Mistake file {:?} is empty, considering it valid.", file_path);
        return true;
    }

    let parsed_json: Result<serde_json::Value, _> = serde_json::from_str(&content_str);
    match parsed_json {
        Ok(json) => {
            if let Some(arr) = json.as_array() {
                if arr.is_empty() {
                    println!(
                        "Mistake file {:?} is an empty array, considering it valid.",
                        file_path
                    );
                    return true;
                }
                for (index, entry) in arr.iter().enumerate() {
                    println!("Validating entry {} in file {:?}", index, file_path);
                    let entry_str = serde_json::to_string(entry)
                        .expect("Failed to re-serialize entry for validation");
                    if !run_cargo_with_input("fuckup", &[], &entry_str, project_root) {
                        eprintln!("Error: Entry {} in file {:?} is not valid.", index, file_path);
                        return false;
                    }
                }
                println!("All entries in mistake file {:?} are valid.", file_path);
                true
            } else {
                // If it's not an array, validate as a single mistake object
                println!("Validating file {:?} as a single mistake object...", file_path);
                run_cargo_with_input("fuckup", &[], &content_str, project_root)
            }
        }
        Err(e) => {
            eprintln!("Error: Mistake file {:?} is not valid JSON: {}", file_path, e);
            false
        }
    }
}

// --- xtask subcommands ---

fn xtask_quicktype(args: &mut std::iter::Skip<std::env::Args>, project_root: &Path) {
    if let Some(input_file) = args.next() {
        run_command(
            &get_cargo_home_bin("quicktype"),
            &["--lang", "rust", "--src", &input_file],
            Some(project_root),
        );
    } else {
        eprintln!("Usage: cargo xtask quicktype <input.json>");
        exit(1);
    }
}

fn xtask_check(project_root: &Path) {
    run_command("cargo", &["check", "--all-targets", "--all-features"], Some(project_root));
}

fn xtask_clippy(project_root: &Path) {
    run_command("cargo", CARGO_CLIPPY_ARGS, Some(project_root));
}

fn xtask_doc(args: &mut std::iter::Skip<std::env::Args>, project_root: &Path) {
    let mut doc_args = vec!["doc", "--no-deps"];
    if args.any(|arg| arg == "--open") {
        doc_args.push("--open");
    }
    run_command("cargo", &doc_args, Some(project_root));
}

fn xtask_mistake_add(
    args: &mut std::iter::Skip<std::env::Args>,
    project_root: &Path,
    mistakes_log_file_path: &Path,
) {
    if let Some(new_entry_file_str_val) = args.next() {
        let new_entry_file_str = &new_entry_file_str_val;
        let new_entry_file = project_root.join(new_entry_file_str);
        if !new_entry_file.exists() {
            eprintln!(
                "Error: New mistake entry file not found: {}\n (Full path: {:?})",
                new_entry_file_str, new_entry_file
            );
            exit(1);
        }
        println!(
            "Attempting to add new mistake from: {} (relative to project root)",
            new_entry_file_str
        );

        if !validate_mistake_file(&new_entry_file, project_root) {
            eprintln!(
                "Error: New mistake entry file {:?} is not valid according to the schema.",
                new_entry_file
            );
            exit(1);
        }
        println!("New mistake entry file {:?} is valid.", new_entry_file);

        let new_entry_content_str =
            fs::read_to_string(&new_entry_file).expect("Failed to read new mistake entry file.");
        let new_entry_json: serde_json::Value = serde_json::from_str(&new_entry_content_str)
            .expect("Failed to parse new mistake entry JSON.");

        let mut mistakes_list: Vec<serde_json::Value> = if mistakes_log_file_path.exists() {
            let log_content = fs::read_to_string(mistakes_log_file_path)
                .expect("Failed to read mistakes.json log file.");
            if log_content.trim().is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&log_content)
                    .expect("Failed to parse mistakes.json as JSON array.")
            }
        } else {
            Vec::new()
        };

        mistakes_list.push(new_entry_json);

        let updated_log_content = serde_json::to_string_pretty(&mistakes_list)
            .expect("Failed to serialize updated mistakes list to JSON.");
        fs::write(mistakes_log_file_path, updated_log_content)
            .expect("Failed to write updated mistakes.json log file.");
        println!(
            "Successfully added new entry to {:?}\nContent:\n{}",
            mistakes_log_file_path,
            fs::read_to_string(mistakes_log_file_path).unwrap_or_default()
        );

        if !validate_mistake_file(mistakes_log_file_path, project_root) {
            eprintln!("Error: The updated {:?} is not valid.", mistakes_log_file_path);
            exit(1);
        }
        println!("Updated {:?} is valid.", mistakes_log_file_path);
    } else {
        eprintln!("Usage: cargo xtask mistake add <path_to_new_mistake_entry.json>");
        exit(1);
    }
}

fn xtask_mistake_validate(
    args: &mut std::iter::Skip<std::env::Args>,
    project_root: &Path,
    mistakes_log_file_path: &Path,
) {
    let file_to_validate_str_opt = args.next();
    let file_path_to_validate = match file_to_validate_str_opt.as_deref() {
        Some(f_str) => project_root.join(f_str),
        None => mistakes_log_file_path.to_path_buf(),
    };

    if validate_mistake_file(&file_path_to_validate, project_root) {
        println!("Mistake file {:?} is valid.", file_path_to_validate);
    } else {
        eprintln!("Mistake file {:?} is NOT valid.", file_path_to_validate);
        exit(1);
    }
}

fn xtask_mistake_legacy_validate(file_str: &str, project_root: &Path) {
    let file_path = project_root.join(file_str);
    if validate_mistake_file(&file_path, project_root) {
        println!("File {:?} is valid.", file_path);
    } else {
        eprintln!("File {:?} is NOT valid.", file_path);
        exit(1);
    }
}

fn xtask_mistake(
    args: &mut std::iter::Skip<std::env::Args>,
    project_root: &Path,
    mistakes_log_file_path: &Path,
) {
    match args.next().as_deref() {
        Some("add") => xtask_mistake_add(args, project_root, mistakes_log_file_path),
        Some("validate") => xtask_mistake_validate(args, project_root, mistakes_log_file_path),
        Some(other_file_str) => xtask_mistake_legacy_validate(other_file_str, project_root),
        None => {
            eprintln!("Usage: cargo xtask mistake <add|validate> [args]");
            eprintln!("  cargo xtask mistake add <path_to_new_mistake_entry.json>");
            eprintln!("  cargo xtask mistake validate [path_to_mistake_file.json] (defaults to mistakes.json)");
            eprintln!("  cargo xtask mistake <path_to_mistake_file.json> (legacy: validates a specific single-entry file)");
            exit(1);
        }
    }
}

fn xtask_taplo_format(project_root: &Path) {
    let taplo_path = get_cargo_home_bin("taplo");
    run_command(&taplo_path, &["format", "."], Some(project_root));
}

fn xtask_taplo_lint(project_root: &Path) {
    let taplo_path = get_cargo_home_bin("taplo");
    run_command(&taplo_path, &["lint", "."], Some(project_root));
}

fn xtask_pre_commit_run(project_root: &Path) {
    run_command("pre-commit", &["run", "--all-files"], Some(project_root));
}

fn xtask_pre_commit_install(project_root: &Path) {
    run_command("pre-commit", &["install"], Some(project_root));
}

fn xtask_cargo_about(project_root: &Path) {
    let output_target = if cfg!(windows) { "NUL" } else { "/dev/null" };
    run_command(
        "cargo",
        &["about", "generate", "--format", "json", "-o", output_target],
        Some(project_root),
    );
}

fn xtask_build(project_root: &Path) {
    run_command("cargo", &["build", "--all-targets", "--all-features"], Some(project_root));
}

fn xtask_test(project_root: &Path) {
    run_command(
        "cargo",
        &["test", "--all-targets", "--all-features", "--verbose"],
        Some(project_root),
    );
}

fn xtask_all_ci(project_root: &Path) {
    println!("Running all CI checks (xtask ci)...");
    let mut all_passed = true;

    println!("\n==> Formatting TOML files (taplo-format)");
    xtask_taplo_format(project_root);

    println!("\n==> Linting TOML files (taplo-lint)");
    xtask_taplo_lint(project_root);

    println!("\n==> Linting Rust code (cargo clippy)");
    xtask_clippy(project_root);

    println!("\n==> Running tests (cargo test)");
    xtask_test(project_root);

    println!("\n==> Checking Rust code (cargo check)");
    xtask_check(project_root);

    println!("\n==> Building documentation (cargo doc)");
    let mut empty_args = env::args().skip(usize::MAX); // Create an empty iterator
    xtask_doc(&mut empty_args, project_root);

    println!("\n==> Checking license compliance (cargo-about)");
    xtask_cargo_about(project_root);

    println!("\n==> Running pre-commit hooks (if configured and installed)");
    match Command::new("pre-commit")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => {
            if status.success() {
                let pre_commit_run_output = Command::new("pre-commit")
                    .args(&["run", "--all-files"])
                    .current_dir(project_root)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("Failed to execute pre-commit run");

                if !pre_commit_run_output.status.success() {
                    eprintln!(
                        "pre-commit run --all-files failed with status: {}",
                        pre_commit_run_output.status
                    );
                    all_passed = false;
                }
            } else {
                println!("Skipping pre-commit run as pre-commit --version did not succeed (is pre-commit configured correctly and in PATH?).");
            }
        }
        Err(e) => {
            println!("Failed to execute pre-commit --version (is pre-commit installed and in PATH?). Skipping pre-commit run. Error: {}", e);
        }
    }

    if all_passed {
        println!("\nAll CI checks passed!");
    } else {
        eprintln!("\nSome CI checks failed.");
        exit(1);
    }
}

// TODO: Consider using a CLI framework like clap or structopt for clearer argument definitions and automatic help generation.
fn main() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let mut args = env::args().skip(1);
    let mistakes_log_file_path = project_root.join("mistakes.json");

    match args.next().as_deref() {
        Some("quicktype") => xtask_quicktype(&mut args, &project_root),
        Some("check") => xtask_check(&project_root),
        Some("clippy") => xtask_clippy(&project_root),
        Some("doc") => xtask_doc(&mut args, &project_root),
        Some("mistake") => xtask_mistake(&mut args, &project_root, &mistakes_log_file_path),
        Some("taplo-format") => xtask_taplo_format(&project_root),
        Some("taplo-lint") => xtask_taplo_lint(&project_root),
        Some("pre-commit-run") => xtask_pre_commit_run(&project_root),
        Some("pre-commit-install") => xtask_pre_commit_install(&project_root),
        Some("cargo-about") => xtask_cargo_about(&project_root),
        Some("build") => xtask_build(&project_root),
        Some("test") => xtask_test(&project_root),
        Some("all") | Some("ci") => xtask_all_ci(&project_root),
        Some(other) => {
            eprintln!("Unknown xtask command: {}", other);
            print_help();
            exit(1);
        }
        None => {
            eprintln!("No xtask command provided.");
            print_help();
            exit(1);
        }
    }
}

fn print_help() {
    eprintln!("\nUsage: cargo xtask <command> [args]");
    eprintln!("\nAvailable commands:");
    eprintln!("  check                      Alias for 'cargo check --all-targets --all-features'");
    eprintln!("  clippy                     Alias for 'cargo clippy --all-targets --all-features -- -D warnings'");
    eprintln!("  doc [--open]               Alias for 'cargo doc --no-deps'. Use --open to open in browser.");
    eprintln!("  build                      Alias for 'cargo build --all-targets --all-features'");
    eprintln!("  test                       Alias for 'cargo test --all-targets --all-features --verbose'");
    eprintln!("  taplo-format               Formats TOML files using 'taplo format .'");
    eprintln!("  taplo-lint                 Lints TOML files using 'taplo lint .'");
    eprintln!("  pre-commit-run             Runs 'pre-commit run --all-files'");
    eprintln!("  pre-commit-install         Runs 'pre-commit install'");
    eprintln!("  cargo-about                Checks licenses using 'cargo about generate --format json' against 'about.toml'.");
    eprintln!("  quicktype <input.json>     Generates Rust types from JSON using quicktype.");
    eprintln!("  mistake <subcommand> [args] Handles LLM mistake logging:");
    eprintln!("    add <entry.json>         Adds a new mistake entry from a JSON file to mistakes.json and validates.");
    eprintln!("    validate [file.json]     Validates mistakes.json (default) or a specified JSON mistake file (validates each entry if it's an array, or the file as a single object).");
    eprintln!("    <file.json>              (Legacy) Validates a specific single-entry JSON mistake file.");
    eprintln!("  all / ci                   Runs all common CI checks: taplo format/lint, clippy, test, check, doc, cargo-about, pre-commit.");
    eprintln!("\nNote: Some tasks require external tools like taplo, pre-commit, cargo-about, quicktype, or fuckup to be installed, typically via `cargo install <tool>` or system package managers, and their binaries accessible (often via $CARGO_HOME/bin or $HOME/.cargo/bin).");
}
