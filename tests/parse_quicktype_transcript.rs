// SPDX-License-Identifier: ISC
// Copyright (c) 2024 Ben Giles and contributors
// Permission to use, copy, modify, and/or distribute this file for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
// See the LICENSE file in the project root for full license text.

use std::path::PathBuf;
use zen404::{parse_transcript_from_file, parse_transcript_from_str, ParseError};

// Helper function to get the full path to a test data file
fn get_test_data_path(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data");
    path.push(file_name);
    path
}

#[test]
fn parses_sample_yt_transcript_successfully() {
    let path = get_test_data_path("yt_transcript.sample.json3");
    let parsed_result = parse_transcript_from_file(&path);

    assert!(
        parsed_result.is_ok(),
        "Failed to parse sample transcript {:?}: {:?}",
        path,
        parsed_result.err()
    );

    let transcript = parsed_result.unwrap();

    // Check root fields exist and have expected values or characteristics
    assert_eq!(transcript.wire_magic, "pb3", "wireMagic field mismatch");

    // Pens might be empty or have specific content depending on the sample
    // For now, just assert its presence. Add more specific checks if needed.
    assert!(!transcript.pens.is_empty(), "Pens array should not be empty (based on typical YouTube transcript structure, though sample might differ)");

    // wsWinStyles and wpWinPositions might also be empty or have specific content
    assert!(!transcript.ws_win_styles.is_empty(), "wsWinStyles should not be empty");
    assert!(!transcript.wp_win_positions.is_empty(), "wpWinPositions should not be empty");

    assert!(!transcript.events.is_empty(), "Events array should not be empty");

    // Check characteristics of the first event, if it exists
    if let Some(first_event) = transcript.events.first() {
        // t_start_ms and d_duration_ms are usually non-negative
        assert!(first_event.t_start_ms >= 0, "First event t_start_ms should be non-negative");
        assert!(first_event.d_duration_ms >= 0, "First event d_duration_ms should be non-negative");

        // Check segments if they exist
        if let Some(segs) = &first_event.segs {
            assert!(!segs.is_empty(), "First event segs array, if present, should not be empty");
            if let Some(first_seg) = segs.first() {
                assert!(
                    !first_seg.utf8.is_empty(),
                    "First segment's utf8 text should not be empty"
                );
                // Optionally check for ac_asr_conf or t_offset_ms if their presence is guaranteed or important
            }
        } else {
            // If no segments, perhaps other fields are mandatory for this event type
            // This depends on the specific logic of how events are structured.
            // For a simple transcript, events usually have segments.
            // If an event can be valid without 'segs', this assertion might need adjustment
            // or other fields on the event should be checked for validity.
            // For now, we'll assume events typically have segments.
            // If yt_transcript.sample.json3 has events without segs, this part will need an update.
            // Consider if events without 'segs' are valid or if 'segs' should always be Some([]).
            println!(
                "Warning: First event has no segments. This might be valid depending on the data."
            );
        }
    }

    // Check that at least one event has either segs or the required numeric fields
    // This assertion was from the original test, let's refine it based on Transcript structure.
    // Given `t_start_ms` and `d_duration_ms` are not optional in `Event`, they will always be present.
    // The main content is usually in `segs`.
    let found_event_with_content =
        transcript.events.iter().any(|event| event.segs.as_ref().is_some_and(|s| !s.is_empty()));
    assert!(found_event_with_content, "At least one event must have non-empty segs.");

    // Add more specific assertions based on known content of yt_transcript.sample.json3
    // For example, count of events, specific text in a segment, etc.
    // e.g. assert_eq!(transcript.events.len(), EXPECTED_NUMBER_OF_EVENTS);
}

#[test]
fn parse_empty_json_input_string() {
    let result = parse_transcript_from_str("");
    assert!(result.is_err(), "Parsing an empty string should result in an error.");
    if let Err(e) = result {
        println!("Correctly failed to parse empty string: {}", e);
        match e {
            ParseError::Json(json_err) => {
                assert!(json_err.is_eof(), "Error for empty string should be EOF")
            }
            _ => panic!("Unexpected error type for empty string parsing"),
        }
    }
}

#[test]
fn parse_malformed_json_string() {
    let malformed_json = r#"{"wireMagic": "pb3", "events": [ { "tStartMs": 0, "dDurationMs": 100, "segs": [{"utf8": "test"}]  }"#; // Missing closing bracket and brace
    let result = parse_transcript_from_str(malformed_json);
    assert!(result.is_err(), "Parsing malformed JSON should result in an error.");
    if let Err(e) = result {
        println!("Correctly failed to parse malformed JSON: {}", e);
        match e {
            ParseError::Json(json_err) => assert!(
                json_err.is_eof() || json_err.is_syntax(),
                "Error for malformed JSON should be EOF or Syntax"
            ),
            _ => panic!("Unexpected error type for malformed JSON parsing"),
        }
    }
}

#[test]
fn parse_json_with_unexpected_fields_at_root() {
    // serde by default ignores unknown fields. This test confirms that behavior.
    let json_with_extra_field = r#"{
        "wireMagic": "pb3",
        "pens": [],
        "wsWinStyles": [],
        "wpWinPositions": [],
        "events": [{ "tStartMs": 0, "dDurationMs": 100, "segs": [{"utf8": "test"}] }],
        "unexpectedField": "some_value"
    }"#;
    let result = parse_transcript_from_str(json_with_extra_field);
    assert!(result.is_ok(), "Parsing JSON with an extra (unexpected) field should still succeed (serde default behavior). Error: {:?}", result.err());
    if let Ok(transcript) = result {
        assert_eq!(transcript.wire_magic, "pb3");
        assert_eq!(transcript.events.len(), 1);
    }
}

#[test]
fn parse_json_with_missing_required_field_wire_magic() {
    // `wireMagic` is not Option<T>, so it's required by serde.
    let json_missing_field = r#"{
        "pens": [],
        "wsWinStyles": [],
        "wpWinPositions": [],
        "events": [{ "tStartMs": 0, "dDurationMs": 100, "segs": [{"utf8": "test"}] }]
    }"#;
    let result = parse_transcript_from_str(json_missing_field);
    assert!(result.is_err(), "Parsing JSON missing a required field (wireMagic) should fail.");
    if let Err(e) = result {
        println!("Correctly failed to parse JSON with missing wireMagic: {}", e);
        match e {
            ParseError::Json(json_err) => {
                assert!(json_err.is_data(), "Error for missing field should be a data error")
            }
            _ => panic!("Unexpected error type for missing field parsing"),
        }
    }
}

#[test]
fn parse_json_with_incorrect_type_for_field() {
    let json_wrong_type = r#"{
        "wireMagic": "pb3",
        "pens": [],
        "wsWinStyles": [],
        "wpWinPositions": [],
        "events": [{ "tStartMs": "not_a_number", "dDurationMs": 100, "segs": [{"utf8": "test"}] }]
    }"#;
    let result = parse_transcript_from_str(json_wrong_type);
    assert!(result.is_err(), "Parsing JSON with incorrect type for tStartMs should fail.");
    if let Err(e) = result {
        println!("Correctly failed to parse JSON with incorrect type: {}", e);
        match e {
            ParseError::Json(json_err) => {
                assert!(json_err.is_data(), "Error for type mismatch should be a data error")
            }
            _ => panic!("Unexpected error type for type mismatch parsing"),
        }
    }
}
