# Signing needs a tag, not an approval

The job `sign` in `release.yml` used to wait for a required reviewer on the Environment `release-signing`. The wait cost 7 seconds in the v0.3.0 run and 20 minutes on 25 October 2026, and it protected nothing the tag does not already protect:

- The reviewer is the operator or the operator's agent, who can approve through the API. The same people can push the `v*` tag that starts the run, so the approval is a second click by the same hands, not a second key holder.
- The approval shows no content. The reviewer sees that a job wants the environment, not what it signs.
- The signing job builds nothing. It runs `update-sign`, which an earlier job without the secret built, over the artifacts of the same run, and the `gate` job has already checked that `ci` on `main` passed for the tagged commit.

So the control that matters is who can start a release and which code can reach the key. Both stay, as rules instead of a person:

- The Environment `release-signing` is limited to the tag rule `v*`. A run from a branch, a pull request or by hand never gets the secret.
- The tag ruleset "Release tags are made by admins only" lets only organization admins create a `v*` tag, and blocks updating (moving) and deleting it for everyone, admins included unless they bypass on purpose.
- `main` stays behind a ruleset that requires a reviewed pull request, so the workflow that holds the key cannot change without review.

The required reviewers (AlxcBTW and TideOPL) are removed from the Environment.

## Consequences

- A release takes the time the builds take. Nobody has to find the approval button, and a release started at night does not stop at `sign`.
- Whoever can push a `v*` tag can make the signing job run. Today these are the three organization admins (AlxcBTW, parl0v, TideOPL). GitHub cannot restrict a ruleset bypass to one named person, only to roles, teams and apps; a team that holds only the operator is the way to narrow it later.
- A compromised admin account could always have approved its own run, so the residual risk does not grow. A leaked key is handled by the rotation steps in [UPDATES.md](../UPDATES.md#rotate-the-key).
- The settings live on GitHub, not in the repository. [UPDATES.md](../UPDATES.md#protecting-the-key) says how to set them up again, and the pull request that added this decision records the exact change so it can be undone.

## Considered options

- **Keep the reviewer:** the same people approve their own tag, with a delay of seconds to a day, and no extra protection.
- **Prevent self-review and name a second reviewer:** this would protect something, but the project has no maintainer who is always around to approve at night, so releases would wait on a person again.
- **A signing key in a cloud key service bound to the workflow by OIDC:** stronger, because the secret would not exist in GitHub at all. Whether the services we could use support Ed25519 is not researched, and many do not. A later step.
