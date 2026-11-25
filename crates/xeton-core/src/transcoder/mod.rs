// xeton_core::transcoder — FFmpeg wrappers for media processing.

pub mod extractor;
pub mod ffi;
pub mod merger;

#[derive(Clone, Debug, uniffi::Enum)]
pub enum AudioFormat {
    Mp3,
    Aac,
    Opus,
    Flac,
}
