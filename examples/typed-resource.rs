#![allow(unused)]

use k8s_openapi::{api::{core::v1::Pod, apps::v1::Deployment}};
use kube::{
    api::{ListParams, WatchEvent, PatchParams, Patch, DynamicObject},
    Api, Client,
    discovery,
};

use futures::{StreamExt, TryStreamExt};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let pods = Api::<Pod>::namespaced(client.clone(), "kube-system");
    pods.get("kube-flannel-ds-czj6l").await.map_err(|e| {println!("find kube-flannel-ds-czj6l eeer"); e})?;
    let pod_client: Api<Pod> = Api::namespaced(client.clone(), "default");
    let pod_list = pod_client.list(&Default::default()).await?;

    pod_list.into_iter().for_each(|pod| {
        println!("{:?}", pod.metadata.name.unwrap());
    });

    let pod = pod_client.get("wordpress-mariadb-0").await?;
    println!("{:?}", pod.metadata.name.unwrap());
    

    let deployment = Api::<Deployment>::namespaced(client.clone(), "default");

    let patch = json!(
        {
            "spec": {
                "replicas": 2
            }
        }
    );

    let pp = PatchParams::apply("resourceVersion");
    let r = deployment.patch("aaa-anvil", &pp, &Patch::Merge(patch)).await?;
    assert_eq!(r.spec.unwrap().replicas.unwrap(), 2);


    Ok(())
}

async fn pod_watch(client: Api<Pod>) -> Result<(), kube::Error>{
    let mut pod_list = client.watch(&ListParams::default(), "").await?.boxed();
    
    while let Some(state) = pod_list.try_next().await? {
        match state {
            WatchEvent::Added(s) => {}
            WatchEvent::Deleted(s) => {
                println!("delete {}", s.metadata.name.unwrap());
            }
            WatchEvent::Modified(s) => {}
            WatchEvent::Error(err) => {}
            _ => {}
        }
    }

    Ok(())
}
