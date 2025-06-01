// SPDX-License-Identifier: ISC
// Copyright (c) 2024 Ben Giles and contributors
// Permission to use, copy, modify, and/or distribute this file for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
// See the LICENSE file in the project root for full license text.

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, Stdio};

<<<<<<< HEAD
<<<<<<< HEAD
// Helper to get $CARGO_HOME/bin or $HOME/.cargo/bin
fn get_cargo_home_bin(tool_name: &str) -> String {
    match env::var("CARGO_HOME") {
        Ok(cargo_home) => Path::new(&cargo_home)
            .join("bin")
            .join(tool_name)
            .to_string_lossy()
            .into_owned(),
        Err(_) => {
            let home = env::var("HOME").expect("HOME environment variable not set");
            Path::new(&home)
                .join(".cargo")
                .join("bin")
                .join(tool_name)
                .to_string_lossy()
                .into_owned()
=======
=======
const CARGO_CLIPPY_ARGS: &[&str] = &["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"];

// Helper to get $CARGO_HOME/bin or $HOME/.cargo/bin
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
fn get_cargo_home_bin(tool_name: &str) -> String {
    match env::var("CARGO_HOME") {
        Ok(cargo_home) => Path::new(&cargo_home)
            .join("bin")
            .join(tool_name)
            .to_string_lossy()
            .into_owned(),
        Err(_) => {
            let home = env::var("HOME").expect("HOME environment variable not set");
<<<<<<< HEAD
            Path::new(&home).join(".cargo").join("bin").join(tool_name).to_string_lossy().into_owned()
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
=======
            Path::new(&home)
                .join(".cargo")
                .join("bin")
                .join(tool_name)
                .to_string_lossy()
                .into_owned()
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
        }
    }
}

<<<<<<< HEAD
<<<<<<< HEAD
fn run_command_pipe_stdin_to_cargo_bin(
    cargo_bin_name: &str,
    cargo_args: &[&str], // Args for `cargo run --bin <name> -- `
=======
fn run_command_pipe_stdin_to_cargo_bin(
    cargo_bin_name: &str,
    cargo_args: &[&str],
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
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
<<<<<<< HEAD
    cmd_instance.current_dir(project_root);
=======
    cmd_instance.current_dir(project_root);
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)

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
        stdin
            .write_all(stdin_content.as_bytes())
            .expect("Failed to write to command stdin");
    }

    let status = child.wait().expect("Failed to wait on command");
    status.success()
}

<<<<<<< HEAD
=======
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
=======
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)

fn run_command(command: &str, args: &[&str], working_dir: Option<&Path>) {
    let mut cmd_instance = Command::new(command);
    cmd_instance.args(args);
    if let Some(dir) = working_dir {
        cmd_instance.current_dir(dir);
    }

    println!(
        "Running command: {} {}",
        command,
        args.join(" ")
    );

    let output = cmd_instance
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(&format!("Failed to execute command: {}", command));

    if !output.status.success() {
        eprintln!(
<<<<<<< HEAD
<<<<<<< HEAD
            "Command {} {} failed with status: {}",
            command,
            args.join(" "),
=======
            "Command {} failed with status: {}",
            command,
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
=======
            "Command {} {} failed with status: {}",
            command,
            args.join(" "),
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
            output.status
        );
        exit(1);
    }
}

<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
fn validate_mistake_file(file_path: &Path, project_root: &Path, is_log_file: bool) -> bool {
    println!("Validating mistake file: {:?} (is_log_file: {})", file_path, is_log_file);
    if !file_path.exists() {
        eprintln!("Error: Mistake file not found: {:?}", file_path);
        return false;
    }
    let content_str = fs::read_to_string(file_path)
        .expect(&format!("Failed to read mistake file: {:?}", file_path));

    if is_log_file {
<<<<<<< HEAD
        if content_str.trim().is_empty() {
=======
        if content_str.trim().is_empty() {
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
            println!("Mistake log file {:?} is empty, considering it valid.", file_path);
            return true;
        }
        let mistakes_array: Result<Vec<serde_json::Value>, _> = serde_json::from_str(&content_str);
        match mistakes_array {
            Ok(arr) => {
                if arr.is_empty() {
                    println!("Mistake log file {:?} is an empty array, considering it valid.", file_path);
                    return true;
                }
                for (index, entry) in arr.iter().enumerate() {
                    println!("Validating entry {} in log file {:?}", index, file_path);
                    let entry_str = serde_json::to_string(entry).expect("Failed to re-serialize log entry for validation");
                    if !run_command_pipe_stdin_to_cargo_bin("fuckup", &[], &entry_str, project_root) {
                        eprintln!("Error: Entry {} in log file {:?} is not valid.", index, file_path);
                        return false;
                    }
                }
                println!("All entries in mistake log file {:?} are valid.", file_path);
                true
            }
            Err(e) => {
                eprintln!("Error: Mistake log file {:?} is not a valid JSON array: {}", file_path, e);
                println!("Attempting to validate {:?} as a single mistake object...", file_path);
                run_command_pipe_stdin_to_cargo_bin("fuckup", &[], &content_str, project_root)
            }
        }
    } else {
        run_command_pipe_stdin_to_cargo_bin("fuckup", &[], &content_str, project_root)
    }
}


<<<<<<< HEAD
fn main() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let mut args = env::args().skip(1);
    let taplo_path = get_cargo_home_bin("taplo");
    let mistakes_log_file_path = project_root.join("mistakes.json");
=======
=======
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
fn main() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let mut args = env::args().skip(1);
    let taplo_path = get_cargo_home_bin("taplo");
<<<<<<< HEAD
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
=======
    let mistakes_log_file_path = project_root.join("mistakes.json");
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)

    match args.next().as_deref() {
        Some("quicktype") => {
            if let Some(input_file) = args.next() {
                run_command(
<<<<<<< HEAD
<<<<<<< HEAD
                    &get_cargo_home_bin("quicktype"),
                    &["--lang", "rust", "--src", &input_file],
                    Some(&project_root),
=======
                    &get_cargo_home_bin("quicktype"), // Assuming quicktype is also in .cargo/bin
                    &["--lang", "rust", "--src", &input_file],
                    Some(project_root),
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
=======
                    &get_cargo_home_bin("quicktype"),
                    &["--lang", "rust", "--src", &input_file],
                    Some(&project_root),
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
                );
            } else {
                eprintln!("Usage: cargo xtask quicktype <input.json>");
                exit(1);
            }
        }
        Some("check") => {
            run_command(
                "cargo",
                &["check", "--all-targets", "--all-features"],
<<<<<<< HEAD
<<<<<<< HEAD
                Some(&project_root),
=======
                Some(project_root),
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
            );
        }
        Some("clippy") => {
            run_command(
                "cargo",
                &[
                    "clippy",
                    "--all-targets",
                    "--all-features",
                    "--",
                    "-D",
                    "warnings",
                ],
<<<<<<< HEAD
                Some(&project_root),
=======
                Some(project_root),
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
            );
=======
                Some(&project_root),
            );
        }
        Some("clippy") => {
            run_command("cargo", CARGO_CLIPPY_ARGS, Some(&project_root));
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
        }
        Some("doc") => {
            let mut doc_args = vec!["doc", "--no-deps"];
            if args.any(|arg| arg == "--open") {
                doc_args.push("--open");
            }
<<<<<<< HEAD
<<<<<<< HEAD
            run_command("cargo", &doc_args, Some(&project_root));
        }
        Some("mistake") => {
            match args.next().as_deref() {
                Some("add") => {
                    if let Some(new_entry_file_str_val) = args.next() {
                        let new_entry_file_str = &new_entry_file_str_val;
                        let new_entry_file = project_root.join(new_entry_file_str);
                        if !new_entry_file.exists() {
                            eprintln!("Error: New mistake entry file not found: {}\n (Full path: {:?})", new_entry_file_str, new_entry_file);
                            exit(1);
                        }
                        println!("Attempting to add new mistake from: {} (relative to project root)", new_entry_file_str);

                        if !validate_mistake_file(&new_entry_file, &project_root, false) {
                            eprintln!("Error: New mistake entry file {:?} is not valid according to the schema.", new_entry_file);
                            exit(1);
                        }
                        println!("New mistake entry file {:?} is valid.", new_entry_file);

                        let new_entry_content_str = fs::read_to_string(&new_entry_file)
                            .expect("Failed to read new mistake entry file.");
                        let new_entry_json: serde_json::Value = serde_json::from_str(&new_entry_content_str)
                            .expect("Failed to parse new mistake entry JSON.");

                        let mut mistakes_list: Vec<serde_json::Value> = if mistakes_log_file_path.exists() {
                            let log_content = fs::read_to_string(&mistakes_log_file_path)
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
                        fs::write(&mistakes_log_file_path, updated_log_content)
                            .expect("Failed to write updated mistakes.json log file.");
                        println!("Successfully added new entry to {:?}\nContent:\n{}", mistakes_log_file_path, fs::read_to_string(&mistakes_log_file_path).unwrap_or_default());

                        if !validate_mistake_file(&mistakes_log_file_path, &project_root, true) {
                            eprintln!("Error: The updated {:?} is not valid.", mistakes_log_file_path);
                            exit(1);
                        }
                        println!("Updated {:?} is valid.", mistakes_log_file_path);

                    } else {
                        eprintln!("Usage: cargo xtask mistake add <path_to_new_mistake_entry.json>");
                        exit(1);
                    }
                }
                Some("validate") => {
                    let file_to_validate_str_opt = args.next();
                    let (file_path_to_validate, is_main_log) = match file_to_validate_str_opt.as_deref() {
                        Some(f_str) => (project_root.join(f_str), f_str == "mistakes.json"),
                        None => (mistakes_log_file_path.clone(), true),
                    };

                    if validate_mistake_file(&file_path_to_validate, &project_root, is_main_log) {
                        println!("Mistake file {:?} is valid.", file_path_to_validate);
                    } else {
                        eprintln!("Mistake file {:?} is NOT valid.", file_path_to_validate);
                        exit(1);
                    }
                }
                Some(other_file_str_val) => {
                    let other_file_str = &other_file_str_val;
                    let file_path = project_root.join(other_file_str);
                     if validate_mistake_file(&file_path, &project_root, false) {
                        println!("File {:?} is valid.", file_path);
                    } else {
                        eprintln!("File {:?} is NOT valid.", file_path);
                        exit(1);
                    }
                }
                None => {
                    eprintln!("Usage: cargo xtask mistake <add|validate> [args]");
                    eprintln!("  cargo xtask mistake add <path_to_new_mistake_entry.json>");
                    eprintln!("  cargo xtask mistake validate [path_to_mistake_file.json] (defaults to mistakes.json)");
                    eprintln!("  cargo xtask mistake <path_to_mistake_file.json> (legacy: validates a specific single-entry file)");
                    exit(1);
                }
            }
        }
        Some("taplo-format") => {
            run_command(&taplo_path, &["format", "."], Some(&project_root));
        }
        Some("taplo-lint") => {
            run_command(&taplo_path, &["lint", "."], Some(&project_root));
        }
        Some("pre-commit-run") => {
            run_command(&get_cargo_home_bin("pre-commit"), &["run", "--all-files"], Some(&project_root));
        }
        Some("pre-commit-install") => {
            run_command(&get_cargo_home_bin("pre-commit"), &["install"], Some(&project_root));
        }
        Some("cargo-about") => {
            let output_target = if cfg!(windows) { "NUL" } else { "/dev/null" };
            run_command("cargo", &["about", "generate", "--format", "json", "-o", output_target], Some(&project_root));
        }
        Some("build") => {
            run_command("cargo", &["build", "--all-targets", "--all-features"], Some(&project_root));
        }
        Some("test") => {
            run_command("cargo", &["test", "--all-targets", "--all-features", "--verbose"], Some(&project_root));
=======
            run_command("cargo", &doc_args, Some(project_root));
=======
            run_command("cargo", &doc_args, Some(&project_root));
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
        }
        Some("mistake") => {
            match args.next().as_deref() {
                Some("add") => {
                    if let Some(new_entry_file_str_val) = args.next() {
                        let new_entry_file_str = &new_entry_file_str_val;
                        let new_entry_file = project_root.join(new_entry_file_str);
                        if !new_entry_file.exists() {
                            eprintln!("Error: New mistake entry file not found: {}\n (Full path: {:?})", new_entry_file_str, new_entry_file);
                            exit(1);
                        }
                        println!("Attempting to add new mistake from: {} (relative to project root)", new_entry_file_str);

                        if !validate_mistake_file(&new_entry_file, &project_root, false) {
                            eprintln!("Error: New mistake entry file {:?} is not valid according to the schema.", new_entry_file);
                            exit(1);
                        }
                        println!("New mistake entry file {:?} is valid.", new_entry_file);

                        let new_entry_content_str = fs::read_to_string(&new_entry_file)
                            .expect("Failed to read new mistake entry file.");
                        let new_entry_json: serde_json::Value = serde_json::from_str(&new_entry_content_str)
                            .expect("Failed to parse new mistake entry JSON.");

                        let mut mistakes_list: Vec<serde_json::Value> = if mistakes_log_file_path.exists() {
                            let log_content = fs::read_to_string(&mistakes_log_file_path)
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
                        fs::write(&mistakes_log_file_path, updated_log_content)
                            .expect("Failed to write updated mistakes.json log file.");
                        println!("Successfully added new entry to {:?}\nContent:\n{}", mistakes_log_file_path, fs::read_to_string(&mistakes_log_file_path).unwrap_or_default());

                        if !validate_mistake_file(&mistakes_log_file_path, &project_root, true) {
                            eprintln!("Error: The updated {:?} is not valid.", mistakes_log_file_path);
                            exit(1);
                        }
                        println!("Updated {:?} is valid.", mistakes_log_file_path);

                    } else {
                        eprintln!("Usage: cargo xtask mistake add <path_to_new_mistake_entry.json>");
                        exit(1);
                    }
                }
                Some("validate") => {
                    let file_to_validate_str_opt = args.next();
                    let (file_path_to_validate, is_main_log) = match file_to_validate_str_opt.as_deref() {
                        Some(f_str) => (project_root.join(f_str), f_str == "mistakes.json"),
                        None => (mistakes_log_file_path.clone(), true),
                    };

                    if validate_mistake_file(&file_path_to_validate, &project_root, is_main_log) {
                        println!("Mistake file {:?} is valid.", file_path_to_validate);
                    } else {
                        eprintln!("Mistake file {:?} is NOT valid.", file_path_to_validate);
                        exit(1);
                    }
                }
                Some(other_file_str_val) => {
                    let other_file_str = &other_file_str_val;
                    let file_path = project_root.join(other_file_str);
                     if validate_mistake_file(&file_path, &project_root, false) {
                        println!("File {:?} is valid.", file_path);
                    } else {
                        eprintln!("File {:?} is NOT valid.", file_path);
                        exit(1);
                    }
                }
                None => {
                    eprintln!("Usage: cargo xtask mistake <add|validate> [args]");
                    eprintln!("  cargo xtask mistake add <path_to_new_mistake_entry.json>");
                    eprintln!("  cargo xtask mistake validate [path_to_mistake_file.json] (defaults to mistakes.json)");
                    eprintln!("  cargo xtask mistake <path_to_mistake_file.json> (legacy: validates a specific single-entry file)");
                    exit(1);
                }
            }
        }
        Some("taplo-format") => {
            run_command(&taplo_path, &["format", "."], Some(&project_root));
        }
        Some("taplo-lint") => {
            run_command(&taplo_path, &["lint", "."], Some(&project_root));
        }
        Some("pre-commit-run") => {
            run_command(&get_cargo_home_bin("pre-commit"), &["run", "--all-files"], Some(&project_root));
        }
        Some("pre-commit-install") => {
            run_command(&get_cargo_home_bin("pre-commit"), &["install"], Some(&project_root));
        }
        Some("cargo-about") => {
            let output_target = if cfg!(windows) { "NUL" } else { "/dev/null" };
            run_command("cargo", &["about", "generate", "--format", "json", "-o", output_target], Some(&project_root));
        }
        Some("build") => {
            run_command("cargo", &["build", "--all-targets", "--all-features"], Some(&project_root));
        }
        Some("test") => {
<<<<<<< HEAD
            run_command("cargo", &["test", "--all-targets", "--all-features", "--verbose"], Some(project_root));
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
=======
            run_command("cargo", &["test", "--all-targets", "--all-features", "--verbose"], Some(&project_root));
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
        }
        Some("all") | Some("ci") => {
            println!("Running all CI checks (xtask ci)...");
            println!("\n==> Formatting TOML files (taplo-format)");
<<<<<<< HEAD
<<<<<<< HEAD
            run_command(&taplo_path, &["format", "."], Some(&project_root));

            println!("\n==> Linting TOML files (taplo-lint)");
            run_command(&taplo_path, &["lint", "."], Some(&project_root));

=======
            run_command(&taplo_path, &["format", "."], Some(project_root));
=======
            run_command(&taplo_path, &["format", "."], Some(&project_root));
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)

            println!("\n==> Linting TOML files (taplo-lint)");
            run_command(&taplo_path, &["lint", "."], Some(&project_root));

>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
            println!("\n==> Linting Rust code (cargo clippy)");
<<<<<<< HEAD
            run_command(
                "cargo",
                &[
                    "clippy",
                    "--all-targets",
                    "--all-features",
                    "--",
                    "-D",
                    "warnings",
                ],
<<<<<<< HEAD
                Some(&project_root),
            );

            println!("\n==> Running tests (cargo test)");
             run_command("cargo", &["test", "--all-targets", "--all-features", "--verbose"], Some(&project_root));
=======
                Some(project_root),
            );

            println!("\n==> Running tests (cargo test)");
             run_command("cargo", &["test", "--all-targets", "--all-features", "--verbose"], Some(project_root));
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
=======
            run_command("cargo", CARGO_CLIPPY_ARGS, Some(&project_root));

            println!("\n==> Running tests (cargo test)");
             run_command("cargo", &["test", "--all-targets", "--all-features", "--verbose"], Some(&project_root));
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)

            println!("\n==> Checking Rust code (cargo check)");
             run_command(
                "cargo",
                &["check", "--all-targets", "--all-features"],
<<<<<<< HEAD
<<<<<<< HEAD
                Some(&project_root),
            );

            println!("\n==> Building documentation (cargo doc)");
            run_command("cargo", &["doc", "--no-deps"], Some(&project_root));

            println!("\n==> Checking license compliance (cargo-about)");
            let output_target = if cfg!(windows) { "NUL" } else { "/dev/null" };
            run_command("cargo", &["about", "generate", "--format", "json", "-o", output_target], Some(&project_root));

=======
                Some(project_root),
=======
                Some(&project_root),
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
            );

            println!("\n==> Building documentation (cargo doc)");
            run_command("cargo", &["doc", "--no-deps"], Some(&project_root));

            println!("\n==> Checking license compliance (cargo-about)");
            let output_target = if cfg!(windows) { "NUL" } else { "/dev/null" };
            run_command("cargo", &["about", "generate", "--format", "json", "-o", output_target], Some(&project_root));

