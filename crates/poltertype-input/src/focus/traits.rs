//! The focus-tracking extension point.

use super::types::{CaretHint, FocusedWindowGeometry};

/// Best-effort identifier of the currently-focused application.
pub trait FocusTracker: Send + Sync {
    /// File-name of the focused process's executable, e.g.
    /// `"Code.exe"` / `"alacritty"`. Returns `None` if no foreground
    /// window exists, the OS denies the query, or this platform's
    /// implementation is a stub.
    fn focused_exe(&self) -> Option<String>;

    /// Geometry of the focused window, when the backend can answer.
    /// Default `None` — callers must treat geometry as a bonus, never
    /// a given (GNOME/KDE Wayland has no path to it).
    ///
    /// Not TTL-cached like [`Self::focused_exe`]: it is queried once
    /// per suggestion-tooltip show, not on the per-keystroke path.
    fn focused_window_geometry(&self) -> Option<FocusedWindowGeometry> {
        None
    }

    /// Last known on-screen caret position, when a caret source is
    /// running. Bonus data with the same caveats as
    /// [`Self::focused_window_geometry`] — many apps expose it, none
    /// guarantee it — queried once per tooltip show, never on the
    /// keystroke path.
    ///
    /// Check [`CaretHint::age`] first: a stale sample means the focused
    /// app emits no a11y caret events, and anchoring to the window is
    /// then the better answer.
    fn caret_hint(&self) -> Option<CaretHint> {
        None
    }

    /// Best-effort: is the currently-focused text entry a password /
    /// secret field?
    ///
    /// `[engine].ignore_in_password_fields` reads this before the
    /// engine buffers, corrects or lets the manual switch-last hotkey
    /// touch a word — see `docs/DECISIONS.md`. Default `false`, the
    /// same fail-safe direction as every other hint here: **`false`
    /// also means "cannot tell"**, not just "confirmed not a
    /// password" — a platform with no implementation, an app with no
    /// accessibility bridge, and an ordinary text field all answer
    /// alike. That is deliberate: a hint that leaned the other way
    /// would silence auto-switch everywhere the OS cannot answer,
    /// which is most terminals and every platform but Linux today.
    /// See `docs/KNOWN-GAPS.md`.
    fn is_password_field(&self) -> bool {
        false
    }

    fn backend_name(&self) -> &'static str;
}
