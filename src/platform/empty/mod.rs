use crate::{MediaControlEvent, MediaMetadata, MediaPlayback, PlatformConfig};

/// A handle to OS media controls.
pub struct MediaControls;

impl MediaControls {
    /// Create media controls with the specified config.
    pub fn new(_config: PlatformConfig) -> Result<Self, std::convert::Infallible> {
        Ok(Self)
    }

    /// Attach the media control events to a handler.
    pub fn attach<F>(&mut self, _event_handler: F) -> Result<(), std::convert::Infallible>
    where
        F: Fn(MediaControlEvent) + Send + 'static,
    {
        Ok(())
    }

    /// Detach the event handler.
    pub fn detach(&mut self) -> Result<(), std::convert::Infallible> {
        Ok(())
    }

    /// Set the current playback status.
    pub fn set_playback(
        &mut self,
        _playback: MediaPlayback,
    ) -> Result<(), std::convert::Infallible> {
        Ok(())
    }

    /// Set the metadata of the currently playing media item.
    pub fn set_metadata(
        &mut self,
        _metadata: MediaMetadata,
    ) -> Result<(), std::convert::Infallible> {
        Ok(())
    }
}
