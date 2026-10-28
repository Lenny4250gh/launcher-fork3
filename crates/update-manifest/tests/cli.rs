//! The release workflow's steps, run for real: keygen, manifest, sign, verify. The keys are made
//! by the test in a temporary folder.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn tool(args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_update-sign"));
    command.args(args);
    for (name, value) in envs {
        command.env(name, value);
    }
    command.output().expect("run update-sign")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn path(p: &Path) -> &str {
    p.to_str().unwrap()
}

#[test]
fn keys_manifest_signature_and_verification_work_end_to_end() {
    let dir = tempfile::tempdir().unwrap();
    let secret = dir.path().join("secret.key");
    let keys = dir.path().join("update-keys.txt");

    let out = tool(
        &[
            "keygen",
            "--secret-out",
            path(&secret),
            "--public-out",
            path(&keys),
            "--label",
            "test",
        ],
        &[],
    );
    assert!(out.status.success(), "{}", text(&out.stderr));
    let seed = fs::read_to_string(&secret).unwrap().trim().to_owned();
    assert_eq!(seed.len(), 64);
    // The secret reaches neither output.
    assert!(!text(&out.stdout).contains(&seed) && !text(&out.stderr).contains(&seed));
    assert!(text(&out.stdout).starts_with("ed25519 "));
    // An existing secret is never overwritten.
    let again = tool(&["keygen", "--secret-out", path(&secret)], &[]);
    assert!(!again.status.success());
    assert_eq!(fs::read_to_string(&secret).unwrap().trim(), seed);

    let exe = dir.path().join("AnyPlay-Launcher-0.2.0-x64.exe");
    fs::write(&exe, b"MZ-the-new-launcher").unwrap();
    let manifest = dir.path().join("latest.json");
    let sig = dir.path().join("latest.json.sig");
    let out = tool(
        &[
            "manifest",
            "--version",
            "v0.2.0",
            "--exe",
            path(&exe),
            "--out",
            path(&manifest),
            "--minimum",
            "0.1.0",
        ],
        &[],
    );
    assert!(out.status.success(), "{}", text(&out.stderr));
    let json = fs::read_to_string(&manifest).unwrap();
    assert!(json.contains("\"version\": \"0.2.0\""), "{json}");
    assert!(json.contains("releases/download/v0.2.0/AnyPlay-Launcher-0.2.0-x64.exe"));

    // Signing from the environment, like the workflow.
    let out = tool(
        &[
            "sign",
            "--manifest",
            path(&manifest),
            "--out",
            path(&sig),
            "--key-env",
            "TEST_SIGNING_KEY",
        ],
        &[("TEST_SIGNING_KEY", &seed)],
    );
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert!(!text(&out.stdout).contains(&seed) && !text(&out.stderr).contains(&seed));

    let verify = |current: &str| {
        tool(
            &[
                "verify",
                "--manifest",
                path(&manifest),
                "--sig",
                path(&sig),
                "--keys",
                path(&keys),
                "--current",
                current,
            ],
            &[],
        )
    };
    assert!(verify("0.1.3").status.success());
    // A rollback is refused by the same check.
    assert!(!verify("0.2.0").status.success());

    // A manifest changed after signing no longer verifies.
    fs::write(&manifest, json.replace("0.2.0", "0.9.0")).unwrap();
    assert!(!verify("0.1.3").status.success());
}

