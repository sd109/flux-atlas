#[macro_use]
extern crate rocket;

use kube::Client;
use kube_custom_resources_rs::{
    helm_toolkit_fluxcd_io::v2::helmreleases::HelmRelease,
    kustomize_toolkit_fluxcd_io::v1::kustomizations::Kustomization,
    source_toolkit_fluxcd_io::{
        v1::{
            gitrepositories::GitRepository, helmcharts::HelmChart, helmrepositories::HelmRepository,
        },
        v1beta2::ocirepositories::OCIRepository,
    },
};
use rocket::{http::Header, serde::json::Json, State};

mod fluxcd;
use fluxcd::{
    fetch_view, GitRepoView, HelmChartView, HelmReleaseView, HelmRepoView, KustomizationView,
    OCIRepoView,
};

#[derive(Responder)]
#[response(content_type = "json")]
struct CorsResponder<T> {
    inner: Json<T>,
    cors: Header<'static>,
}

impl<T> CorsResponder<T> {
    fn new(inner: Json<T>, allowed_origins: &str) -> Self {
        CorsResponder {
            inner,
            cors: Header::new("Access-Control-Allow-Origin", allowed_origins.to_owned()),
        }
    }
}

#[get("/git-repos")]
async fn git_repos(client: &State<Client>) -> CorsResponder<Vec<GitRepoView>> {
    CorsResponder::new(
        Json(fetch_view::<GitRepository, GitRepoView>(client).await),
        "*",
    )
}

#[get("/oci-repos")]
async fn oci_repos(client: &State<Client>) -> CorsResponder<Vec<OCIRepoView>> {
    CorsResponder::new(
        Json(fetch_view::<OCIRepository, OCIRepoView>(client).await),
        "*",
    )
}

#[get("/helm-repos")]
async fn helm_repos(client: &State<Client>) -> CorsResponder<Vec<HelmRepoView>> {
    CorsResponder::new(
        Json(fetch_view::<HelmRepository, HelmRepoView>(client).await),
        "*",
    )
}

#[get("/helm-charts")]
async fn helm_charts(client: &State<Client>) -> CorsResponder<Vec<HelmChartView>> {
    CorsResponder::new(
        Json(fetch_view::<HelmChart, HelmChartView>(client).await),
        "*",
    )
}

#[get("/helm-releases")]
async fn helm_releases(client: &State<Client>) -> CorsResponder<Vec<HelmReleaseView>> {
    CorsResponder::new(
        Json(fetch_view::<HelmRelease, HelmReleaseView>(client).await),
        "*",
    )
}

#[get("/kustomizations")]
async fn kustomizations(client: &State<Client>) -> CorsResponder<Vec<KustomizationView>> {
    CorsResponder::new(
        Json(fetch_view::<Kustomization, KustomizationView>(client).await),
        "*",
    )
}

#[launch]
async fn rocket() -> _ {
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default()
        .await
        .expect("Kubernetes client to be inferrable from runtime environment");

    rocket::build().manage(client).mount(
        "/api/",
        routes![
            helm_repos,
            helm_charts,
            helm_releases,
            kustomizations,
            git_repos,
            oci_repos
        ],
    )
}
