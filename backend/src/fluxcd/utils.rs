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
    // NOTE: It looks like client clones are cheap, so using to_owned here is fine
    // https://github.com/kube-rs/kube/blob/0.93.1/kube-client/src/client/mod.rs#L76-L81
    Api::<K>::all(client.to_owned())
        .list(&ListParams::default())
        .await
        .unwrap() // TODO: Handle errors here
        .items
}

pub async fn fetch_view<K: Debug + Clone + kube::Resource + DeserializeOwned, V>(
    client: &Client,
) -> Vec<V>
where
    V: From<K>,
    <K as kube::Resource>::DynamicType: Default,
{
    list_resources(client)
        .await
        .into_iter()
        .map(|hr: K| V::from(hr))
        .collect()
}
