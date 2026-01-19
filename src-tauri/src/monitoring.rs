//! Continuous Monitoring Module for 4DA
//!
//! Provides background analysis scheduling, system tray integration,
//! and native OS notifications for new relevant items.

use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder},
    AppHandle, Emitter, Manager, Runtime,
};
use tauri_plugin_notification::NotificationExt;

// ============================================================================
// Monitoring State
// ============================================================================

/// Global monitoring state
pub struct MonitoringState {
    /// Whether monitoring is enabled
    pub enabled: AtomicBool,
    /// Interval between checks in seconds
    pub interval_secs: AtomicU64,
    /// Whether a check is currently running
    pub is_checking: AtomicBool,
    /// Last check timestamp (unix seconds)
    pub last_check: AtomicU64,
    /// Number of new relevant items found in last check
    pub last_relevant_count: AtomicU64,
    /// Total checks performed
    pub total_checks: AtomicU64,
}

impl Default for MonitoringState {
    fn default() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            interval_secs: AtomicU64::new(1800), // 30 minutes default
            is_checking: AtomicBool::new(false),
            last_check: AtomicU64::new(0),
            last_relevant_count: AtomicU64::new(0),
            total_checks: AtomicU64::new(0),
        }
    }
}

impl MonitoringState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn get_interval(&self) -> u64 {
        self.interval_secs.load(Ordering::Relaxed)
    }

    pub fn set_interval(&self, secs: u64) {
        self.interval_secs.store(secs, Ordering::Relaxed);
    }
}

// ============================================================================
// System Tray
// ============================================================================

/// Set up the system tray with menu
pub fn setup_tray<R: Runtime>(app: &AppHandle<R>) -> Result<TrayIcon<R>, String> {
    // Create menu items
    let show_item = MenuItem::with_id(app, "show", "Show 4DA Home", true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let analyze_item = MenuItem::with_id(app, "analyze", "Analyze Now", true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let separator = PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?;
    let monitoring_item = MenuItem::with_id(
        app,
        "toggle_monitoring",
        "Start Monitoring",
        true,
        None::<&str>,
    )
    .map_err(|e| e.to_string())?;
    let separator2 = PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?;
    let quit_item =
        MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).map_err(|e| e.to_string())?;

    // Build menu
    let menu = Menu::with_items(
        app,
        &[
            &show_item,
            &analyze_item,
            &separator,
            &monitoring_item,
            &separator2,
            &quit_item,
        ],
    )
    .map_err(|e| e.to_string())?;

    // Build tray icon
    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("4DA Home - The internet searches for you")
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "analyze" => {
                    // Emit event to trigger analysis
                    let _ = app.emit("tray-analyze", ());
                }
                "toggle_monitoring" => {
                    // Emit event to toggle monitoring
                    let _ = app.emit("tray-toggle-monitoring", ());
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)
        .map_err(|e| e.to_string())?;

    println!("[4DA/Tray] System tray initialized");
    Ok(tray)
}

// ============================================================================
// Background Scheduler
// ============================================================================

/// Start the background monitoring scheduler
pub fn start_scheduler<R: Runtime>(app: AppHandle<R>, state: Arc<MonitoringState>) {
    println!("[4DA/Monitor] Starting background scheduler");

    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60)); // Check every minute

        loop {
            interval.tick().await;

            // Check if monitoring is enabled
            if !state.is_enabled() {
                continue;
            }

            // Check if enough time has passed since last check
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let last = state.last_check.load(Ordering::Relaxed);
            let interval_secs = state.get_interval();

            if now - last < interval_secs {
                continue;
            }

            // Check if already running
            if state.is_checking.swap(true, Ordering::SeqCst) {
                continue;
            }

            println!("[4DA/Monitor] Running scheduled analysis...");
            state.last_check.store(now, Ordering::Relaxed);
            state.total_checks.fetch_add(1, Ordering::Relaxed);

            // Emit event to run analysis
            let _ = app.emit("scheduled-analysis", ());

            // Analysis completion will be handled by the main analysis flow
            // and will call complete_scheduled_check when done
        }
    });
}

/// Called when a scheduled analysis completes
pub fn complete_scheduled_check<R: Runtime>(
    app: &AppHandle<R>,
    state: &MonitoringState,
    new_relevant_count: usize,
    total_count: usize,
) {
    state.is_checking.store(false, Ordering::Relaxed);
    state
        .last_relevant_count
        .store(new_relevant_count as u64, Ordering::Relaxed);

    // Send notification if we found new relevant items
    if new_relevant_count > 0 {
        send_notification(app, new_relevant_count, total_count);
    }

    println!(
        "[4DA/Monitor] Scheduled check complete: {}/{} relevant",
        new_relevant_count, total_count
    );
}

// ============================================================================
// Notifications
// ============================================================================

/// Send a notification about new relevant items
pub fn send_notification<R: Runtime>(
    app: &AppHandle<R>,
    relevant_count: usize,
    _total_count: usize,
) {
    let title = "4DA Home - New Relevant Items";
    let body = if relevant_count == 1 {
        "1 new item matches your interests".to_string()
    } else {
        format!("{} new items match your interests", relevant_count)
    };

    // Use Tauri's notification plugin
    if let Err(e) = app.notification().builder().title(title).body(&body).show() {
        println!("[4DA/Notify] Failed to send notification: {}", e);
    } else {
        println!("[4DA/Notify] Sent notification: {}", body);
    }
}

// ============================================================================
// Tray Menu Updates
// ============================================================================

/// Update the tray menu text based on monitoring state
pub fn update_tray_menu<R: Runtime>(
    tray: &TrayIcon<R>,
    app: &AppHandle<R>,
    monitoring_enabled: bool,
) -> Result<(), String> {
    let show_item = MenuItem::with_id(app, "show", "Show 4DA Home", true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let analyze_item = MenuItem::with_id(app, "analyze", "Analyze Now", true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let separator = PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?;

    let monitoring_text = if monitoring_enabled {
        "Stop Monitoring"
    } else {
        "Start Monitoring"
    };
    let monitoring_item = MenuItem::with_id(
        app,
        "toggle_monitoring",
        monitoring_text,
        true,
        None::<&str>,
    )
    .map_err(|e| e.to_string())?;

    let separator2 = PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?;
    let quit_item =
        MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).map_err(|e| e.to_string())?;

    let menu = Menu::with_items(
        app,
        &[
            &show_item,
            &analyze_item,
            &separator,
            &monitoring_item,
            &separator2,
            &quit_item,
        ],
    )
    .map_err(|e| e.to_string())?;

    tray.set_menu(Some(menu)).map_err(|e| e.to_string())?;

    // Update tooltip
    let tooltip = if monitoring_enabled {
        "4DA Home - Monitoring active"
    } else {
        "4DA Home - The internet searches for you"
    };
    tray.set_tooltip(Some(tooltip)).map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_state_default() {
        let state = MonitoringState::default();
        assert!(!state.is_enabled());
        assert_eq!(state.get_interval(), 1800);
    }

    #[test]
    fn test_monitoring_state_toggle() {
        let state = MonitoringState::new();
        assert!(!state.is_enabled());
        state.set_enabled(true);
        assert!(state.is_enabled());
        state.set_enabled(false);
        assert!(!state.is_enabled());
    }
}
