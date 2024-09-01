use kube::{
    api::{Api, ListParams},
    Client, Resource,
};
use rocket::serde::DeserializeOwned;
use std::fmt::Debug;
use thiserror::Error;

/// Enum of possible API error responses
#[derive(Responder, Error, Debug)]
pub enum ApiError<'a> {
    #[error("Error: {:?}", .0)]
    #[response(status = 503)]
    KubernetesError(&'a str),
}

async fn list_resources<K: Resource + Debug + DeserializeOwned + Clone>(
    client: &Client,
) -> Result<Vec<K>, ApiError>
where
    <K as Resource>::DynamicType: Default,
{
    // NOTE: It looks like client clones are cheap, so using to_owned here is fine
    // https://github.com/kube-rs/kube/blob/0.93.1/kube-client/src/client/mod.rs#L76-L81
    match Api::<K>::all(client.to_owned())
        .list(&ListParams::default())
        .await
    {
        Ok(response) => Ok(response.items),
        Err(kube::Error::Api(kube::core::ErrorResponse { code: 404, .. })) => Err(
            ApiError::KubernetesError("Flux CRDs not installed on cluster"),
        ),
        Err(e) => {
            println!("{:?}", e);
            Err(ApiError::KubernetesError("Kubernetes API unavailable"))
        }
    }
}

pub async fn fetch_view<K: Debug + Clone + kube::Resource + DeserializeOwned, V>(
    client: &Client,
) -> Result<Vec<V>, ApiError>
where
    V: From<K>,
    <K as kube::Resource>::DynamicType: Default,
{
    Ok(list_resources(client)
        .await?
        .into_iter()
        .map(|hr: K| V::from(hr))
        .collect())
}
