//! xtask: metatask CLI and workflow entrypoint for project automation.
mod tasks;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exit_code = tasks::dispatch(&args[1..]);
    std::process::exit(exit_code);
}
