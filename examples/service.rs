use nacos_rs_sdk::model::instance::{
    Instance, InstanceObject, QueryInstances, RegisterInstanceOption,
};
use nacos_rs_sdk::NacosClient;
use nacos_rs_sdk::NacosConfig;
use std::error::Error;
// use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpListener;

// use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = test_nacos_config();
    println!(" -- > {:?}", config);
    let client = config.connect_with_auth().await.unwrap();
    println!(" -- > {:?}", &client);
    let namespace_id = "18130d3d-598c-4794-af27-aa4c8fbfc6e4".to_string();
    // let option = InstanceOptions {
    //     cluster_name:None, group_name:None, namespace_id, ephemeral: true, weight:1.0, enabled:true, healthy:true, metadata:None,
    // };
    let instance = InstanceObject::builder()
        .service_name("test".to_string())
        .ip("10.188.18.18".to_string())
        .port(8080)
        .build()
        .unwrap();
    let options = RegisterInstanceOption::builder()
        .cluster_name("cta".to_string())
        .group_name("cta".to_string())
        .namespace_id(namespace_id.clone())
        .build()
        .unwrap();
    // println!(" -- > {:?}", instance);
    // println!(" -- > Register {:?}", &client);
    // println!(" -- > Register {:?}",
    let r = client
        .register_with_object(&instance, &Some(options.clone()))
        .await;
    println!(" -- > {:?}", r);
    time::sleep(Duration::from_millis(1000)).await;
    instance.hart(&client, &Some(options.clone())).await;
    time::sleep(Duration::from_millis(5000)).await;
    let ql = QueryInstances::builder()
        .service_name("cta".to_string())
        .build()
        .unwrap();
    let r = client.list_instances_with_object(&ql, &None).await;
    println!(" -- > {:?}", r);
    loop {}
    // println!(" -- > DeRegister {:?}", &client.deregister().await);
}

fn test_nacos_config() -> NacosConfig {
    NacosConfig::builder()
        .scheme("http".to_string())
        .nacos_ip("nacos.vvmm.ink".to_string())
        .nacos_port(8848)
        .nacos_user("nacos".to_string())
        .nacos_pass("vvmm".to_string())
        .auth(true)
        .build()
        .unwrap()
}

#[cfg(test)]
mod server_test {
    use crate::test_nacos_config;
    use nacos_rs_sdk::NacosConfig;

    #[tokio::test]
    async fn test_get_addr_simple() {
        let addr = test_client().get_addr_simple("test").await.unwrap();
        println!(" -- > addr : {}", addr);
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance = NacosServiceApi::get_instance(
            test_client().nacos_config(),
            "test",
            "127.0.0.1",
            8080,
            &None,
        )
        .await;
        println!(" -- > instance : {:?}", instance);
    }

    #[tokio::test]
    async fn test_get_server() {
        let server = NacosServiceApi::get_server(test_client().nacos_config(), "test", &None).await;
        println!(" -- > server : {:?}", server);
    }

    #[tokio::test]
    async fn test_get_server_list() {
        let server_list =
            NacosServiceApi::get_server_list(test_client().nacos_config(), 1, 10, &None).await;
        println!(" -- > server_list : {:?}", server_list);
    }

    #[tokio::test]
    async fn test_get_operator_servers() {
        let operator_servers =
            NacosServiceApi::get_operator_servers(test_client().nacos_config()).await;
        println!(" -- > operator_servers : {:?}", operator_servers);
    }

    #[tokio::test]
    async fn test_get_operator_metrics() {
        let operator_metrics =
            NacosServiceApi::get_operator_metrics(test_client().nacos_config()).await;
        println!(" -- > operator_metrics : {:?}", operator_metrics);
    }
}
