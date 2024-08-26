use kube_custom_resources_rs::helm_toolkit_fluxcd_io::v2::helmreleases::HelmRelease;
use rocket::serde::Serialize;

use super::utils::latest_status;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HelmReleaseView {
    name: String,
    namespace: String,
    chart_ref: String,
    status: String,
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
            status: match hr.status {
                Some(status) => latest_status(status.conditions),
                None => String::new(),
            },
            suspended: hr.spec.suspend.unwrap_or(false),
        }
    }
}
