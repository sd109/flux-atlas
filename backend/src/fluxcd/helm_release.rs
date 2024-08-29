use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube_custom_resources_rs::helm_toolkit_fluxcd_io::v2::helmreleases::{
    HelmRelease, HelmReleaseChartRefKind,
};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HelmReleaseView {
    name: String,
    namespace: String,
    chart_ref: ChartRef,
    conditions: Vec<Condition>,
    suspended: bool,
    interval: String,
}

impl From<HelmRelease> for HelmReleaseView {
    fn from(hr: HelmRelease) -> Self {
        HelmReleaseView {
            name: hr.metadata.name.unwrap_or_default(),
            namespace: hr.metadata.namespace.clone().unwrap_or_default(),
            chart_ref: if let Some(chart_ref) = hr.spec.chart_ref {
                ChartRef {
                    kind: chart_ref.kind.into(),
                    name: chart_ref.name,
                    namespace: chart_ref
                        .namespace
                        .unwrap_or(hr.metadata.namespace.unwrap_or("default".to_string())),
                }
            } else {
                match hr.spec.chart {
                    Some(chart) => ChartRef {
                        kind: HelmReleaseChartRefKind::HelmChart.into(),
                        name: chart.spec.chart,
                        namespace: hr.metadata.namespace.unwrap_or("default".to_string()),
                    },
                    None => panic!(),
                }
            },
            conditions: match hr.status {
                Some(status) => status.conditions.unwrap_or_default(),
                None => vec![],
            },
            suspended: hr.spec.suspend.unwrap_or(false),
            interval: hr.spec.interval,
        }
    }
}

// Utility types

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ChartRef {
    kind: HelmReleaseChartRefKind,
    name: String,
    namespace: String,
}
