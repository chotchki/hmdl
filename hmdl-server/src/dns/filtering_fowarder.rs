use sqlx::SqlitePool;
use trust_dns_server::authority::{
    Authority, LookupError, LookupOptions, MessageRequest, UpdateResult, ZoneType,
};
use trust_dns_server::client::op::ResponseCode;
use trust_dns_server::client::rr::{LowerName, RecordType};
use trust_dns_server::resolver::config::{NameServerConfigGroup, ResolverOpts};
use trust_dns_server::resolver::Name;
use trust_dns_server::server::RequestInfo;
use trust_dns_server::store::forwarder::{ForwardAuthority, ForwardConfig, ForwardLookup};

use super::{should_filter, Decision};

pub struct FilteringForwarder {
    fwd_authority: ForwardAuthority,
    pool: SqlitePool,
}

impl FilteringForwarder {
    pub async fn create(pool: SqlitePool) -> FilteringForwarder {
        let fa_config = ForwardConfig {
            name_servers: NameServerConfigGroup::google(),
            options: Some(ResolverOpts::default()),
        };
        let fwd_authority =
            ForwardAuthority::try_from_config(Name::root(), ZoneType::Forward, &fa_config)
                .await
                .unwrap();

        FilteringForwarder {
            fwd_authority,
            pool,
        }
    }

    /*  pub async fn log_client(&mut self, src: SocketAddr) -> Result<(), sqlx::Error> {

    }*/
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
        match should_filter(
            self.pool.clone(),
            &request_info.src.ip(),
            request_info.query.name(),
        )
        .await
        {
            Decision::Block => return Err(LookupError::ResponseCode(ResponseCode::Unknown(3841))),
            Decision::Allow => {
                return Ok(self
                    .fwd_authority
                    .search(request_info, lookup_options)
                    .await?)
            }
        };
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
