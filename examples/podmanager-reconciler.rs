#![allow(dead_code, unused_variables)]

use std::sync::Arc;
use std::time::Duration;

use futures::StreamExt;
use k8s_openapi::{
    api::core::v1::{Pod, PodSpec},
    apimachinery::pkg::apis::meta::v1::Time,
};
use kube::{
    api::{ListParams, Patch, PatchParams},
    core::ObjectMeta,
    runtime::{
        controller::{Action, Context},
        Controller,
    },
    Api, Client, CustomResource, Resource,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    let context = Context::new(Data {
        client: client.clone(),
    });

    let api = Api::<PodManager>::all(client.clone());

    // Ensure CRD is installed before loop-watching
    let _r = api
        .list(&ListParams::default().limit(1))
        .await
        .expect("is the crd installed? please run: cargo run --bin crdgen | kubectl apply -f -");

    Controller::new(api, ListParams::default().limit(1))
        .run(reconciler, error_policy, context)
        .filter_map(|x| async move { anyhow::Result::ok(x) })
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}

#[derive(Clone, Debug, CustomResource, Serialize, Deserialize, JsonSchema)]
#[kube(
    kind = "PodManager",
    group = "bestgopher.com",
    version = "v1",
    namespaced
)]
#[kube(status = "Status")]
struct Spec {
    template: PodSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct Status {
    create_time: Option<Time>,
}

// Context for our reconciler
#[derive(Clone)]
struct Data {
    client: Client,
}

async fn reconciler(manager: Arc<PodManager>, ctx: Context<Data>) -> Result<Action, kube::Error> {
    let api = Api::<PodManager>::namespaced(
        ctx.get_ref().client.clone(),
        manager.meta().namespace.as_ref().unwrap().as_str(),
    );
    let pods = Api::<Pod>::default_namespaced(ctx.get_ref().client.clone());

    let podfilter = ListParams::default()
        .labels(format!("owned_by/{}", manager.metadata.name.as_ref().unwrap()).as_ref());

    // let pod = match pods.list(&podfilter).await?.into_iter().next() {
    //     Some(p) => p,
    //     None => {
    //         let pod_data = create_owned_pod(&manager);
    //         pods.create(&Default::default(), &pod_data).await?
    //     }
    // };

    // if let Some(ref status) = manager.as_ref().status {
    //     if status.create_time.is_none() {
    //         // 更新create_time字段
    //         let patch = json! {
    //             {
    //                 "status": Status{
    //                     create_time: pod.meta().creation_timestamp.clone(),
    //                 }
    //             }
    //         };

    //         api.patch_status(
    //             manager.as_ref().meta().name.as_ref().unwrap().as_str(),
    //             &Default::default(),
    //             &Patch::Merge(patch),
    //         )
    //         .await?;
    //     }
    // }

    // 使用server-side apply，但是保留上面的检查可以减少网络的调用
    let pod_data = create_owned_pod(&manager);
    let patch_params = PatchParams::default();
    let pod_name = pod_data.meta().name.as_ref().unwrap().as_str();
    let pod = pods
        .patch(pod_name, &patch_params, &Patch::Apply(&pod_data))
        .await?;

    let patch = json! {
        {
            "status": Status{
                create_time: pod.meta().creation_timestamp.clone(),
            }
        }
    };

    let pod_manager_name = manager.meta().name.as_ref().unwrap().as_str();
    api.patch_status(pod_manager_name, &Default::default(), &Patch::Apply(patch))
        .await?;

    Ok(Action::await_change())
}

fn create_owned_pod(source: &PodManager) -> Pod {
    let oref = source.controller_owner_ref(&()).unwrap();

    Pod {
        metadata: ObjectMeta {
            name: source.metadata.name.clone(),
            owner_references: Some(vec![oref]),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn error_policy(error: &kube::Error, ctx: Context<Data>) -> Action {
    Action::requeue(Duration::from_secs(5 * 60))
}
