use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube_custom_resources_rs::source_toolkit_fluxcd_io::v1::helmrepositories::HelmRepository;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HelmRepoView {
    name: String,
    namespace: String,
    url: String,
    conditions: Vec<Condition>,
    suspended: bool,
    interval: String,
    yaml: String,
}

impl From<HelmRepository> for HelmRepoView {
    fn from(hr: HelmRepository) -> Self {
        let yaml = serde_yaml::to_string(&hr.clone()).expect("HelmRepo to be yaml serializable");

        HelmRepoView {
            name: hr.metadata.name.unwrap_or_default(),
            namespace: hr.metadata.namespace.unwrap_or_default(),
            url: hr.spec.url,
            conditions: match hr.status {
                Some(status) => status.conditions.unwrap_or_default(),
                None => vec![],
            },
            suspended: hr.spec.suspend.unwrap_or(false),
            interval: hr.spec.interval.unwrap_or_default(),
            yaml,
        }
    }
}
