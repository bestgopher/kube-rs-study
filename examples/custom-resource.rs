use kube::{Client, Api};

use kube_study::cronjob::CronJob;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let api = Api::<CronJob>::namespaced(client.clone(), "default");
    let r = api.get("cronjob-sampl1e").await?;
    println!("{:?}", r.metadata.name);

    Ok(())
}
