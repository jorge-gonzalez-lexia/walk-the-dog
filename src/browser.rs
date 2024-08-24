use anyhow::{anyhow, Result};
use web_sys::{Document, Window};

macro_rules!log {
  ($($t:tt)*) => {
    web_sys::consol::log_1($format!($($t)*).into());
  }
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("No Document Found"))
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}
