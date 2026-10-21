/// WebView2 must not keep browser accelerator keys. The desktop chords are
/// those keys; while the setting stays at its default they never reach script.
pub const WEBVIEW2_BROWSER_ACCELERATOR_KEYS: bool = false;

/// Standard edit accelerators. On macOS these only register as predefined
/// application-menu items (`PredefinedMenuItem::copy` and the rest).
pub const EDIT_MENU_ITEMS: &[EditMenuItem] = &[
    EditMenuItem {
        id: "undo",
        label: "Undo",
    },
    EditMenuItem {
        id: "redo",
        label: "Redo",
    },
    EditMenuItem {
        id: "cut",
        label: "Cut",
    },
    EditMenuItem {
        id: "copy",
        label: "Copy",
    },
    EditMenuItem {
        id: "paste",
        label: "Paste",
    },
    EditMenuItem {
        id: "select-all",
        label: "Select All",
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EditMenuItem {
    pub id: &'static str,
    pub label: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuShortcut {
    pub id: &'static str,
    pub label: &'static str,
    /// Accelerator string accepted by Tauri's menu items (`CmdOrCtrl+KeyT`).
    pub accelerator: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Modifier {
    /// Command on macOS, Control on Windows and Linux. Matches `Mod` in
    /// `@tanstack/react-hotkeys` and `metaKey || ctrlKey` in the chrome.
    Mod,
    /// Always Control, including on macOS (`Ctrl+Tab` cycles tabs).
    Control,
    /// Control and Shift (`Ctrl+Shift+Tab`).
    ControlShift,
    /// Alt with no other modifier (`Alt+ArrowLeft` / `Alt+ArrowRight`).
    Alt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Shortcut {
    menu: MenuShortcut,
    key: &'static str,
    code: &'static str,
    modifier: Modifier,
}

const SHORTCUTS: &[Shortcut] = &[
    Shortcut {
        menu: MenuShortcut {
            id: "new-tab",
            label: "New Tab",
            accelerator: "CmdOrCtrl+KeyT",
        },
        key: "t",
        code: "KeyT",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "close-tab",
            label: "Close Tab",
            accelerator: "CmdOrCtrl+KeyW",
        },
        key: "w",
        code: "KeyW",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "focus-url",
            label: "Open URL",
            accelerator: "CmdOrCtrl+KeyL",
        },
        key: "l",
        code: "KeyL",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "back",
            label: "Back",
            accelerator: "CmdOrCtrl+BracketLeft",
        },
        key: "[",
        code: "BracketLeft",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "forward",
            label: "Forward",
            accelerator: "CmdOrCtrl+BracketRight",
        },
        key: "]",
        code: "BracketRight",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "cycle-next",
            label: "Select Next Tab",
            accelerator: "Ctrl+Tab",
        },
        key: "Tab",
        code: "Tab",
        modifier: Modifier::Control,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "cycle-previous",
            label: "Select Previous Tab",
            accelerator: "Ctrl+Shift+Tab",
        },
        key: "Tab",
        code: "Tab",
        modifier: Modifier::ControlShift,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "tab-1",
            label: "Select Tab 1",
            accelerator: "CmdOrCtrl+Digit1",
        },
        key: "1",
        code: "Digit1",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "tab-2",
            label: "Select Tab 2",
            accelerator: "CmdOrCtrl+Digit2",
        },
        key: "2",
        code: "Digit2",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "tab-3",
            label: "Select Tab 3",
            accelerator: "CmdOrCtrl+Digit3",
        },
        key: "3",
        code: "Digit3",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "tab-4",
            label: "Select Tab 4",
            accelerator: "CmdOrCtrl+Digit4",
        },
        key: "4",
        code: "Digit4",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "tab-5",
            label: "Select Tab 5",
            accelerator: "CmdOrCtrl+Digit5",
        },
        key: "5",
        code: "Digit5",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "tab-6",
            label: "Select Tab 6",
            accelerator: "CmdOrCtrl+Digit6",
        },
        key: "6",
        code: "Digit6",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "tab-7",
            label: "Select Tab 7",
            accelerator: "CmdOrCtrl+Digit7",
        },
        key: "7",
        code: "Digit7",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "tab-8",
            label: "Select Tab 8",
            accelerator: "CmdOrCtrl+Digit8",
        },
        key: "8",
        code: "Digit8",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "tab-last",
            label: "Select Last Tab",
            accelerator: "CmdOrCtrl+Digit9",
        },
        key: "9",
        code: "Digit9",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "command-menu",
            label: "Command Menu",
            accelerator: "CmdOrCtrl+KeyK",
        },
        key: "k",
        code: "KeyK",
        modifier: Modifier::Mod,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "history-back",
            label: "Back",
            accelerator: "Alt+ArrowLeft",
        },
        key: "ArrowLeft",
        code: "ArrowLeft",
        modifier: Modifier::Alt,
    },
    Shortcut {
        menu: MenuShortcut {
            id: "history-forward",
            label: "Forward",
            accelerator: "Alt+ArrowRight",
        },
        key: "ArrowRight",
        code: "ArrowRight",
        modifier: Modifier::Alt,
    },
];

