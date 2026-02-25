#!/usr/bin/env bash
# Post-install hook for miniserve fork: re-allow in macOS application firewall.
# Called after cargo install or binary copy.
set -euo pipefail

BINARY="${1:-$HOME/.local/share/cargo/bin/miniserve}"

if [[ ! -f "$BINARY" ]]; then
  echo "miniserve binary not found at $BINARY" >&2
  exit 1
fi

# Remove stale entry (old hash), then re-add + unblock
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --remove "$BINARY" 2>/dev/null || true
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add "$BINARY" 2>/dev/null
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --unblockapp "$BINARY" 2>/dev/null

echo "firewall: allowed $BINARY"
