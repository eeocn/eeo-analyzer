use trust_dns_resolver::Resolver;

fn main() -> anyhow::Result<()> {
    // 获取当前的DNS配置
    let (config, options) = trust_dns_resolver::system_conf::read_system_conf()?;
    // config.add_name_server(NameServerConfig {
    //     socket_addr: "8.8.8.8:53".parse()?,
    //     protocol: Protocol::Tcp,
    //     tls_dns_name: None,
    //     trust_nx_responses: false,
    //     bind_addr: None,
    // });
    // config.add_name_server(NameServerConfig {
    //     socket_addr: "8.8.8.8:53".parse()?,
    //     protocol: Protocol::Udp,
    //     tls_dns_name: None,
    //     trust_nx_responses: false,
    //     bind_addr: None,
    // });
    // 获取所有的nameserver
    let nameservers: Vec<String> = config
        .name_servers()
        .iter()
        .map(|nameserver| nameserver.to_string())
        .collect();

    // 打印所有的nameserver
    for nameserver in nameservers {
        println!("{}", nameserver);
    }

    let resolver = Resolver::new(config, options)?;
    let r = resolver.lookup_ip("www.baidu.com");
    // 打印解析记录
    println!("{:?}", r);
    Ok(())
}
