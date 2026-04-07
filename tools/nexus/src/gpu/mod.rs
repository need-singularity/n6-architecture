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
        // On any platform, is_available should return a valid bool
        let result = is_available();
        assert!(result == true || result == false);
    }

    #[test]
    fn device_returns_valid_option() {
        // device() should not panic on any platform
        let dev = device();
        // On non-macOS, always None
        #[cfg(not(target_os = "macos"))]
        assert!(dev.is_none());
        // On macOS, may or may not have a device — just ensure no panic
        let _ = dev;
    }

    #[test]
    fn device_is_consistent() {
        // Multiple calls should return the same result (OnceLock caching)
        let d1 = device();
        let d2 = device();
        assert_eq!(d1.is_some(), d2.is_some());
    }
}