/// Menu items the shell should register on the application menu.
pub fn menu_shortcuts() -> impl Iterator<Item = MenuShortcut> {
    SHORTCUTS.iter().map(|shortcut| shortcut.menu)
}

/// `keydown` the page already understands. `meta_is_mod` is true on macOS,
/// where `Mod` is the Command key, and false on Windows and Linux.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DispatchEvent {
    pub key: &'static str,
    pub code: &'static str,
    pub meta_key: bool,
    pub ctrl_key: bool,
    pub alt_key: bool,
    pub shift_key: bool,
}

pub fn key_event(id: &str, meta_is_mod: bool) -> Option<DispatchEvent> {
    let shortcut = SHORTCUTS.iter().find(|shortcut| shortcut.menu.id == id)?;
    let (meta_key, ctrl_key, alt_key, shift_key) = match shortcut.modifier {
        Modifier::Mod if meta_is_mod => (true, false, false, false),
        Modifier::Mod => (false, true, false, false),
        Modifier::Control => (false, true, false, false),
        Modifier::ControlShift => (false, true, false, true),
        Modifier::Alt => (false, false, true, false),
    };
    Some(DispatchEvent {
        key: shortcut.key,
        code: shortcut.code,
        meta_key,
        ctrl_key,
        alt_key,
        shift_key,
    })
}

/// JavaScript for `webview.eval` on the `main` webview. Unknown ids,
/// including the tray ids, return `None` so those events stay with the tray.
pub fn dispatch_script(id: &str, meta_is_mod: bool) -> Option<String> {
    let event = key_event(id, meta_is_mod)?;
    Some(format!(
        "document.dispatchEvent(new KeyboardEvent('keydown',{{key:'{}',code:'{}',bubbles:true,cancelable:true,metaKey:{},ctrlKey:{},altKey:{},shiftKey:{}}}))",
        event.key,
        event.code,
        js_bool(event.meta_key),
        js_bool(event.ctrl_key),
        js_bool(event.alt_key),
        js_bool(event.shift_key),
    ))
}

