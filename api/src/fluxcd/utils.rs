use kube::{
    api::{Api, ListParams},
    Client, Resource,
};
use rocket::{
    form::validate::Contains,
    serde::{DeserializeOwned, Serialize},
};
use std::fmt::Debug;
use thiserror::Error;

/// Generic Flux CD source reference
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SourceRef<T> {
    pub kind: T,
    pub name: String,
    pub namespace: String,
}

/// Enum of possible API error responses
#[derive(Responder, Error, Debug)]
pub enum ApiError {
    #[error("Error: {:?}", .0)]
    #[response(status = 503)]
    KubernetesError(String),

    #[error("Resource: {:?}", .0)]
    #[response(status = 404)]
    ResourceNotFound(String),
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
            ApiError::KubernetesError("Flux CRDs not installed on cluster".into()),
        ),
        Err(e) => {
            println!("{:?}", e);
            Err(ApiError::KubernetesError(
                "Kubernetes API unavailable".into(),
            ))
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

pub async fn try_get_resource<'a, K: Resource + Debug + DeserializeOwned + Clone>(
    client: &'a Client,
    name: &'a str,
) -> Result<K, ApiError>
where
    <K as Resource>::DynamicType: Default,
{
    let resource_list = list_resources::<K>(client).await?;
    if let Some(r) = resource_list
        .into_iter()
        .find(|r| r.meta().name.as_ref().unwrap().contains(name))
    {
        // TODO: Use filter instead of find here and handle
        // cases where there are multiple matches
        return Ok(r);
    }
    return Err(ApiError::ResourceNotFound(name.into()));
}