#[test]
fn bad_input_is_refused_and_the_secret_is_never_echoed() {
    let dir = tempfile::tempdir().unwrap();
    let exe = dir.path().join("renamed.exe");
    fs::write(&exe, b"MZ").unwrap();
    let manifest = dir.path().join("latest.json");
    // The exe must carry the name the launcher stages.
    let out = tool(
        &[
            "manifest",
            "--version",
            "0.2.0",
            "--exe",
            path(&exe),
            "--out",
            path(&manifest),
        ],
        &[],
    );
    assert!(!out.status.success());
    // Longer than the cap.
    let named = dir.path().join("AnyPlay-Launcher-0.2.0-x64.exe");
    fs::write(&named, b"MZ").unwrap();
    let out = tool(
        &[
            "manifest",
            "--version",
            "0.2.0",
            "--exe",
            path(&named),
            "--out",
            path(&manifest),
            "--days",
            "400",
        ],
        &[],
    );
    assert!(!out.status.success());
    // A secret that is not a key is refused without being printed.
    fs::write(&manifest, "{}").unwrap();
    let out = tool(
        &[
            "sign",
            "--manifest",
            path(&manifest),
            "--out",
            path(&dir.path().join("x.sig")),
            "--key-env",
            "TEST_SIGNING_KEY",
        ],
        &[("TEST_SIGNING_KEY", "hunter2-not-a-key")],
    );
    assert!(!out.status.success());
    assert!(!text(&out.stderr).contains("hunter2"));
    // No variable at all.
    let out = tool(
        &[
            "sign",
            "--manifest",
            path(&manifest),
            "--out",
            path(&dir.path().join("x.sig")),
            "--key-env",
            "NO_SUCH_SIGNING_VARIABLE_FOR_TEST",
        ],
        &[],
    );
    assert!(!out.status.success());
}

#[test]
fn the_appimage_is_an_optional_second_platform_with_its_own_name() {
    let dir = tempfile::tempdir().unwrap();
    let exe = dir.path().join("AnyPlay-Launcher-0.2.0-x64.exe");
    fs::write(&exe, b"MZ-the-new-launcher").unwrap();
    let image = dir.path().join("AnyPlay-Launcher-0.2.0-x86_64.AppImage");
    fs::write(&image, b"\x7fELF-the-new-image").unwrap();
    let manifest = dir.path().join("latest.json");
    let make = |extra: &[&str]| {
        let mut args = vec![
            "manifest",
            "--version",
            "0.2.0",
            "--exe",
            path(&exe),
            "--out",
            path(&manifest),
        ];
        args.extend_from_slice(extra);
        tool(&args, &[])
    };

    // Without --appimage the manifest is the Windows-only one it always was.
    assert!(make(&[]).status.success());
    let json = fs::read_to_string(&manifest).unwrap();
    assert!(
        json.contains("windows-x86_64") && !json.contains("linux-x86_64"),
        "{json}"
    );

    // With it, a second entry: the hash and size of that file, served beside the exe.
    let out = make(&["--appimage", path(&image)]);
    assert!(out.status.success(), "{}", text(&out.stderr));
    let json = fs::read_to_string(&manifest).unwrap();
    assert!(
        json.contains("windows-x86_64") && json.contains("linux-x86_64"),
        "{json}"
    );
    assert!(json.contains("releases/download/v0.2.0/AnyPlay-Launcher-0.2.0-x86_64.AppImage"));
    let want = {
        use sha2::{Digest, Sha256};
        anyplay_update_manifest::to_hex(&Sha256::digest(b"\x7fELF-the-new-image"))
    };
    assert!(json.contains(&want), "{json}");

    // The AppImage must carry the name the launcher stages, and exist.
    let wrong = dir.path().join("renamed.AppImage");
    fs::write(&wrong, b"x").unwrap();
    assert!(!make(&["--appimage", path(&wrong)]).status.success());
    let missing = dir
        .path()
        .join("AnyPlay-Launcher-0.2.0-x86_64.AppImage.gone");
    assert!(!make(&["--appimage", path(&missing)]).status.success());
}