fn js_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tray menu ids already used by the shell. Shortcut ids must not reuse
    /// them, because the application menu and the tray share one event channel.
    const TRAY_MENU_IDS: &[&str] = &["show", "hide", "update", "quit"];

    /// What the desktop chrome's `keydown` listener does with an event.
    /// Mirrors `DesktopWorkspace` in `web/src/features/desktop/desktop-chrome.tsx`.
    fn chrome_action(event: DispatchEvent) -> Option<&'static str> {
        if event.ctrl_key && event.key == "Tab" {
            return Some(if event.shift_key {
                "cycle-previous"
            } else {
                "cycle-next"
            });
        }
        let mod_key = event.meta_key || event.ctrl_key;
        if mod_key && !event.alt_key && !event.shift_key {
            let key = event.key.to_ascii_lowercase();
            return Some(match key.as_str() {
                "t" => "new-tab",
                "w" => "close-tab",
                "l" => "focus-url",
                "[" => "back",
                "]" => "forward",
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" => "select-tab",
                // The chrome treats 9 as the last tab, same as index 999.
                "9" => "select-last-tab",
                _ => return None,
            });
        }
        if event.alt_key && !event.meta_key && !event.ctrl_key && !event.shift_key {
            return match event.key {
                "ArrowLeft" => Some("history-back"),
                "ArrowRight" => Some("history-forward"),
                _ => None,
            };
        }
        None
    }

    /// `useHotkey('Mod+K')` resolves Mod to Meta on macOS and Control elsewhere.
    fn command_menu_matches(event: DispatchEvent, meta_is_mod: bool) -> bool {
        if event.alt_key || event.shift_key || event.key != "k" {
            return false;
        }
        if meta_is_mod {
            event.meta_key && !event.ctrl_key
        } else {
            event.ctrl_key && !event.meta_key
        }
    }

    #[test]
    fn browser_accelerator_keys_stay_off() {
        assert!(!WEBVIEW2_BROWSER_ACCELERATOR_KEYS);
    }

    #[test]
    fn edit_menu_covers_the_text_shortcuts_macos_drops() {
        let ids: Vec<_> = EDIT_MENU_ITEMS.iter().map(|item| item.id).collect();
        assert_eq!(ids, ["undo", "redo", "cut", "copy", "paste", "select-all"]);
    }

    #[test]
    fn shortcut_ids_do_not_collide_with_the_tray_menu() {
        for shortcut in menu_shortcuts() {
            assert!(
                !TRAY_MENU_IDS.contains(&shortcut.id),
                "{} collides with the tray menu",
                shortcut.id
            );
        }
    }

    #[test]
    fn accelerators_and_ids_are_unique() {
        let shortcuts: Vec<_> = menu_shortcuts().collect();
        for (index, shortcut) in shortcuts.iter().enumerate() {
            assert!(
                shortcuts[..index]
                    .iter()
                    .all(|other| other.id != shortcut.id),
                "duplicate id {}",
                shortcut.id
            );
            assert!(
                shortcuts[..index]
                    .iter()
                    .all(|other| other.accelerator != shortcut.accelerator),
                "duplicate accelerator {}",
                shortcut.accelerator
            );
        }
    }

    #[test]
    fn unknown_and_tray_ids_do_not_dispatch() {
        assert_eq!(dispatch_script("quit", true), None);
        assert_eq!(dispatch_script("show", false), None);
        assert_eq!(key_event("not-a-shortcut", true), None);
    }

    #[test]
    fn each_shortcut_reaches_the_handler_the_page_already_has() {
        for meta_is_mod in [true, false] {
            for shortcut in SHORTCUTS {
                let event = key_event(shortcut.menu.id, meta_is_mod).unwrap();
                let script = dispatch_script(shortcut.menu.id, meta_is_mod).unwrap();
                assert!(script.starts_with("document.dispatchEvent(new KeyboardEvent('keydown',"));
                assert!(script.contains("bubbles:true"));
                assert!(script.contains(&format!("key:'{}'", event.key)));
                assert!(script.contains(&format!("code:'{}'", event.code)));

                if shortcut.menu.id == "command-menu" {
                    assert!(command_menu_matches(event, meta_is_mod));
                    assert_eq!(chrome_action(event), None);
                    continue;
                }
                assert!(!command_menu_matches(event, meta_is_mod));
                let expected = match shortcut.menu.id {
                    "tab-1" | "tab-2" | "tab-3" | "tab-4" | "tab-5" | "tab-6" | "tab-7"
                    | "tab-8" => "select-tab",
                    "tab-last" => "select-last-tab",
                    other => other,
                };
                assert_eq!(chrome_action(event), Some(expected));
            }
        }
    }

    #[test]
    fn mod_follows_the_platform_and_ctrl_tab_does_not() {
        let mac_new_tab = key_event("new-tab", true).unwrap();
        assert!(mac_new_tab.meta_key && !mac_new_tab.ctrl_key);

        let win_new_tab = key_event("new-tab", false).unwrap();
        assert!(win_new_tab.ctrl_key && !win_new_tab.meta_key);

        // Ctrl+Tab cycles tabs on macOS too. Cmd+Tab belongs to the OS.
        let mac_cycle = key_event("cycle-next", true).unwrap();
        assert!(mac_cycle.ctrl_key && !mac_cycle.meta_key && !mac_cycle.shift_key);
        let mac_cycle_back = key_event("cycle-previous", true).unwrap();
        assert!(mac_cycle_back.ctrl_key && mac_cycle_back.shift_key);

        let back = key_event("history-back", true).unwrap();
        assert!(back.alt_key && !back.meta_key && !back.ctrl_key);
        assert_eq!(back.key, "ArrowLeft");
    }

    #[test]
    fn digit_nine_selects_the_last_tab() {
        let event = key_event("tab-last", true).unwrap();
        assert_eq!(chrome_action(event), Some("select-last-tab"));
        assert_eq!(
            chrome_action(key_event("tab-1", false).unwrap()),
            Some("select-tab")
        );
    }
}
