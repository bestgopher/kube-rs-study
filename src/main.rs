use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::ListParams,
    runtime::{controller::Context, Controller},
    Api, Client,
};
use kube_study::{error_policy, reconciler, Data, PodManager};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let logger = tracing_subscriber::fmt::layer().json();
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let collector = Registry::default().with(logger).with(env_filter);
    tracing::subscriber::set_global_default(collector).unwrap();
    

    let client = Client::try_default().await?;

    let context = Context::new(Data::new(client.clone()));

    let pod_manager_api = Api::<PodManager>::all(client.clone());
    let pod_api = Api::<Pod>::all(client.clone());

    // Ensure CRD is installed before loop-watching
    let _r = pod_manager_api
        .list(&ListParams::default().limit(1))
        .await
        .expect("is the crd installed? please run: cargo run --bin crdgen | kubectl apply -f -");

    let _r = pod_api
        .list(&ListParams::default().limit(1))
        .await
        .expect("cant get pods resource");

    Controller::new(pod_manager_api, ListParams::default())
        .owns(
            pod_api,
            ListParams::default().labels("managed_my=podmanager"),
        )
        // .owns(pod_api, ListParams::default())
        .run(reconciler, error_policy, context)
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}
