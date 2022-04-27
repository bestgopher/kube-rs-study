use kube::{Client, discovery, Api, core::DynamicObject};

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let client = Client::try_default().await?;
    let apigroup = discovery::group(&client, "batch.tutorial.kubebuilder.io").await?;
    let (ar, caps) = apigroup.recommended_kind("CronJob").unwrap();

    let api = Api::<DynamicObject>::namespaced_with(client, "default", &ar);
    println!("get, {:?}", ar);
    println!("get, {:?}", caps);
    let r = api.list(&Default::default()).await?;
   
    for object in r.iter() {
        println!("{:?}",  object.metadata.name);
    }

    let object = api.get("cronjob-sampl1e").await?;
    println!("{:?}",  object.metadata.name);

    Ok(())
}
