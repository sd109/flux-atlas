use kube::{api::ListParams, Api, Client};
use kube_custom_resources_rs::helm_toolkit_fluxcd_io::v2::helmreleases::HelmRelease;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HelmReleaseView {
    name: String,
    namespace: String,
    chart: String,
}

impl From<HelmRelease> for HelmReleaseView {
    fn from(hr: HelmRelease) -> Self {
        HelmReleaseView {
            name: hr.metadata.name.unwrap_or_default(),
            namespace: hr.metadata.namespace.unwrap_or_default(),
            chart: match hr.spec.chart_ref {
                Some(chart_ref) => chart_ref.name,
                None => String::new(),
            },
        }
    }
}

impl HelmReleaseView {
    pub async fn fetch(client: &Client) -> Vec<HelmReleaseView> {
        Api::all(client.to_owned())
            .list(&ListParams::default())
            .await
            .unwrap()
            .items
            .into_iter()
            .map(|hr: HelmRelease| HelmReleaseView::from(hr))
            .collect()
    }
}