#[test]
fn the_supported_version_file_writes_the_field_and_an_empty_one_leaves_it_out() {
    let dir = tempfile::tempdir().unwrap();
    let exe = dir.path().join("AnyPlay-Launcher-0.5.0-x64.exe");
    fs::write(&exe, b"MZ-the-new-launcher").unwrap();
    let file = dir.path().join("MIN_SUPPORTED_VERSION");
    let make = |content: &str| {
        fs::write(&file, content).unwrap();
        let manifest = dir.path().join("latest.json");
        let _ = fs::remove_file(&manifest);
        let out = tool(
            &[
                "manifest",
                "--version",
                "0.5.0",
                "--exe",
                path(&exe),
                "--out",
                path(&manifest),
                "--supported-file",
                path(&file),
            ],
            &[],
        );
        (out, fs::read_to_string(&manifest).unwrap_or_default())
    };
    let (out, json) = make(
        "# the oldest launcher that may go online
0.4.0
",
    );
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert!(
        json.contains("\"min_supported_version\": \"0.4.0\""),
        "{json}"
    );
    // Only comments: the mechanism is off and the manifest is what it always was.
    let (out, json) = make(
        "# nothing is retired
",
    );
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert!(!json.contains("min_supported_version"), "{json}");
    // A version above the release, and a typo, stop the release instead of passing as "off".
    for bad in [
        "0.6.0
",
        "v0.4.0
",
        "latest
",
        "0.3.0
0.4.0
",
    ] {
        let (out, json) = make(bad);
        assert!(!out.status.success(), "{bad:?}");
        assert!(json.is_empty(), "nothing is written for {bad:?}");
    }
}

#[test]
fn a_macos_archive_joins_the_same_signed_manifest_as_windows_and_linux() {
    let dir = tempfile::tempdir().unwrap();
    let exe = dir.path().join("AnyPlay-Launcher-0.3.0-x64.exe");
    let image = dir.path().join("AnyPlay-Launcher-0.3.0-x86_64.AppImage");
    let archive = dir.path().join("AnyPlay-Launcher-0.3.0-macos-arm64.zip");
    let manifest = dir.path().join("latest.json");
    fs::write(&exe, b"windows").unwrap();
    fs::write(&image, b"linux").unwrap();
    fs::write(&archive, b"macos").unwrap();
    let out = tool(
        &[
            "manifest",
            "--version",
            "0.3.0",
            "--exe",
            path(&exe),
            "--appimage",
            path(&image),
            "--macos-arm64",
            path(&archive),
            "--out",
            path(&manifest),
        ],
        &[],
    );
    assert!(out.status.success(), "{}", text(&out.stderr));
    let parsed: anyplay_update_manifest::Manifest =
        serde_json::from_slice(&fs::read(&manifest).unwrap()).unwrap();
    assert_eq!(parsed.platforms.len(), 3);
    let mac = parsed
        .artifact(anyplay_update_manifest::MACOS_ARM64)
        .unwrap();
    assert_eq!(mac.name, archive.file_name().unwrap().to_str().unwrap());
    assert_eq!(mac.size, 5);
    use sha2::{Digest, Sha256};
    assert_eq!(
        mac.sha256,
        anyplay_update_manifest::to_hex(&Sha256::digest(b"macos"))
    );
    let wrong = dir.path().join("wrong.zip");
    fs::write(&wrong, b"macos").unwrap();
    let out = tool(
        &[
            "manifest",
            "--version",
            "0.3.0",
            "--exe",
            path(&exe),
            "--macos-arm64",
            path(&wrong),
            "--out",
            path(&manifest),
        ],
        &[],
    );
    assert!(!out.status.success());
}

