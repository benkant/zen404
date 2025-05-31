// SPDX-License-Identifier: ISC
// Copyright (c) 2024 Ben Giles and contributors
// Permission to use, copy, modify, and/or distribute this file for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
// See the LICENSE file in the project root for full license text.

use std::env;
use std::process::exit;
use zen404::parse_transcript_from_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_json3_file>", args[0]);
        exit(1);
    }

    let file_path = &args[1];
    println!("Attempting to parse transcript from: {}", file_path);

    match parse_transcript_from_file(file_path) {
        Ok(transcript) => {
            println!("Successfully parsed transcript!");

            println!("Wire Magic: {}", transcript.wire_magic);
            println!("Number of Pens: {}", transcript.pens.len());
            println!("Number of Window Styles: {}", transcript.ws_win_styles.len());
            println!("Number of Window Positions: {}", transcript.wp_win_positions.len());
            println!("Number of Events: {}", transcript.events.len());

            if let Some(first_event) = transcript.events.first() {
                println!("\n--- First Event Details ---");
                println!("  Start Time (ms): {}", first_event.t_start_ms);
                println!("  Duration (ms): {}", first_event.d_duration_ms);
                if let Some(id) = first_event.id {
                    println!("  ID: {}", id);
                }
                if let Some(wp_id) = first_event.wp_win_pos_id {
                    println!("  Window Position ID: {}", wp_id);
                }
                if let Some(ws_id) = first_event.ws_win_style_id {
                    println!("  Window Style ID: {}", ws_id);
                }
                if let Some(w_id) = first_event.w_win_id {
                    println!("  Window ID: {}", w_id);
                }
                if let Some(append) = first_event.a_append {
                    println!("  Append: {}", append);
                }

                if let Some(segs) = &first_event.segs {
                    println!("  Segments ({}):", segs.len());
                    for (i, seg) in segs.iter().take(3).enumerate() {
                        // Print first 3 segments
                        print!("    Seg {}: '{}'", i + 1, seg.utf8);
                        if let Some(conf) = seg.ac_asr_conf {
                            print!(" (Confidence: {})", conf);
                        }
                        if let Some(offset) = seg.t_offset_ms {
                            print!(" (Offset: {}ms)", offset);
                        }
                        println!();
                    }
                    if segs.len() > 3 {
                        println!("    ... and {} more segments", segs.len() - 3);
                    }
                } else {
                    println!("  No segments in first event.");
                }
            } else {
                println!("\nNo events found in the transcript.");
            }

            // For full debug output:
            // println!("\n--- Full Transcript (Debug) ---");
            // println!("{:?}", transcript);
        }
        Err(e) => {
            eprintln!("Error parsing transcript: {}", e);
            exit(1);
        }
    }
}
