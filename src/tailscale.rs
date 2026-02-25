use std::io::ErrorKind;
use std::net::IpAddr;
use std::process::Command;

use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;

#[derive(Debug)]
pub struct TailscaleInfo {
    pub ips: Vec<IpAddr>,
    pub dns_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TailscaleStatus {
    #[serde(rename = "Self")]
    self_node: Option<TailscaleNode>,
}

#[derive(Debug, Deserialize)]
struct TailscaleNode {
    #[serde(rename = "TailscaleIPs", default)]
    tailscale_ips: Vec<IpAddr>,
    #[serde(rename = "DNSName")]
    dns_name: Option<String>,
}

pub fn resolve_tailscale_info() -> Result<TailscaleInfo> {
    let output = match Command::new("tailscale")
        .args(["status", "--json"])
        .output()
    {
        Ok(output) => output,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            bail!(
                "Could not find the `tailscale` binary in PATH. Install Tailscale or run miniserve without --tailscale."
            );
        }
        Err(err) => {
            return Err(err).context("Failed to execute `tailscale status --json`");
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let details = if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            "tailscale returned a non-zero exit code".to_owned()
        };
        bail!("`tailscale status --json` failed: {details}");
    }

    parse_tailscale_status_json(&output.stdout)
}

fn parse_tailscale_status_json(raw_json: &[u8]) -> Result<TailscaleInfo> {
    let status: TailscaleStatus = serde_json::from_slice(raw_json)
        .context("Failed to parse `tailscale status --json` output")?;

    let self_node = status.self_node.ok_or_else(|| {
        anyhow!("`tailscale status --json` output did not include `Self` node information")
    })?;

    let mut ips = self_node.tailscale_ips;
    if ips.is_empty() {
        bail!("No Tailscale IPs found for this machine. Verify that Tailscale is connected.");
    }

    ips.sort();
    ips.dedup();

    let dns_name = self_node
        .dns_name
        .map(|name| name.trim_end_matches('.').to_owned())
        .filter(|name| !name.is_empty());

    Ok(TailscaleInfo { ips, dns_name })
}

#[cfg(test)]
mod tests {
    use super::parse_tailscale_status_json;

    #[test]
    fn parse_tailscale_status_json_extracts_ips_and_dns_name() {
        let payload = br#"{
            "Self": {
                "DNSName": "host-name.tailnet.ts.net.",
                "TailscaleIPs": ["100.101.102.103", "fd7a:115c:a1e0::1234"]
            }
        }"#;

        let parsed =
            parse_tailscale_status_json(payload).expect("expected valid tailscale status json");

        assert_eq!(parsed.ips.len(), 2);
        assert_eq!(parsed.ips[0].to_string(), "100.101.102.103");
        assert_eq!(parsed.ips[1].to_string(), "fd7a:115c:a1e0::1234");
        assert_eq!(parsed.dns_name.as_deref(), Some("host-name.tailnet.ts.net"));
    }

    #[test]
    fn parse_tailscale_status_json_requires_ips() {
        let payload = br#"{"Self":{"DNSName":"host.tailnet.ts.net.","TailscaleIPs":[]}}"#;
        let err = parse_tailscale_status_json(payload)
            .expect_err("expected missing tailscale ips to fail");
        assert!(err.to_string().contains("No Tailscale IPs found"));
    }

    #[test]
    fn parse_tailscale_status_json_requires_self_node() {
        let payload = br#"{"BackendState":"Running"}"#;
        let err =
            parse_tailscale_status_json(payload).expect_err("expected missing self node to fail");
        assert!(
            err.to_string()
                .contains("did not include `Self` node information")
        );
    }
}
