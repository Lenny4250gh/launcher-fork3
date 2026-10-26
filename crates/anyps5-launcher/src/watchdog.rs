//! The UI watchdog: a background thread that notices when the interface has stopped.
//!
//! The UI thread reports each frame ([`frame_begin`], [`frame_end`]) and what it shows
//! ([`set_screen`], [`set_action`]). The watchdog wakes once a second and writes one line to
//! `launcher.log` when
//!
//! * a frame has been running for more than [`LIMIT`] (a deadlock or a blocking call inside the
//!   interface code, the 0.2.8 freeze), or
//! * the launcher is focused and animating, so frames are due every few milliseconds, yet none has
//!   finished for that long (a blocked present or a stuck event loop).
//!
//! The line names the screen and the last action. On Windows it also records the UI thread's call
//! stack as module offsets. A second line says when the interface moves again. The watchdog never
//! ends the process and never touches the UI thread beyond reading its stack.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// How long a frame may run, or a focused animating launcher may draw nothing, before the log
/// says so.
pub const LIMIT: Duration = Duration::from_secs(5);
/// How often the watchdog looks.
const TICK: Duration = Duration::from_secs(1);

fn epoch() -> Instant {
    static EPOCH: OnceLock<Instant> = OnceLock::new();
    *EPOCH.get_or_init(Instant::now)
}

/// Milliseconds since the epoch, never 0.
fn now_ms() -> u64 {
    epoch().elapsed().as_millis() as u64 + 1
}

/// When the running frame began (0: no frame runs).
static BEGUN: AtomicU64 = AtomicU64::new(0);
/// When the last frame ended (0: none yet).
static ENDED: AtomicU64 = AtomicU64::new(0);
/// Frames are due all the time: the window has focus and the backdrop moves.
static EXPECT_FRAMES: AtomicBool = AtomicBool::new(false);
/// The operating system's id of the UI thread (0: unknown).
static UI_THREAD: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
struct Notes {
    screen: String,
    action: String,
}

fn notes() -> &'static Mutex<Notes> {
    static NOTES: OnceLock<Mutex<Notes>> = OnceLock::new();
    NOTES.get_or_init(Mutex::default)
}

/// Starts the watchdog thread. Call once, from the UI thread, before the first frame.
pub fn start() {
    let _ = epoch();
    UI_THREAD.store(u64::from(os_thread_id()), Ordering::Relaxed);
    let _ = std::thread::Builder::new()
        .name("watchdog".into())
        .spawn(watch);
}

/// The UI thread begins a frame.
pub fn frame_begin() {
    BEGUN.store(now_ms(), Ordering::Release);
}

/// The UI thread has finished a frame. `expect_frames`: the next one is due within milliseconds.
pub fn frame_end(expect_frames: bool) {
    ENDED.store(now_ms(), Ordering::Release);
    BEGUN.store(0, Ordering::Release);
    EXPECT_FRAMES.store(expect_frames, Ordering::Relaxed);
}

/// What the player is looking at ("home", "Settings", ...). Call when it changes.
pub fn set_screen(screen: impl Into<String>) {
    if let Ok(mut notes) = notes().lock() {
        notes.screen = screen.into();
    }
}

/// The last thing the player did ("Down", "Tab", ...). Never text the player typed.
pub fn set_action(action: impl Into<String>) {
    if let Ok(mut notes) = notes().lock() {
        notes.action = action.into();
    }
}

/// What the watchdog decides from the two stamps; apart from the thread so a test can drive it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Fine,
    /// A frame has been running this long.
    FrameStuck(Duration),
    /// No frame has finished for this long although they are due.
    NoFrames(Duration),
}

/// `begun`/`ended` are the stamps in milliseconds (0: none), `now` the same clock.
pub fn judge(begun: u64, ended: u64, expect_frames: bool, now: u64) -> Verdict {
    let limit = LIMIT.as_millis() as u64;
    if begun != 0 && now.saturating_sub(begun) > limit {
        return Verdict::FrameStuck(Duration::from_millis(now - begun));
    }
    if begun == 0 && expect_frames && ended != 0 && now.saturating_sub(ended) > limit {
        return Verdict::NoFrames(Duration::from_millis(now - ended));
    }
    Verdict::Fine
}

fn watch() {
    let mut stuck_since: Option<u64> = None;
    loop {
        std::thread::sleep(TICK);
        let now = now_ms();
        let verdict = judge(
            BEGUN.load(Ordering::Acquire),
            ENDED.load(Ordering::Acquire),
            EXPECT_FRAMES.load(Ordering::Relaxed),
            now,
        );
        match (verdict, stuck_since) {
            (Verdict::Fine, Some(since)) => {
                stuck_since = None;
                tracing::warn!(
                    target: "watchdog",
                    "the interface moves again after about {} s",
                    now.saturating_sub(since) / 1000
                );
            }
            (Verdict::FrameStuck(for_how_long) | Verdict::NoFrames(for_how_long), None) => {
                stuck_since = Some(now.saturating_sub(for_how_long.as_millis() as u64));
                report(verdict, for_how_long);
            }
            _ => {}
        }
    }
}

