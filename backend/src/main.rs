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
use rocket::{
    fairing::AdHoc,
    http::Status,
    serde::{json::Json, Serialize},
    Request, State,
};

mod fluxcd;
use fluxcd::{
    fetch_view, GitRepoView, HelmChartView, HelmReleaseView, HelmRepoView, KustomizationView,
    OCIRepoView,
};

#[get("/git-repos")]
async fn git_repos(client: &State<Client>) -> Json<Vec<GitRepoView>> {
    Json(
        fetch_view::<GitRepository, GitRepoView>(client)
            .await
            .unwrap(),
    )
}

#[get("/oci-repos")]
async fn oci_repos(client: &State<Client>) -> Json<Vec<OCIRepoView>> {
    Json(
        fetch_view::<OCIRepository, OCIRepoView>(client)
            .await
            .unwrap(),
    )
}

#[get("/helm-repos")]
async fn helm_repos(client: &State<Client>) -> Json<Vec<HelmRepoView>> {
    Json(
        fetch_view::<HelmRepository, HelmRepoView>(client)
            .await
            .unwrap(),
    )
}

#[get("/helm-charts")]
async fn helm_charts(client: &State<Client>) -> Json<Vec<HelmChartView>> {
    Json(
        fetch_view::<HelmChart, HelmChartView>(client)
            .await
            .unwrap(),
    )
}

#[get("/helm-releases")]
async fn helm_releases(client: &State<Client>) -> Json<Vec<HelmReleaseView>> {
    Json(
        fetch_view::<HelmRelease, HelmReleaseView>(client)
            .await
            .unwrap(),
    )
}

#[get("/kustomizations")]
async fn kustomizations(client: &State<Client>) -> Result<Json<Vec<KustomizationView>>, Status> {
    let response = fetch_view::<Kustomization, KustomizationView>(client).await?;
    Ok(Json(response))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    fn new<S: Into<String>>(error: S) -> Self {
        ErrorResponse {
            error: error.into(),
        }
    }
}

#[catch(503)]
fn handle_503_error<'a>(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::new(format!(
        "endpoint {} currently unavailable",
        req.uri()
    )))
}

#[launch]
async fn rocket() -> _ {
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default()
        .await
        .expect("Kubernetes client to be inferrable from runtime environment");

    rocket::build()
        .mount(
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
        .register("/api/", catchers![handle_503_error])
        // Add shared state accessible from handlers
        .manage(client)
        // Remove CORS restrictions from all responses
        .attach(AdHoc::on_response("CORS Header", |_, response| {
            Box::pin(async move {
                let cors_header = rocket::http::Header::new("Access-Control-Allow-Origin", "*");
                response.set_header(cors_header);
            })
        }))
}
