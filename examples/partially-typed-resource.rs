#![allow(dead_code, unused)]

use k8s_openapi::api::core::v1::Pod;
use kube::{core::{Object, NotUsed}, Client, api::ApiResource, Api};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let ar = ApiResource::erase::<Pod>(&());

    let api = Api::<PodSimple>::namespaced_with(client.clone(), "default", &ar);
    println!("{:?}", api);
    if let Ok(object) = api.get("cronjob-sampl1e-1650962280-s64vx").await {
        println!("{}", object.spec.containers[0].image);
    };

    for object in api.list(&Default::default()).await? {
        println!("{}", object.spec.containers[0].image);
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PodSpecSimple {
    containers: Vec<ContainerSpec>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ContainerSpec {
    #[allow(dead_code)]
    image: String,
}

type PodSimple = Object<PodSpecSimple, NotUsed>;
