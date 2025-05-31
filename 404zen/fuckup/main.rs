use std::io::{self, Read};
use std::path::PathBuf;
use serde_json::Value;
use jsonschema::{JSONSchema, Draft};

fn main() {
    // Where is the canonical schema?
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| ".".to_string());
    let schema_path = PathBuf::from(manifest_dir)
        .join("tests/schemas/llm_mistake_report_schema.json");

    // Load schema
    let schema_str = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| {
            eprintln!("Unable to read schema at {:?}", schema_path);
            std::process::exit(2);
        });
    let schema_json: Value = serde_json::from_str(&schema_str)
        .unwrap_or_else(|_| {
            eprintln!("Unable to parse schema JSON at {:?}", schema_path);
            std::process::exit(2);
        });
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_json)
        .unwrap_or_else(|e| {
            eprintln!("Schema failed to compile: {e}");
            std::process::exit(2);
        });

    // Read stdin
    let mut stdin = io::stdin();
    let mut input = String::new();
    if let Err(e) = stdin.read_to_string(&mut input) {
        eprintln!("Failed to read stdin: {e}");
        std::process::exit(2);
    }
    let instance: Value = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Invalid JSON: {e}");
            std::process::exit(1);
        }
    };

    let validation_result = compiled.validate(&instance);
    match validation_result {
        Ok(()) => std::process::exit(0),
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
            std::process::exit(1);
        }
    };
}
