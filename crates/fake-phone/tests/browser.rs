//! LudusCtrl's browser bridge against `fake-phone browser`, over loopback: the link the settings
//! show, the TLS handshake, the token, input into the one pad, and the neutral packet at the end.

use std::{
    net::Ipv4Addr,
    process::Command,
    sync::{Arc, Condvar, Mutex, MutexGuard},
    time::{Duration, Instant},
};

use anyps5_core::remote::{
    BrowserConfig, LudusConfig, LudusEvents, LudusServer, LudusStatus, PadTarget, PairingRequest,
    vpad::PadState,
};

struct Quiet;

impl LudusEvents for Quiet {
    fn pairing(&self, _: PairingRequest) {}
    fn pairing_ended(&self, _: u64) {}
    fn status(&self, _: LudusStatus) {}
}

/// Every state the pad was given, with a condvar that wakes whoever waits for the next one.
#[derive(Default)]
struct Pad {
    writes: Mutex<Vec<PadState>>,
    changed: Condvar,
}

impl Pad {
    /// A failed assertion on the test thread must not poison this lock for the server's threads,
    /// or the one real failure is buried under a `PoisonError`.
    fn lock(&self) -> MutexGuard<'_, Vec<PadState>> {
        self.writes.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Waits until the writes satisfy `done`, and returns a copy of them. Fails with `what` and
    /// the writes seen so far if that does not happen before the deadline.
    fn wait_for(&self, what: &str, done: impl Fn(&[PadState]) -> bool) -> Vec<PadState> {
        let deadline = Instant::now() + Duration::from_secs(20);
        let mut writes = self.lock();
        while !done(&writes) {
            let left = deadline.saturating_duration_since(Instant::now());
            assert!(
                !left.is_zero(),
                "timed out waiting for {what}; the pad got {writes:?}"
            );
            writes = self
                .changed
                .wait_timeout(writes, left)
                .unwrap_or_else(|e| e.into_inner())
                .0;
        }
        writes.clone()
    }
}

impl PadTarget for Pad {
    fn write(&self, state: &PadState) -> bool {
        self.lock().push(*state);
        self.changed.notify_all();
        true
    }
    fn release(&self) {}
}

#[test]
fn the_fake_browser_plays_through_the_link() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = LudusConfig::new(dir.path().join("LudusCtrl.json"));
    config.port = 0;
    config.bind = Some(Ipv4Addr::LOCALHOST.into());
    config.beacon = None;
    let server = LudusServer::start(config, Arc::new(Quiet)).unwrap();
    let pad = Arc::new(Pad::default());
    server.set_pad(Some(pad.clone()));
    let mut browser = BrowserConfig::new(dir.path().join("LudusCtrlWeb.json"));
    browser.port = 0;
    browser.bind = Some(Ipv4Addr::LOCALHOST.into());
    let info = server.start_browser(browser).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_fake-phone"))
        .args([
            "browser",
            "--url",
            &info.urls[0],
            "--press",
            "cross+r1",
            "--seconds",
            "1",
        ])
        .output()
        .expect("the fake phone runs");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{stdout}");
    for line in ["< ready", "< padkind:x360", "< ack:0:1:1", "< pong:"] {
        assert!(stdout.contains(line), "{line} missing in:\n{stdout}");
    }
    // The phone has left, but the server may not have handled its last packets yet: wait for the
    // press and for the neutral state after it instead of reading whatever is there.
    let writes = pad.wait_for("the press and the release after it", |w| {
        w.iter()
            .rposition(|s| s.buttons == 0x1000 | 0x0200)
            .is_some_and(|press| w[press..].last() == Some(&PadState::default()))
    });
    assert_eq!(
        writes.last(),
        Some(&PadState::default()),
        "released at the end"
    );
}
