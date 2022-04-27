use std::time::Duration;
use std::{collections::BTreeMap, sync::Arc};

use k8s_openapi::{
    api::core::v1::{Pod, PodSpec},
    apimachinery::pkg::apis::meta::v1::Time,
};
use kube::{
    api::{ListParams, Patch},
    core::ObjectMeta,
    runtime::controller::{Action, Context},
    Api, Client, CustomResource, Resource,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod cronjob;

#[derive(Clone, Debug, CustomResource, Serialize, Deserialize, JsonSchema)]
#[kube(
    kind = "PodManager",
    group = "bestgopher.com",
    version = "v1",
    namespaced
)]
#[kube(status = "Status")]
pub struct Spec {
    template: PodSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Status {
    create_time: Option<Time>,
}

// Context for our reconciler
#[derive(Clone)]
pub struct Data {
    client: Client,
}

impl Data {
    pub fn new(client: Client) -> Data {
        Data { client }
    }
}

pub async fn reconciler(
    manager: Arc<PodManager>,
    ctx: Context<Data>,
) -> Result<Action, kube::Error> {
    println!("reconcil starts");

    let api = Api::<PodManager>::namespaced(
        ctx.get_ref().client.clone(),
        manager.meta().namespace.as_ref().unwrap().as_str(),
    );

    // 获取最新的资源
    let manager = api.get(manager.metadata.name.as_ref().unwrap()).await?;

    let pods = Api::<Pod>::default_namespaced(ctx.get_ref().client.clone());

    let podfilter = ListParams::default()
        .labels(format!("owned-by={}", manager.metadata.name.as_ref().unwrap()).as_ref());

    let pod = match pods.list(&podfilter).await?.into_iter().next() {
        Some(p) => p,
        None => {
            let pod_data = create_owned_pod(&manager);
            pods.create(&Default::default(), &pod_data).await?
        }
    };

    if let Some(ref status) = manager.status {
        if status.create_time.is_none() {
            // 更新create_time字段
            let patch = json! {
                {
                    "status": Status{
                        create_time: pod.meta().creation_timestamp.clone(),
                    }
                }
            };

            api.patch_status(
                manager.meta().name.as_ref().unwrap().as_str(),
                &Default::default(),
                &Patch::Merge(patch),
            )
            .await?;
        }
    }

    // 使用server-side apply，但是保留上面的检查可以减少网络的调用
    // let pod_data = create_owned_pod(&manager);
    // let patch_params = PatchParams::default();
    // let pod_name = pod_data.meta().name.as_ref().unwrap().as_str();
    // let pod = pods
    //     .patch(pod_name, &patch_params, &Patch::Apply(&pod_data))
    //     .await?;

    // let patch = json! {
    //     {
    //         "status": Status{
    //             create_time: pod.meta().creation_timestamp.clone(),
    //         }
    //     }
    // };

    // let pod_manager_name = manager.meta().name.as_ref().unwrap().as_str();
    // api.patch_status(pod_manager_name, &Default::default(), &Patch::Apply(patch))
    //     .await?;

    Ok(Action::await_change())
}

fn create_owned_pod(source: &PodManager) -> Pod {
    let oref = source.controller_owner_ref(&()).unwrap();
    let mut lables = BTreeMap::new();
    lables.insert(
        "owned-by".to_string(),
        source.metadata.name.as_ref().unwrap().to_string(),
    );
    lables.insert("managed_my".to_string(), "podmanager".to_string());

    Pod {
        metadata: ObjectMeta {
            name: source.metadata.name.clone(),
            owner_references: Some(vec![oref]),
            labels: Some(lables),
            ..Default::default()
        },

        spec: Some(source.spec.template.clone()),
        ..Default::default()
    }
}

pub fn error_policy(error: &kube::Error, _ctx: Context<Data>) -> Action {
    println!("reconcil failed: {:?}", error);
    Action::requeue(Duration::from_secs(5 * 60))
}