#[test]
fn the_kill_switch_yanks_and_lowers_the_rollout_without_a_rebuild() {
    let dir = tempfile::tempdir().unwrap();
    let secret = dir.path().join("secret.key");
    let keys = dir.path().join("update-keys.txt");
    assert!(
        tool(
            &[
                "keygen",
                "--secret-out",
                path(&secret),
                "--public-out",
                path(&keys)
            ],
            &[]
        )
        .status
        .success()
    );
    // Two releases: 0.3.8 is good, 0.3.9 is the latest and bad.
    let mut manifests = Vec::new();
    for version in ["0.3.8", "0.3.9"] {
        let exe = dir
            .path()
            .join(format!("AnyPlay-Launcher-{version}-x64.exe"));
        fs::write(&exe, format!("exe {version}")).unwrap();
        let json = dir.path().join(format!("{version}.json"));
        let sig = dir.path().join(format!("{version}.sig"));
        let made = tool(
            &[
                "manifest",
                "--version",
                version,
                "--exe",
                path(&exe),
                "--out",
                path(&json),
            ],
            &[],
        );
        assert!(made.status.success(), "{}", text(&made.stderr));
        let signed = tool(
            &[
                "sign",
                "--manifest",
                path(&json),
                "--out",
                path(&sig),
                "--key-file",
                path(&secret),
            ],
            &[],
        );
        assert!(signed.status.success(), "{}", text(&signed.stderr));
        manifests.push((json, sig));
    }
    let (live, live_sig) = &manifests[1];
    let (good, good_sig) = &manifests[0];

    // Lower the rollout of the live release: same files, new policy, then sign and verify.
    let next = dir.path().join("next.json");
    let next_sig = dir.path().join("next.sig");
    let out = tool(
        &[
            "reissue",
            "--from",
            path(live),
            "--sig",
            path(live_sig),
            "--keys",
            path(&keys),
            "--rollout",
            "10",
            "--out",
            path(&next),
        ],
        &[],
    );
    assert!(out.status.success(), "{}", text(&out.stderr));
    let value: serde_json::Value = serde_json::from_slice(&fs::read(&next).unwrap()).unwrap();
    assert_eq!(value["version"], "0.3.9");
    assert_eq!(value["rollout_percent"], 10);
    assert!(value.get("yanked").is_none());
    let signed = tool(
        &[
            "sign",
            "--manifest",
            path(&next),
            "--out",
            path(&next_sig),
            "--key-file",
            path(&secret),
        ],
        &[],
    );
    assert!(signed.status.success(), "{}", text(&signed.stderr));
    let verified = tool(
        &[
            "verify",
            "--manifest",
            path(&next),
            "--sig",
            path(&next_sig),
            "--keys",
            path(&keys),
        ],
        &[],
    );
    assert!(verified.status.success(), "{}", text(&verified.stderr));

    // Walk back: offer 0.3.8 on the latest release and yank 0.3.9. A launcher on 0.3.9 takes it.
    let back = dir.path().join("back.json");
    let back_sig = dir.path().join("back.sig");
    let out = tool(
        &[
            "reissue",
            "--from",
            path(&next),
            "--sig",
            path(&next_sig),
            "--keys",
            path(&keys),
            "--offer-from",
            path(good),
            "--offer-sig",
            path(good_sig),
            "--yank",
            "0.3.9",
            "--out",
            path(&back),
        ],
        &[],
    );
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert!(
        tool(
            &[
                "sign",
                "--manifest",
                path(&back),
                "--out",
                path(&back_sig),
                "--key-file",
                path(&secret)
            ],
            &[],
        )
        .status
        .success()
    );
    let ok = tool(
        &[
            "verify",
            "--manifest",
            path(&back),
            "--sig",
            path(&back_sig),
            "--keys",
            path(&keys),
            "--current",
            "0.3.9",
        ],
        &[],
    );
    assert!(ok.status.success(), "{}", text(&ok.stderr));
    // Whoever runs 0.3.10 (not yanked) gets nothing.
    let no = tool(
        &[
            "verify",
            "--manifest",
            path(&back),
            "--sig",
            path(&back_sig),
            "--keys",
            path(&keys),
            "--current",
            "0.3.10",
        ],
        &[],
    );
    assert!(!no.status.success());

    // Refusals: yanking the version on offer, a bad percentage, a manifest with a wrong signature.
    for args in [
        vec!["--yank", "0.3.9"],
        vec!["--rollout", "101"],
        vec!["--rollout", "ten"],
        vec!["--yank", "v0.3.9x"],
    ] {
        let mut full = vec![
            "reissue",
            "--from",
            path(live),
            "--sig",
            path(live_sig),
            "--keys",
            path(&keys),
            "--out",
            path(&next),
        ];
        full.extend(args.iter().copied());
        assert!(!tool(&full, &[]).status.success(), "{args:?}");
    }
    let wrong = tool(
        &[
            "reissue",
            "--from",
            path(live),
            "--sig",
            path(good_sig),
            "--keys",
            path(&keys),
            "--out",
            path(&next),
        ],
        &[],
    );
    assert!(
        !wrong.status.success(),
        "a manifest that does not match its signature"
    );
}
