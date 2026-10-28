# The signed manifest carries the kill switch

Until now a bad release could only be fixed by shipping another one: build, test, tag, wait. The report on faster builds and patches (step 10) asks for the other half of a fast hotfix: take a bad version out of circulation within minutes, and release a good one to few players first. Every launcher already reads `latest.json` of the latest release and trusts it through its Ed25519 signature, so the manifest is the place for it.

## Decision

Two optional fields join the signed manifest, and a third control that already existed is named for what it does here:

- **`yanked`**: versions that must no longer be offered. The manifest cannot offer a version it yanks (`validate` refuses it, so the signing tool refuses to sign it). A launcher that **runs** a yanked version is offered the manifest's version even if that is older, and is never held back by a rollout. This is the one downgrade a manifest can grant, and only away from a version the signed manifest names.
- **`rollout_percent`** (0 to 100, absent means 100): the share of installs that see the update yet. An install's bucket is `SHA-256("anyplay-launcher-rollout-v1\n" + install_id + 0x00 + version)`, first four bytes, modulo 100; it sees the update when its bucket is below the share. Raising the share only adds installs. The version is in the hash, so the same installs are not always first. The install id is 128 random bits made once and kept in `install-id` in the data folder; it is tied to nothing and never sent. No id (the file cannot be written) means no hold: fail open, because an update is worth more than a delay.
- **`min_supported_version`** is the "min version" of the request and already existed (see DESIGN.md 9.5.1 and UPDATES.md): a launcher below it is told an update is required and keeps offline play. It is unchanged. A required update ignores `rollout_percent`.

The control is an operation, not a build: `update-sign reissue` takes the manifest that is live, changes only these fields and the validity window, and the `resign` workflow (started by hand) signs it in the existing environment `release-resigning` and replaces the two assets. The policy is carried by the signed manifest itself: the weekly renewal also starts from the live manifest, so a yank or a rollout survives it. A new release starts with a clean policy (rollout 100, nothing yanked), because it is a new decision; a yank of an older version is applied again with one run.

## Compatibility

Everything is optional and ignored by readers that do not know it:

- **Old launchers, new manifests.** The fields are unknown keys; the old reader has no `deny_unknown_fields` (a test parses a new manifest with a struct of the old shape). An old launcher therefore ignores a rollout (it sees the update at once) and a yank (it never walks back). The walk back is offered by a manifest whose version is older than what an old launcher runs, which an old launcher reads as "nothing newer", so it stays where it is. Old launchers are never harmed, only not protected.
- **New launchers, old manifests.** `serde(default)` on both fields; a test reads a manifest without them and gets no yank and 100 percent.
- **Signature.** The fields are inside the signed bytes. A flipped `rollout_percent` fails verification (tested), and a manifest whose signature fails is ignored as before.

## Consequences

- A rollout is a hold of the **update notice**, not of the download link: someone who finds the release page can still install it by hand. That is acceptable for a staged release; a yank is the control for a bad one.
- The walk back is a rollback primitive, so the replay window matters: a genuine older manifest that yanks a version, replayed until it expires (at most 30 days), can move a launcher on that version back to the older release. It can never reach a version the operator did not yank in that manifest, and the older release is a genuine, signed one.
- A staged download that was fetched before a yank is not removed by the yank. It is re-verified with the manifest stored beside it, which predates the yank. The next check offers the manifest's version instead; closing this gap needs the launcher to drop a staged file whose version the live manifest yanks, which is a follow-up.
- Players can look at the rollout: the manifest is public. It is a hold for convenience and safety, not a secret.

## Considered options

- **A `channel` field (`stable` / `beta`).** Not added. The launcher reads one manifest, `latest.json` of the latest GitHub release, and a second channel needs a second manifest (a `latest-beta.json` on a pre-release, a preference in the UI, its own expiry and renewal). A `channel` field inside one manifest would claim a distinction nothing could act on. The schema leaves room: a client that ever reads `latest-beta.json` needs no change to this format.
- **Rolling out by a counter on a server.** Needs a service the launcher would have to call, and breaks the property that GitHub is the only thing an update needs. The hash of a local random id gives the same staging with no state and no personal data.
- **A new field `min_version` next to `min_supported_version`.** Two names for one thing. Launchers up to v0.2.x also read `minimum_version` as "may not update from here", which is why `min_supported_version` has its own name (see the field's comment in `anyplay-update-manifest`).
- **Putting the policy in a repository file, like `MIN_SUPPORTED_VERSION`.** A change would need a reviewed pull request to `main`, which is the opposite of a kill switch. `MIN_SUPPORTED_VERSION` stays a file because retiring versions is a planned decision.
