use std::{io, net::IpAddr, string::FromUtf8Error};

use thiserror::Error;
use tokio::process::Command;

pub async fn lookup_mac(ip_addr: &IpAddr) -> Result<(String, String), ArpError> {
    if ip_addr.is_loopback() {
        return Ok(("localhost".to_string(), "00:00:00:00:00:00".to_string()));
    }

    let ip_str = format!("({})", ip_addr);

    let output = Command::new("/usr/sbin/arp").arg("-a").output().await?;
    let output_str = String::from_utf8(output.stdout)?;

    //Arp has super easy to parse output, let's just do it the easy way
    let (hostname, mac) = output_str
        .lines()
        .find_map(|x| {
            let cols = x.split_whitespace().collect::<Vec<&str>>();
            if cols.len() > 4 && cols.get(1) == Some(&ip_str.as_str()) {
                Some((
                    cols.first().unwrap().to_string(),
                    cols.get(3).unwrap().to_string(),
                ))
            } else {
                None
            }
        })
        .ok_or(ArpError::NotFound(ip_str))?;

    Ok((hostname, mac))
}

#[derive(Debug, Error)]
pub enum ArpError {
    #[error("Unknown host in arp call {0}")]
    NotFound(String),
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_router() -> Result<(), Box<dyn std::error::Error>> {
        let (hostname, mac) = lookup_mac(&IpAddr::from([10u8, 0u8, 1u8, 1u8])).await?;
        assert_eq!(hostname, "?");
        assert_eq!(mac, "0:11:32:77:85:7a");
        Ok(())
    }
}
