use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube_custom_resources_rs::kustomize_toolkit_fluxcd_io::v1::kustomizations::{
    Kustomization, KustomizationSourceRefKind,
};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct KustomizationView {
    name: String,
    namespace: String,
    source_ref: SourceRef,
    conditions: Vec<Condition>,
    suspended: bool,
    interval: String,
}

impl From<Kustomization> for KustomizationView {
    fn from(k: Kustomization) -> Self {
        let ns = k.metadata.namespace.unwrap_or_default();
        KustomizationView {
            name: k.metadata.name.unwrap_or_default(),
            namespace: ns.clone(),
            source_ref: SourceRef {
                kind: k.spec.source_ref.kind,
                name: k.spec.source_ref.name,
                namespace: k.spec.source_ref.namespace.unwrap_or(ns),
            },
            conditions: match k.status {
                Some(status) => status.conditions.unwrap_or_default(),
                None => vec![],
            },
            suspended: k.spec.suspend.unwrap_or(false),
            interval: k.spec.interval,
        }
    }
}

// Utility types

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct SourceRef {
    kind: KustomizationSourceRefKind,
    name: String,
    namespace: String,
}