>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
            println!("\n==> Running pre-commit hooks (if configured and installed)");
            let pre_commit_path = get_cargo_home_bin("pre-commit");

            match Command::new(&pre_commit_path)
                .arg("--version")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
<<<<<<< HEAD
                .status();

            if pre_commit_check.is_ok() && pre_commit_check.unwrap().success() {
<<<<<<< HEAD
                 run_command(&pre_commit_path, &["run", "--all-files"], Some(&project_root));
=======
                 run_command(&pre_commit_path, &["run", "--all-files"], Some(project_root));
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
            } else {
                println!("Skipping pre-commit run as pre-commit does not seem to be installed or configured correctly at {}.", pre_commit_path);
=======
                .status()
            {
                Ok(status) => {
                    if status.success() {
                        run_command(&pre_commit_path, &["run", "--all-files"], Some(&project_root));
                    } else {
                        println!("Skipping pre-commit run as pre-commit --version did not succeed (is pre-commit configured correctly at {}?).", pre_commit_path);
                    }
                }
                Err(_) => {
                     println!("Failed to execute pre-commit --version (is pre-commit installed and in PATH at {}?). Skipping pre-commit run.", pre_commit_path);
                }
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
            }
            println!("\nAll CI checks passed!");
        }
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
<<<<<<< HEAD
<<<<<<< HEAD
    eprintln!("  quicktype <input.json>     Generates Rust types from JSON using quicktype.");
    eprintln!("  mistake <subcommand> [args] Handles LLM mistake logging:");
    eprintln!("    add <entry.json>         Adds a new mistake entry from a JSON file to mistakes.json and validates.");
    eprintln!("    validate [file.json]     Validates mistakes.json (default) or a specified JSON mistake log/entry (validates each entry if it's an array).");
    eprintln!("    <file.json>              (Legacy) Validates a specific single-entry JSON mistake file.");
