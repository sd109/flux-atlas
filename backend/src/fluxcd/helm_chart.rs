use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube_custom_resources_rs::source_toolkit_fluxcd_io::v1::helmcharts::HelmChart;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HelmChartView {
    name: String,
    namespace: String,
    repo: String,
    chart: String,
    version: String,
    conditions: Vec<Condition>,
    suspended: bool,
}

impl From<HelmChart> for HelmChartView {
    fn from(hc: HelmChart) -> Self {
        HelmChartView {
            name: hc.metadata.name.unwrap_or_default(),
            namespace: hc.metadata.namespace.unwrap_or_default(),
            repo: hc.spec.source_ref.name,
            chart: hc.spec.chart,
            version: hc.spec.version.unwrap_or("*".to_owned()),
            conditions: match hc.status {
                Some(status) => status.conditions.unwrap_or_default(),
                None => vec![],
            },
            suspended: hc.spec.suspend.unwrap_or(false),
        }
    }
}
