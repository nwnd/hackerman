use std::{net::IpAddr, str::FromStr};

use super::CommandHandlerError;

pub async fn lookup_dns(domain: String) -> Result<String, CommandHandlerError> {
    // look up the domain
    match dns_lookup::lookup_host(&domain) {
        Ok(ips) => Ok(format!(
            "`{}` resolves to: ```\n{}```",
            domain,
            ips.iter()
                .map(|ip| ip.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )),
        Err(_) => Err(CommandHandlerError::Ignore),
    }
}

pub async fn lookup_dns_reverse(ip_addr: String) -> Result<String, CommandHandlerError> {
    // look up the address
    match dns_lookup::lookup_addr(&IpAddr::from_str(&ip_addr).unwrap()) {
        Ok(domain) => Ok(format!("`{}` resolves to: `{}`", ip_addr, domain)),
        Err(_) => Err(CommandHandlerError::Ignore),
    }
}
