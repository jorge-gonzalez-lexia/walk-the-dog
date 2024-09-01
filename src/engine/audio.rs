use anyhow::{anyhow, Result};
use web_sys::{AudioBuffer, AudioContext};

use crate::{browser, sound};

#[derive(Clone)]
pub struct Audio {
    context: AudioContext,
}

impl Audio {
    pub fn new() -> Result<Self> {
        Ok(Audio {
            context: sound::create_audio_context()?,
        })
    }

    pub async fn load_sound(&self, filename: &str) -> Result<Sound> {
        let array_buffer = browser::fetch_array_buffer(filename).await?;
        let buffer = sound::decode_audio_data(&self.context, array_buffer).await?;

        Ok(Sound { buffer })
    }

    pub fn play_sound(&self, sound: &Sound) -> Result<()> {
        sound::play_sound(&self.context, &sound.buffer)
    }
}

#[derive(Clone)]
pub struct Sound {
    buffer: AudioBuffer,
}