=======
    // eprintln!("  check-license-headers      Checks SPDX license headers using '.mise/lint/legal/spdx' script."); // Temporarily removed
    // eprintln!("                             (Note: --fix-it is announced but not implemented by the script)"); // Temporarily removed
    // eprintln!("  fix-legal-copyright        Runs 'cargo xtask check_license_headers --fix-it' (existing xtask)."); // Temporarily removed
    eprintln!("  quicktype <input.json>     Generates Rust types from JSON using quicktype.");
    eprintln!("  mistake <report.json>      Processes a mistake report (requires 'fuckup' command).");
>>>>>>> ecc0b6f (feat: Migrate build and dev tasks from mise to cargo xtask)
=======
    eprintln!("  quicktype <input.json>     Generates Rust types from JSON using quicktype.");
    eprintln!("  mistake <subcommand> [args] Handles LLM mistake logging:");
    eprintln!("    add <entry.json>         Adds a new mistake entry from a JSON file to mistakes.json and validates.");
    eprintln!("    validate [file.json]     Validates mistakes.json (default) or a specified JSON mistake log/entry (validates each entry if it's an array).");
    eprintln!("    <file.json>              (Legacy) Validates a specific single-entry JSON mistake file.");
>>>>>>> 6d83e08 (refactor: Apply review suggestions to xtask pre-commit check and clippy calls)
    eprintln!("  all / ci                   Runs all common CI checks: taplo format/lint, clippy, test, check, doc, cargo-about, pre-commit.");
    eprintln!("\nNote: Some tasks require external tools like taplo, pre-commit, cargo-about, quicktype, or fuckup to be installed, typically via `cargo install <tool>` or system package managers, and their binaries accessible (often via $CARGO_HOME/bin or $HOME/.cargo/bin).");
}
