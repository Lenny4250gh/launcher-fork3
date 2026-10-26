//! The operating system's own emoji picker (docs/DESIGN.md, Messages, "Writing").
//!
//! The launcher has no picker of its own: the composer's smiley asks the system to open its panel,
//! and what the player picks arrives in the focused field as text like any other typing. Windows
//! has no call for it; apps open the panel the way the player would, with Win + Period, sent to the
//! window that has the focus. The Mac has `orderFrontCharacterPalette`. Linux has no picker for all
//! desktops, so there [`AVAILABLE`] is false and the composer shows no glyph.

/// Whether this platform has a system emoji picker the launcher can open.
pub const AVAILABLE: bool = cfg!(any(windows, target_os = "macos"));

/// Opens the picker. The caller has just given the text field the focus (the picker types into the
/// focused window), and calls this from the UI thread.
pub fn open() {
    imp::open();
}

#[cfg(windows)]
mod imp {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VK_LWIN,
        VK_OEM_PERIOD,
    };

    fn key(vk: u16, up: bool) -> INPUT {
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: 0,
                    dwFlags: if up { KEYEVENTF_KEYUP } else { 0 },
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }
    }

    pub fn open() {
        // Win down, Period down, Period up, Win up: one call, so no other input slips in between.
        let inputs = [
            key(VK_LWIN, false),
            key(VK_OEM_PERIOD, false),
            key(VK_OEM_PERIOD, true),
            key(VK_LWIN, true),
        ];
        // SAFETY: `inputs` is an array of fully initialised INPUT structs and the size is theirs.
        let sent = unsafe {
            SendInput(
                inputs.len() as u32,
                inputs.as_ptr(),
                std::mem::size_of::<INPUT>() as i32,
            )
        };
        if sent as usize != inputs.len() {
            tracing::warn!(
                "emoji picker: SendInput sent {sent} of {} events",
                inputs.len()
            );
        }
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use objc2::MainThreadMarker;
    use objc2_app_kit::NSApplication;

    pub fn open() {
        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        NSApplication::sharedApplication(mtm).orderFrontCharacterPalette(None);
    }
}

#[cfg(not(any(windows, target_os = "macos")))]
mod imp {
    pub fn open() {}
}
