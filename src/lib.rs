//! Keyboard shortcuts for the Budalerts desktop shell.
//!
//! The web client already handles these chords: tab management in
//! `DesktopWorkspace`, back and forward in `registerDesktopLinks`, and
//! the command menu via `useHotkey('Mod+K')`. Those listeners sit on the
//! page. The desktop webview never delivers the chords to the page, so the
//! listeners never run:
//!
//! - WebView2 treats Ctrl+T, Ctrl+W, Ctrl+L, Ctrl+K, Ctrl+Tab, Ctrl+1–9,
//!   and Alt+Left/Right as browser accelerator keys and consumes them
//!   before script.
//! - WKWebView does not receive Cmd chords while an in-app child webview
//!   is focused, and Cmd+W closes the window instead of a tab.
//! - macOS copy, paste, cut, undo, and select-all do not register unless
//!   the application menu has those accelerators. This shell only builds
//!   a tray menu.
//!
//! Register [`menu_shortcuts`] on the application menu (not the tray) and
//! eval [`dispatch_script`] for that menu item into the `main` webview.
//! The script dispatches a bubbling `keydown` on `document`, which is the
//! target TanStack Hotkeys listens on and which then bubbles to the
//! `window` listener in the desktop chrome. Also append [`EDIT_MENU_ITEMS`]
//! with Tauri's predefined menu items, and turn WebView2 browser
//! accelerator keys off ([`WEBVIEW2_BROWSER_ACCELERATOR_KEYS`]).

mod shortcuts;

pub use shortcuts::{
    dispatch_script, key_event, menu_shortcuts, DispatchEvent, EditMenuItem, MenuShortcut,
    EDIT_MENU_ITEMS, WEBVIEW2_BROWSER_ACCELERATOR_KEYS,
};
