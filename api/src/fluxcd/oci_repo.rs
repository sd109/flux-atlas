use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube_custom_resources_rs::source_toolkit_fluxcd_io::v1beta2::ocirepositories::{
    OCIRepository, OCIRepositoryRef,
};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct OCIRepoView {
    name: String,
    namespace: String,
    url: String,
    conditions: Vec<Condition>,
    target_ref: VersionRef,
    interval: String,
    suspended: bool,
}

impl From<OCIRepository> for OCIRepoView {
    fn from(repo: OCIRepository) -> Self {
        OCIRepoView {
            name: repo.metadata.name.unwrap_or_default(),
            namespace: repo.metadata.namespace.unwrap_or_default(),
            url: repo.spec.url,
            conditions: match repo.status {
                Some(status) => status.conditions.unwrap_or_default(),
                None => vec![],
            },
            target_ref: repo.spec.r#ref.into(),
            interval: repo.spec.interval,
            suspended: repo.spec.suspend.unwrap_or(false),
        }
    }
}

// Utility types

#[derive(Serialize)]
#[serde(crate = "rocket::serde", tag = "type", rename_all = "lowercase")]
enum VersionRef {
    Digest { version: String },
    SemVer { version: String },
    Tag { version: String },
}

impl Default for VersionRef {
    fn default() -> Self {
        Self::Tag {
            version: "latest".to_string(),
        }
    }
}

impl From<Option<OCIRepositoryRef>> for VersionRef {
    fn from(ref_: Option<OCIRepositoryRef>) -> Self {
        match ref_ {
            Some(ref_) => {
                if let Some(val) = ref_.digest {
                    Self::Digest { version: val }
                } else if let Some(val) = ref_.semver {
                    Self::SemVer { version: val }
                } else if let Some(val) = ref_.tag {
                    Self::Tag { version: val }
                } else {
                    Self::default()
                }
            }
            None => Self::default(),
        }
    }
}
