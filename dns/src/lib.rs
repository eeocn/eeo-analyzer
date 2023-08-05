use std::net::SocketAddr;

use anyhow::Context;
use hyper::client::connect::dns::Name;
use reqwest::dns::{Addrs, Resolve, Resolving};
use trust_dns_resolver::{lookup_ip::LookupIpIntoIter, TokioAsyncResolver};

// 自定义 DNS 解析器
pub struct CustomResolver;

struct SocketAddrs {
    iter: LookupIpIntoIter,
}

impl Iterator for SocketAddrs {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ip_addr| SocketAddr::new(ip_addr, 0))
    }
}

impl Resolve for CustomResolver {
    fn resolve(&self, name: Name) -> Resolving {
        log::info!("[DNSLookup] domain: {}", name);
        Box::pin(async move {
            let (config, options) = trust_dns_resolver::system_conf::read_system_conf()
                .context("[DNSLookup] read system dns config fail")?;
            let nameservers: Vec<String> = config
                .name_servers()
                .iter()
                .map(|nameserver| nameserver.to_string())
                .collect();

            for nameserver in nameservers {
                log::info!("[DNSLookup] nameserver: {}", nameserver);
            }
            let resolver = TokioAsyncResolver::tokio(config, options)
                .context("[DNSLookup] create resolver fail")?;

            let response = resolver
                .lookup_ip(name.to_string())
                .await
                .context("[DNSLookup] look ip fail")?;

            let look_ip = response.clone();
            let lookup = look_ip.as_lookup();
            log::info!(
                "[DNSLookup] query name:{}, class:{}, type:{}",
                lookup.query().name(),
                lookup.query().query_class(),
                lookup.query().query_type(),
            );

            for r in lookup.records() {
                log::info!(
                    "[DNSLookup] name:{}, class:{}, type:{}, ttl:{}, data:{:?}",
                    r.name(),
                    r.dns_class(),
                    r.rr_type(),
                    r.ttl(),
                    r.data()
                )
            }

            let addrs: Addrs = Box::new(SocketAddrs {
                iter: response.into_iter(),
            });
            Ok(addrs)
        })
    }
}
