# Maintenance

## Background

Maintained fork: `0xble/miniserve` of `svenstaro/miniserve`; maintained and
upstream branch `master`. This temporary isolated checkout is remote-only, not
a runtime canonical checkout. Accepted baseline:
`35b34057ad0d30388f25e15bd6f836122ab93fae`. Publish only to `origin`; never
push upstream.

## Preserve

- Native Tailscale binding and its clickable-IPv4 presentation remain intact.
- Fork branding and macOS firewall source wiring are not installation evidence;
installation and runtime activation remain separately authorized.

## Active patches

### MINISERVE-001: `feat: add native tailscale binding mode`

- **Status:** Active; `c26b5782`, `ae36f0d4`.
- **Behavior:** native Tailscale mode binds correctly and presents only the clickable IPv4 URL.
- **Surfaces:** `src/args.rs`, `src/config.rs`, `src/main.rs`, `src/tailscale.rs`, `tests/cli.rs`, README.
- **Upstream issue:** None after checked 2026-09-09.
- **Upstream PR:** None after checked 2026-09-09.
- **Regression:** `cargo test --locked`; focused CLI Tailscale cases pass.
- **Rollback:** revert both commits together and rerun the focused CLI proof.
- **Retire when:** released upstream behavior is equivalent and its tests pass after reconciliation.

### MINISERVE-002: `chore: apply fork version suffix (0.33.0-0xble.1)`

- **Status:** Active; `9dafdfed`, `f60860af`.
- **Behavior:** fork builds retain their version suffix.
- **Surfaces:** `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`.
- **Upstream issue:** None after checked 2026-09-09.
- **Upstream PR:** None after checked 2026-09-09.
- **Regression:** `cargo build --release --locked` succeeds and reports fork version metadata.
- **Rollback:** revert the version commits after deciding the replacement version.
- **Retire when:** an authorized unbranded distribution replaces the fork build.

### MINISERVE-003: `chore: add post-install firewall allow script for macOS`

- **Status:** Active; `5367f1ae`.
- **Behavior:** macOS firewall allowance remains source wiring only.
- **Surfaces:** `scripts/post-install.sh`.
- **Upstream issue:** None after checked 2026-09-09.
- **Upstream PR:** None after checked 2026-09-09.
- **Regression:** `bash -n scripts/post-install.sh`; no firewall action is run.
- **Rollback:** revert `5367f1ae`; do not change live firewall state.
- **Retire when:** installer policy is explicitly retired or replaced and separately verified.

## Update

Every maintenance run fetches `origin` and latest `upstream/master`, reconciles
`master`, preserves only these active records, and runs declared regression, test,
and build proof before authorized publication. Immediately before `Updated` or
`Already current`, fetch upstream again and prove no upstream-only commits;
otherwise report `Blocked` with stage, refs, and evidence. Update this contract
with each patch addition, change, or retirement; missing coverage blocks
publication.

## Verify

```text
cargo fmt --check
cargo test --locked
cargo build --release --locked
bash -n scripts/post-install.sh
git rev-list --left-right --count upstream/master...master
```

Require a fresh final fetch with zero upstream-only commits and local/`origin`
SHA parity after authorized publication. Installation, firewall modification, and
runtime SHA proof require a separate authorized stage.