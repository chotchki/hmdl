use std::net::Ipv4Addr;

use trust_dns_server::authority::{
    Authority, LookupError, LookupOptions, MessageRequest, UpdateResult, ZoneType,
};
use trust_dns_server::client::rr::{LowerName, RData, RecordType};
use trust_dns_server::resolver::config::{NameServerConfigGroup, ResolverOpts};
use trust_dns_server::resolver::lookup::Lookup;
use trust_dns_server::resolver::Name;
use trust_dns_server::server::RequestInfo;
use trust_dns_server::store::forwarder::{ForwardAuthority, ForwardConfig, ForwardLookup};

pub struct FilteringForwarder {
    fwd_authority: ForwardAuthority,
}

impl FilteringForwarder {
    pub async fn create() -> Self {
        let fa_config = ForwardConfig {
            name_servers: NameServerConfigGroup::google(),
            options: Some(ResolverOpts::default()),
        };
        let fwd_authority =
            ForwardAuthority::try_from_config(Name::root(), ZoneType::Forward, &fa_config)
                .await
                .unwrap();

        Self { fwd_authority }
    }
}

#[async_trait::async_trait]
impl Authority for FilteringForwarder {
    type Lookup = ForwardLookup;

    fn zone_type(&self) -> trust_dns_server::authority::ZoneType {
        self.fwd_authority.zone_type()
    }

    fn is_axfr_allowed(&self) -> bool {
        self.fwd_authority.is_axfr_allowed()
    }

    async fn update(&self, _update: &MessageRequest) -> UpdateResult<bool> {
        self.fwd_authority.update(_update).await
    }

    fn origin(&self) -> &LowerName {
        self.fwd_authority.origin()
    }

    async fn lookup(
        &self,
        name: &LowerName,
        rtype: RecordType,
        lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        let result = self
            .fwd_authority
            .lookup(name, rtype, lookup_options)
            .await?;

        Ok(result)
    }

    async fn search(
        &self,
        request_info: RequestInfo<'_>,
        lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        if request_info.src.ip() == Ipv4Addr::new(127, 0, 0, 1) {
            return Ok(ForwardLookup(Lookup::from_rdata(
                request_info.query.original().clone(),
                RData::A(Ipv4Addr::new(127, 0, 0, 1)),
            )));
        }
        self.fwd_authority
            .search(request_info, lookup_options)
            .await
    }

    async fn get_nsec_records(
        &self,
        _name: &LowerName,
        _lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        self.fwd_authority
            .get_nsec_records(_name, _lookup_options)
            .await
    }
}
