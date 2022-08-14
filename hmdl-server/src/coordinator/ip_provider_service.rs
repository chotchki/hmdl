use std::{
    collections::HashSet,
    net::{IpAddr, Ipv6Addr},
    time::Duration,
};

use local_ip_address::list_afinet_netifas;
use thiserror::Error;
use tokio::{
    sync::broadcast::Sender,
    time::{interval, MissedTickBehavior},
};

pub struct IpProvderService {}

impl IpProvderService {
    //This creation function simply saves the sender since we have no other dependancies
    pub fn create() -> Self {
        Self {}
    }

    /// This schedules a task to periodically wake up and see
    /// if the IP addresses for the machine have changed, if so
    /// they are broadcoast
    ///
    /// TODO: Switch to a model that asks the underlying operating system
    pub async fn start(
        &self,
        ip_changed: Sender<HashSet<IpAddr>>,
    ) -> Result<(), IpProvderServiceError> {
        let mut duration = interval(Duration::from_millis(60 * 1000));
        duration.set_missed_tick_behavior(MissedTickBehavior::Skip);
        let mut current_ips = Self::server_ips()?;

        //Always send the starting IPs
        tracing::debug!("Sending Initial IP addresses");
        ip_changed.send(current_ips.clone()).ok();

        loop {
            duration.tick().await;

            let new_ips = Self::server_ips()?;
            if current_ips != new_ips {
                current_ips = new_ips;
                tracing::info!("IP addresses changed, broadcasting");
                ip_changed.send(current_ips.clone()).ok();
            }
        }
    }

    /// This function is to figure out what ip addresses should be used to serve HMDL
    fn server_ips() -> Result<HashSet<IpAddr>, IpProvderServiceError> {
        let addrs = list_afinet_netifas()?;

        let mut filtered_addrs = HashSet::new();

        for (_, addr) in addrs {
            if let IpAddr::V4(addrv4) = addr {
                if !addrv4.is_link_local() {
                    filtered_addrs.insert(IpAddr::V4(addrv4));
                }
            } else if let IpAddr::V6(addrv6) = addr {
                if !Self::has_unicast_link_local_scope(addrv6) {
                    filtered_addrs.insert(IpAddr::V6(addrv6));
                }
            }
        }

        Ok(filtered_addrs)
    }

    // This fn is to work around feature "ip" not being stable yet
    pub const fn has_unicast_link_local_scope(addr: Ipv6Addr) -> bool {
        (addr.segments()[0] & 0xffc0) == 0xfe80
    }
}

#[derive(Debug, Error)]
pub enum IpProvderServiceError {
    #[error(transparent)]
    LocalIpError(#[from] local_ip_address::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_ips() -> Result<(), Box<dyn std::error::Error>> {
        let ips = IpProvderService::server_ips()?;
        assert!(!ips.is_empty());
        Ok(())
    }
}
