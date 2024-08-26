mod helm_release;
mod kustomization;

pub use helm_release::HelmReleaseView;
use k8s_openapi::Resource;
use kube::Client;
pub use kustomization::KustomizationView;

// use std::fmt::Debug;
// use k8s_openapi::{Metadata, Resource};
// use kube::{api::ListParams, Api, Client};
// use rocket::serde::DeserializeOwned;

// trait Fetch: Sized {
//     type ApiType: Resource + Metadata + Clone + Debug + DeserializeOwned;

//     async fn fetch(client: &Client) -> Vec<Self> {
//         Api::all(client.to_owned())
//             .list(&ListParams::default())
//             .await
//             .unwrap()
//             .items
//             .into_iter()
//             .map(|item: Self::ApiType| Self::from(item))
//             .collect()
//     }
// }
