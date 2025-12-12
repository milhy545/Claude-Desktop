// Debug utilities and logging

use std::sync::Once;
use std::time::Instant;

static INIT: Once = Once::new();

/// Initialize logging (call once at startup)
pub fn init_logging() {
    INIT.call_once(|| {
        #[cfg(debug_assertions)]
        {
            // Development: verbose logging
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
                .init();
            log::debug!("🐛 Debug logging enabled");
        }

        #[cfg(not(debug_assertions))]
        {
            // Production: only errors and warnings
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
                .init();
        }
    });
}

/// Log system information for debugging
pub fn log_system_info() {
    log::info!("🦀 Claude Desktop (Tauri) v{}", env!("CARGO_PKG_VERSION"));
    log::info!("📦 OS: {} {}", std::env::consts::OS, std::env::consts::ARCH);

    #[cfg(debug_assertions)]
    log::debug!("🏗️  Build: Debug");

    #[cfg(not(debug_assertions))]
    log::info!("🚀 Build: Release");
}

/// Performance timer for debugging and production monitoring
pub struct PerfTimer {
    name: String,
    start: Instant,
    threshold_ms: Option<u64>,
}

impl PerfTimer {
    /// Create a new performance timer
    pub fn new(name: &str) -> Self {
        log::debug!("⏱️  Starting: {}", name);
        Self {
            name: name.to_string(),
            start: Instant::now(),
            threshold_ms: None,
        }
    }

    /// Create a timer that logs warning if exceeds threshold (production monitoring)
    pub fn with_threshold(name: &str, threshold_ms: u64) -> Self {
        log::debug!("⏱️  Starting: {} (threshold: {}ms)", name, threshold_ms);
        Self {
            name: name.to_string(),
            start: Instant::now(),
            threshold_ms: Some(threshold_ms),
        }
    }
}

impl Drop for PerfTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        let elapsed_ms = elapsed.as_millis() as u64;

        if let Some(threshold) = self.threshold_ms {
            if elapsed_ms > threshold {
                log::warn!(
                    "⚠️  Slow operation: {} took {:.2?} (threshold: {}ms)",
                    self.name,
                    elapsed,
                    threshold
                );
            } else {
                log::debug!("✅ Finished: {} ({:.2?})", self.name, elapsed);
            }
        } else {
            log::debug!("✅ Finished: {} ({:.2?})", self.name, elapsed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging() {
        init_logging();
        // Should not panic
    }

    #[test]
    fn test_perf_timer() {
        let _timer = PerfTimer::new("test");
        std::thread::sleep(std::time::Duration::from_millis(1));
        // Should log on drop
    }
}
