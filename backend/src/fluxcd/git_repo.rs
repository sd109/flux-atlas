use kube_custom_resources_rs::source_toolkit_fluxcd_io::v1::gitrepositories::{
    GitRepository, GitRepositoryRef,
};
use rocket::serde::Serialize;

use super::utils::latest_status;

#[derive(Serialize)]
#[serde(crate = "rocket::serde", tag = "type", rename_all = "lowercase")]
enum VersionRef {
    Commit { version: String },
    Name { version: String },
    SemVer { version: String },
    Tag { version: String },
    Branch { version: String },
}

impl Default for VersionRef {
    fn default() -> Self {
        Self::Branch {
            version: "master".to_string(),
        }
    }
}

impl From<Option<GitRepositoryRef>> for VersionRef {
    fn from(ref_: Option<GitRepositoryRef>) -> Self {
        match ref_ {
            Some(ref_) => {
                if let Some(val) = ref_.commit {
                    Self::Commit { version: val }
                } else if let Some(val) = ref_.name {
                    Self::Name { version: val }
                } else if let Some(val) = ref_.semver {
                    Self::SemVer { version: val }
                } else if let Some(val) = ref_.tag {
                    Self::Tag { version: val }
                } else if let Some(val) = ref_.branch {
                    Self::Branch { version: val }
                } else {
                    Self::default()
                }
            }
            None => Self::default(),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GitRepoView {
    name: String,
    namespace: String,
    source: String,
    status: String,
    target_ref: VersionRef,
    interval: String,
    suspended: bool,
}

impl From<GitRepository> for GitRepoView {
    fn from(gr: GitRepository) -> Self {
        GitRepoView {
            name: gr.metadata.name.unwrap_or_default(),
            namespace: gr.metadata.namespace.unwrap_or_default(),
            source: gr.spec.url,
            status: match gr.status {
                Some(status) => latest_status(status.conditions),
                None => String::new(),
            },
            target_ref: gr.spec.r#ref.into(),
            interval: gr.spec.interval,
            suspended: gr.spec.suspend.unwrap_or(false),
        }
    }
}
