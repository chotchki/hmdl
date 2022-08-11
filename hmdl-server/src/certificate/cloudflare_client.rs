use std::{
    collections::{HashMap, HashSet},
    net::IpAddr,
    str::FromStr,
};

use cloudflare::{
    endpoints::{
        dns::{
            CreateDnsRecord, CreateDnsRecordParams, DeleteDnsRecord, DnsContent, DnsRecord,
            ListDnsRecords, ListDnsRecordsParams,
        },
        zone::{ListZones, ListZonesParams, Status},
    },
    framework::{
        apiclient::ApiClient, auth::Credentials, response::ApiFailure, Environment, HttpApiClient,
        HttpApiClientConfig, SearchMatch,
    },
};
use thiserror::Error;
use trust_dns_server::{client::rr::LowerName, proto::error::ProtoError, resolver::Name};

pub struct CloudflareClient {
    client: HttpApiClient,
    zone_id: String,
    domain: LowerName,
}

impl CloudflareClient {
    pub fn create(api_token: String, domain_name: &str) -> Result<Self, CloudflareClientError> {
        let client = Self::create_client(api_token)?;
        let domain = LowerName::from(Name::from_str(domain_name)?);
        let zone_id = Self::get_zone_id(&client, domain.clone())?;

        Ok(Self {
            client,
            zone_id,
            domain,
        })
    }

    pub fn update_dns(&self, addrs: HashSet<IpAddr>) -> Result<(), CloudflareClientError> {
        let dns_recs = self.get_recs_by_name(self.domain.to_string())?;
        let mut dns_ip_to_id: HashMap<IpAddr, String> = dns_recs
            .iter()
            .filter_map(|x| match x.content {
                DnsContent::A { content: y } => Some((IpAddr::V4(y), x.id.clone())),
                DnsContent::AAAA { content: y } => Some((IpAddr::V6(y), x.id.clone())),
                _ => None,
            })
            .collect();

        //Now we need to figure out, the sets of actions to take
        let mut missing_recs = addrs.clone();
        missing_recs.retain(|x| !dns_ip_to_id.contains_key(x));

        dns_ip_to_id.retain(|k, _| !addrs.contains(k));

        //Now we create and delete records
        for rec in missing_recs {
            self.create_record(self.domain.to_string(), rec)?;
        }
        for id in dns_ip_to_id.values() {
            self.delete_record(id)?;
        }

        Ok(())
    }

    pub fn create_proof(&self, proof_value: String) -> Result<(), CloudflareClientError> {
        let name = "_acme-challenge.".to_string() + &self.domain.to_string() + ".";

        self.client.request(&CreateDnsRecord {
            zone_identifier: &self.zone_id,
            params: CreateDnsRecordParams {
                ttl: Some(1),
                name: &name,
                priority: None,
                proxied: Some(false),
                content: DnsContent::TXT {
                    content: proof_value,
                },
            },
        })?;

        Ok(())
    }

    fn create_client(token: String) -> Result<HttpApiClient, CloudflareClientError> {
        Ok(HttpApiClient::new(
            Credentials::UserAuthToken { token },
            HttpApiClientConfig::default(),
            Environment::Production,
        )?)
    }

    fn get_zone_id(
        client: &HttpApiClient,
        domain: LowerName,
    ) -> Result<String, CloudflareClientError> {
        let parent = domain.base_name().to_string();

        let zone_result = client.request(&ListZones {
            params: ListZonesParams {
                name: Some(parent.to_string()),
                status: Some(Status::Active),
                page: Some(1),
                per_page: Some(5),
                order: None,
                direction: None,
                search_match: Some(SearchMatch::All),
            },
        })?;

        Ok(zone_result
            .result
            .get(0)
            .ok_or_else(|| CloudflareClientError::CouldNotFindZone(parent.clone()))?
            .id
            .clone())
    }

    /// I'm making an assumption that I'll never have more than 100 addresses for HMDL
    pub fn get_recs_by_name(&self, name: String) -> Result<Vec<DnsRecord>, CloudflareClientError> {
        let dns_results = self.client.request(&ListDnsRecords {
            zone_identifier: &self.zone_id,
            params: ListDnsRecordsParams {
                record_type: None,
                name: Some(name),
                page: Some(1),
                per_page: Some(100),
                order: None,
                direction: None,
                search_match: Some(SearchMatch::All),
            },
        })?;

        Ok(dns_results.result)
    }

    pub fn create_record(&self, name: String, addr: IpAddr) -> Result<(), CloudflareClientError> {
        let content = match addr {
            IpAddr::V4(v4) => DnsContent::A { content: v4 },
            IpAddr::V6(v6) => DnsContent::AAAA { content: v6 },
        };

        self.client.request(&CreateDnsRecord {
            zone_identifier: &self.zone_id,
            params: CreateDnsRecordParams {
                ttl: Some(1),
                priority: None,
                proxied: Some(false),
                name: &name,
                content,
            },
        })?;

        Ok(())
    }

    pub fn delete_record(&self, dns_id: &String) -> Result<(), CloudflareClientError> {
        self.client.request(&DeleteDnsRecord {
            zone_identifier: &self.zone_id,
            identifier: dns_id,
        })?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CloudflareClientError {
    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),

    #[error(transparent)]
    ApiFailureError(#[from] ApiFailure),

    #[error("Could not locate zone {0}")]
    CouldNotFindZone(String),

    #[error(transparent)]
    LocalIPError(#[from] local_ip_address::Error),

    #[error(transparent)]
    ProtoError(#[from] ProtoError),
}

#[cfg(test)]
mod tests {

    use super::*;

    // Test function for the cloudflare code
    //#[test]
    fn _test_example() -> Result<(), Box<dyn std::error::Error>> {
        let client = CloudflareClient::create("".to_string(), "")?;

        let mut addrs = HashSet::new();
        addrs.insert(IpAddr::V6(std::net::Ipv6Addr::from_str("::1")?));

        client.update_dns(addrs)?;

        Ok(())
    }
}
