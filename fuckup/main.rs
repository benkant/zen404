use anyhow::{Context, Result};
use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> Result<()> {
    // Where is the canonical schema?
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .context("Failed to get CARGO_MANIFEST_DIR environment variable")?;
    let schema_path =
        PathBuf::from(manifest_dir).join("tests/schemas/llm_mistake_report_schema.json");

    // Load schema
    let schema_str = std::fs::read_to_string(&schema_path)
        .with_context(|| format!("Schema Error: Unable to read schema at {:?}", schema_path))?;
    let schema_json_val: Value = serde_json::from_str(&schema_str).with_context(|| {
        format!("Schema Error: Unable to parse schema JSON at {:?}", schema_path)
    })?;

    // Leak the schema Value to get a 'static reference, as required by JSONSchema::compile
    let static_schema_json: &'static Value = Box::leak(Box::new(schema_json_val));

    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(static_schema_json) // Pass the &'static Value
        .with_context(|| "Schema Error: Failed to compile schema".to_string())?;

    // Read stdin
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).context("Failed to read from stdin")?;

    let instance: Value = serde_json::from_str(&input)
        .with_context(|| format!("Invalid JSON: Failed to parse stdin content: {}", input))?;

    let validation_result = compiled.validate(&instance);
    match validation_result {
        Ok(()) => {
            // Optionally print a success message to stderr if verbose output is desired
            // eprintln!("Input JSON is valid against the schema.");
            std::process::exit(0)
        }
        Err(errors) => {
            eprintln!("Input JSON is NOT valid against the schema:");
            for err in errors {
                eprintln!("  - {}", err);
            }
            std::process::exit(1);
        }
    }
}
