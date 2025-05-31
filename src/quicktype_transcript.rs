// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Transcript;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Transcript = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transcript {
    pub wire_magic: String,
    pub pens: Vec<Pen>,
    pub ws_win_styles: Vec<WsWinStyle>,
    pub wp_win_positions: Vec<WpWinPosition>,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub t_start_ms: i64,
    pub d_duration_ms: i64,
    pub id: Option<i64>,
    pub wp_win_pos_id: Option<i64>,
    pub ws_win_style_id: Option<i64>,
    pub w_win_id: Option<i64>,
    pub segs: Option<Vec<Seg>>,
    pub a_append: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Seg {
    pub utf8: String,
    pub ac_asr_conf: Option<i64>,
    pub t_offset_ms: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pen {
    // Add fields if known, otherwise it's an empty struct
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WpWinPosition {
    pub ap_point: Option<i64>,
    pub ah_hor_pos: Option<i64>,
    pub av_ver_pos: Option<i64>,
    pub rc_rows: Option<i64>,
    pub cc_cols: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WsWinStyle {
    pub mh_mode_hint: Option<i64>,
    pub ju_justif_code: Option<i64>,
    pub sd_scroll_dir: Option<i64>,
}
