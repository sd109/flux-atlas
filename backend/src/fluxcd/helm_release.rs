use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube_custom_resources_rs::helm_toolkit_fluxcd_io::v2::helmreleases::HelmRelease;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HelmReleaseView {
    name: String,
    namespace: String,
    chart_ref: String,
    conditions: Vec<Condition>,
    suspended: bool,
}

impl From<HelmRelease> for HelmReleaseView {
    fn from(hr: HelmRelease) -> Self {
        HelmReleaseView {
            name: hr.metadata.name.unwrap_or_default(),
            namespace: hr.metadata.namespace.unwrap_or_default(),
            chart_ref: match hr.spec.chart_ref {
                Some(chart_ref) => chart_ref.name,
                None => String::new(),
            },
            conditions: match hr.status {
                Some(status) => status.conditions.unwrap_or_default(),
                None => vec![],
            },
            suspended: hr.spec.suspend.unwrap_or(false),
        }
    }
}
