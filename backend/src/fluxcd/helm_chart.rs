use kube_custom_resources_rs::source_toolkit_fluxcd_io::v1::helmcharts::HelmChart;
use rocket::serde::Serialize;

use super::utils::latest_status;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HelmChartView {
    name: String,
    namespace: String,
    chart: String,
    repo: String,
    status: String,
}

impl From<HelmChart> for HelmChartView {
    fn from(hc: HelmChart) -> Self {
        HelmChartView {
            name: hc.metadata.name.unwrap_or_default(),
            namespace: hc.metadata.namespace.unwrap_or_default(),
            chart: hc.spec.chart,
            repo: hc.spec.source_ref.name,
            status: match hc.status {
                Some(status) => latest_status(status.conditions),
                None => String::new(),
            },
        }
    }
}
