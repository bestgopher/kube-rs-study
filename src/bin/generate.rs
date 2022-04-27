use kube::CustomResourceExt;

fn main() {
    println!("{}", serde_yaml::to_string(&kube_study::PodManager::crd()).unwrap())
}
