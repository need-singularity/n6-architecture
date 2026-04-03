//! GPU architecture analysis and n=6 SM/HBM verification.
pub mod buffer_pool;
pub mod fallback;

use std::sync::OnceLock;

/// Check whether a Metal GPU device is available on this system.
pub fn is_available() -> bool {
    #[cfg(target_os = "macos")]
    {
        metal::Device::system_default().is_some()
    }
    #[cfg(not(target_os = "macos"))]
    {
        false
    }
}

/// Cached Metal device handle. Returns None on non-macOS or if no GPU found.
#[cfg(target_os = "macos")]
static DEVICE: OnceLock<Option<metal::Device>> = OnceLock::new();

/// Get the default Metal device (cached). Returns None if unavailable.
#[cfg(target_os = "macos")]
pub fn device() -> Option<&'static metal::Device> {
    DEVICE
        .get_or_init(|| metal::Device::system_default())
        .as_ref()
}

#[cfg(not(target_os = "macos"))]
pub fn device() -> Option<&'static ()> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_available_returns_bool() {
        // On macOS this may be true or false depending on hardware;
        // on other platforms always false. Just verify it doesn't panic.
        let _avail = is_available();
    }

    #[test]
    fn device_returns_without_panic() {
        let _dev = device();
    }
}
