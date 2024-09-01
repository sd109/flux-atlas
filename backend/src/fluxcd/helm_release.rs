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
        let name = hr.metadata.name.unwrap_or_default();
        let namespace = hr.metadata.namespace.unwrap_or("default".to_string());
        HelmReleaseView {
            name: name.clone(),
            namespace: namespace.clone(),
            chart_ref: if let Some(chart_ref) = hr.spec.chart_ref {
                ChartRef {
                    kind: chart_ref.kind.into(),
                    name: chart_ref.name,
                    namespace: chart_ref.namespace.unwrap_or(namespace),
                }
            } else {
                // Chart template semantics:
                // https://fluxcd.io/flux/components/helm/helmreleases/#chart-template
                match hr.spec.chart {
                    Some(_) => ChartRef {
                        kind: HelmReleaseChartRefKind::HelmChart.into(),
                        name: format!("{}-{}", namespace, name),
                        namespace,
                    },
                    // Shouldn't happen, FluxCD docs state that either spec.chart_ref
                    // or spec.chart will always be populated.
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