fn report(verdict: Verdict, for_how_long: Duration) {
    // `try_lock`: this is the moment to trust nothing the UI thread may hold.
    let (screen, action) = match notes().try_lock() {
        Ok(notes) => (notes.screen.clone(), notes.action.clone()),
        Err(_) => (String::new(), String::new()),
    };
    let what = match verdict {
        Verdict::FrameStuck(_) => "a frame has not finished",
        _ => "no frame has finished although the window is focused and animating",
    };
    tracing::error!(
        target: "watchdog",
        "the interface is frozen: {what} for {} s; screen: {}; last action: {}",
        for_how_long.as_secs(),
        if screen.is_empty() { "unknown" } else { &screen },
        if action.is_empty() { "none" } else { &action },
    );
    let id = UI_THREAD.load(Ordering::Relaxed) as u32;
    if let Some(stack) = ui_stack(id) {
        tracing::error!(target: "watchdog", "the interface thread is at: {stack}");
    }
}

#[cfg(windows)]
fn os_thread_id() -> u32 {
    // SAFETY: plain call.
    unsafe { windows_sys::Win32::System::Threading::GetCurrentThreadId() }
}

#[cfg(not(windows))]
fn os_thread_id() -> u32 {
    0
}

/// The UI thread's call stack as `module+offset` entries, innermost first.
#[cfg(not(all(windows, target_arch = "x86_64")))]
fn ui_stack(_thread: u32) -> Option<String> {
    None
}

#[cfg(all(windows, target_arch = "x86_64"))]
fn ui_stack(thread: u32) -> Option<String> {
    stack::capture(thread)
}

#[cfg(all(windows, target_arch = "x86_64"))]
mod stack {
    use windows_sys::Win32::Foundation::{CloseHandle, HMODULE};
    use windows_sys::Win32::System::Diagnostics::Debug::{
        CONTEXT, GetThreadContext, RtlLookupFunctionEntry, RtlVirtualUnwind,
    };
    use windows_sys::Win32::System::LibraryLoader::{
        GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
        GetModuleFileNameW, GetModuleHandleExW,
    };
    use windows_sys::Win32::System::Threading::{
        OpenThread, ResumeThread, SuspendThread, THREAD_GET_CONTEXT, THREAD_QUERY_INFORMATION,
        THREAD_SUSPEND_RESUME,
    };

    const DEPTH: usize = 24;
    /// `CONTEXT_FULL` of x86-64: control, integer and floating-point registers.
    const CONTEXT_FULL: u32 = 0x0010_000B;

    /// Walks the thread's stack with the unwind tables the system keeps for every module.
    ///
    /// The thread is suspended only while the raw addresses are copied into a fixed array: nothing
    /// is allocated, logged or locked in that time (the thread may hold the allocator's lock).
    /// The names are looked up after it runs again.
    pub fn capture(thread: u32) -> Option<String> {
        if thread == 0 {
            return None;
        }
        let mut frames = [0usize; DEPTH];
        let mut count = 0;
        // SAFETY: the calls follow their documented contracts; `context` is plain data and aligned
        // by its type; the unwind functions only read the module tables and update `context`.
        unsafe {
            let handle = OpenThread(
                THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT | THREAD_QUERY_INFORMATION,
                0,
                thread,
            );
            if handle.is_null() {
                return None;
            }
            if SuspendThread(handle) != u32::MAX {
                let mut context: CONTEXT = std::mem::zeroed();
                context.ContextFlags = CONTEXT_FULL;
                if GetThreadContext(handle, &mut context) != 0 {
                    while count < DEPTH && context.Rip != 0 {
                        frames[count] = context.Rip as usize;
                        count += 1;
                        let mut base = 0u64;
                        let entry =
                            RtlLookupFunctionEntry(context.Rip, &mut base, std::ptr::null_mut());
                        if entry.is_null() {
                            break;
                        }
                        let mut data = std::ptr::null_mut();
                        let mut frame = 0u64;
                        let pc = context.Rip;
                        RtlVirtualUnwind(
                            0,
                            base,
                            pc,
                            entry,
                            &mut context,
                            &mut data,
                            &mut frame,
                            std::ptr::null_mut(),
                        );
                    }
                }
                ResumeThread(handle);
            }
            CloseHandle(handle);
        }
        if count == 0 {
            return None;
        }
        Some(
            frames[..count]
                .iter()
                .map(|&address| describe(address))
                .collect::<Vec<_>>()
                .join(" < "),
        )
    }

    /// `module.dll+0x1234` for an address.
    fn describe(address: usize) -> String {
        let mut module: HMODULE = std::ptr::null_mut();
        // SAFETY: FROM_ADDRESS treats the argument as an address; the module is not retained.
        let found = unsafe {
            GetModuleHandleExW(
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS
                    | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                address as *const u16,
                &mut module,
            )
        };
        if found == 0 || module.is_null() {
            return format!("{address:#x}");
        }
        let mut name = [0u16; 260];
        // SAFETY: the buffer length is passed.
        let length = unsafe { GetModuleFileNameW(module, name.as_mut_ptr(), name.len() as u32) };
        let full = String::from_utf16_lossy(&name[..length as usize]);
        let short = full.rsplit(['\\', '/']).next().unwrap_or(&full);
        format!("{short}+{:#x}", address - module as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_frame_that_runs_too_long_is_stuck() {
        assert_eq!(judge(1_000, 900, true, 3_000), Verdict::Fine);
        assert_eq!(
            judge(1_000, 900, false, 7_000),
            Verdict::FrameStuck(Duration::from_secs(6))
        );
    }

    #[test]
    fn silence_counts_only_when_frames_are_due() {
        // Idle and unfocused: no frames for a minute is normal.
        assert_eq!(judge(0, 1_000, false, 61_000), Verdict::Fine);
        assert_eq!(
            judge(0, 1_000, true, 7_000),
            Verdict::NoFrames(Duration::from_secs(6))
        );
        // Never drawn yet: the window is still starting.
        assert_eq!(judge(0, 0, true, 61_000), Verdict::Fine);
    }
}
