use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::collections::HashMap;

use crate::browser;

#[derive(Clone, Deserialize)]
pub struct Sheet {
    pub frames: HashMap<String, Cell>,
}

impl Sheet {
    pub async fn load(json_path: &str) -> Result<Self> {
        let json = browser::fetch_json(json_path).await?;

        serde_wasm_bindgen::from_value::<Sheet>(json)
            .map_err(|err| anyhow!("Error deserializing {} {:#?}", json_path, err))
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub frame: SheetRect,
    pub sprite_source_size: SheetRect,
}

#[derive(Clone, Deserialize)]
pub struct SheetRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}
