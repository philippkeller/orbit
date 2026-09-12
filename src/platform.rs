//! Per-OS tuning for audio device-loss recovery, plus diagnostics.

/// Per-OS knobs for the device-loss recovery state machine.
#[derive(Clone, Copy, Debug)]
pub struct PlatformTuning {
    /// Whether the Tier-2 position-stall heuristic runs. Linux only — elsewhere
    /// cpal's error callback (Tier 1) is authoritative, so the heuristic is off.
    pub heuristic_enabled: bool,
    /// Consecutive no-progress ticks before the heuristic declares a stall.
    pub stall_limit_ticks: u32,
    /// Consecutive failed/re-stalled rebuilds before giving up.
    pub rebuild_cap: u32,
    /// Ticks after a rebuild during which new lost-events/stalls are ignored (debounce).
    pub rebuild_window_ticks: u32,
}

/// Post a desktop notification (macOS only).
pub fn show_notification(title: &str, body: &str, subtitle: Option<&str>) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        return macos::show_notification(title, body, subtitle);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (title, body, subtitle);
        Err("desktop notifications are only supported on macOS".into())
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use std::process::Command;

    fn esc_applescript(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', " ")
            .replace('\r', " ")
    }

    pub fn show_notification(title: &str, body: &str, subtitle: Option<&str>) -> Result<(), String> {
        let script = match subtitle {
            Some(sub) => format!(
                "display notification \"{}\" with title \"{}\" subtitle \"{}\"",
                esc_applescript(body),
                esc_applescript(title),
                esc_applescript(sub),
            ),
            None => format!(
                "display notification \"{}\" with title \"{}\"",
                esc_applescript(body),
                esc_applescript(title),
            ),
        };
        let status = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("osascript failed ({status})"))
        }
    }
}

/// Tuning for the host platform. `tick()` runs at 20 Hz.
pub fn tuning() -> PlatformTuning {
    PlatformTuning {
        heuristic_enabled: cfg!(target_os = "linux"),
        stall_limit_ticks: 60,    // ~3s — generous for suspend-on-idle / XRUNs
        rebuild_cap: 3,
        rebuild_window_ticks: 20, // ~1s debounce after a rebuild
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuning_constants_are_sane() {
        let t = tuning();
        assert!(t.rebuild_cap >= 1, "must allow at least one rebuild");
        assert!(t.stall_limit_ticks > 0);
        // The stall heuristic is Linux-only; everywhere else Tier-1 events are authoritative.
        assert_eq!(t.heuristic_enabled, cfg!(target_os = "linux"));
    }
}
