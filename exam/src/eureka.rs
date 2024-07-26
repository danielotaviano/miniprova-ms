use eureka_client::{BaseConfig, EurekaClient, PortData};
use local_ip_address::local_ip;

pub fn init_eureka(
    server_host: String,
    server_port: u16,
    hostname: String,
    instance_port: u16,
) -> EurekaClient {
    let mut config = BaseConfig::default();
    config.eureka.host = server_host;
    config.eureka.port = server_port;
    config.instance.ip_addr = local_ip().unwrap().to_string();
    config.instance.port = Some(PortData::new(instance_port, true));
    config.instance.app = "exam".to_string();
    config.instance.host_name = hostname;
    let eureka = EurekaClient::new(config);
    eureka.start();
    eureka
}
