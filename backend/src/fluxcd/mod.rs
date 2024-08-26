mod helm_release;
mod kustomization;

pub use helm_release::HelmReleaseView;
pub use kustomization::KustomizationView;

use kube::{
    api::{Api, ListParams},
    Client, Resource,
};
use rocket::serde::DeserializeOwned;
use std::fmt::Debug;

async fn list_resources<K: Resource + Debug + DeserializeOwned + Clone>(client: &Client) -> Vec<K>
where
    <K as Resource>::DynamicType: Default,
{
    Api::<K>::all(client.to_owned())
        .list(&ListParams::default())
        .await
        .unwrap() // TODO: Handle errors here
        .items
}
