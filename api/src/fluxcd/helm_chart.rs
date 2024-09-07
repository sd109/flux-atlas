use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube_custom_resources_rs::source_toolkit_fluxcd_io::v1::helmcharts::{
    HelmChart, HelmChartSourceRefKind,
};
use rocket::serde::Serialize;

use super::utils::SourceRef;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HelmChartView {
    name: String,
    namespace: String,
    source_ref: SourceRef<HelmChartSourceRefKind>,
    chart: String,
    version: String,
    conditions: Vec<Condition>,
    suspended: bool,
    interval: String,
    yaml: String,
}

impl From<HelmChart> for HelmChartView {
    fn from(hc: HelmChart) -> Self {
        let yaml = serde_yaml::to_string(&hc.clone()).expect("HelmChart to be yaml serializable");
        let namespace = hc.metadata.namespace.unwrap_or("default".to_string());

        HelmChartView {
            name: hc.metadata.name.unwrap_or_default(),
            namespace: namespace.clone(),
            source_ref: SourceRef {
                kind: hc.spec.source_ref.kind,
                name: hc.spec.source_ref.name,
                namespace,
            },
            chart: hc.spec.chart,
            version: hc.spec.version.unwrap_or("*".to_owned()),
            conditions: match hc.status {
                Some(status) => status.conditions.unwrap_or_default(),
                None => vec![],
            },
            suspended: hc.spec.suspend.unwrap_or(false),
            interval: hc.spec.interval,
            yaml,
        }
    }
}
