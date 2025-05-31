// xtask/tasks.rs
// Handles dispatch and execution for custom project automation tasks.

pub fn dispatch(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("No xtask provided. Try: cargo xtask <clippy|check|doc|mistake|migration>");
        return 1;
    }
    match args[0].as_str() {
        "clippy" => {
            let status = std::process::Command::new("cargo")
                .args(["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])
                .status()
                .expect("Failed to run cargo clippy");
            status.code().unwrap_or(1)
        }
        "check" => {
            let status = std::process::Command::new("cargo")
                .args(["check", "--all-targets", "--all-features"])
                .status()
                .expect("Failed to run cargo check");
            status.code().unwrap_or(1)
        }
        "doc" => {
            let status = std::process::Command::new("cargo")
                .args(["doc", "--no-deps", "--all-features"])
                .status()
                .expect("Failed to run cargo doc");
            status.code().unwrap_or(1)
        }
        "mistake" => {
            eprintln!("xtask 'mistake': Not yet implemented.");
            1
        }
        "migration" => {
            eprintln!("xtask 'migration': Not yet implemented.");
            0
        }
        arg => {
            eprintln!("Unknown xtask: {arg}");
            1
        }
    }
}
