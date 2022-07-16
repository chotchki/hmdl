pub async fn next_ip(transaction: &Transaction) -> Result<IpAddr, UnusedIpError>{

}

#[derive(Debug, Error)]
pub enum UnusedIpError {
    #[error("Address Space Exhausted")]
    SpaceExhuasted(),
}
