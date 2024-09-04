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
    serde::{json::Json, Serialize},
    Request, State,
};

mod fluxcd;
use fluxcd::{
    utils::{fetch_view, ApiError},
    GitRepoView, HelmChartView, HelmReleaseView, HelmRepoView, KustomizationView, OCIRepoView,
};

#[get("/git-repos")]
async fn git_repos(client: &State<Client>) -> Result<Json<Vec<GitRepoView>>, ApiError> {
    let response = fetch_view::<GitRepository, GitRepoView>(client).await?;
    Ok(Json(response))
}

#[get("/oci-repos")]
async fn oci_repos(client: &State<Client>) -> Result<Json<Vec<OCIRepoView>>, ApiError> {
    let response = fetch_view::<OCIRepository, OCIRepoView>(client).await?;
    Ok(Json(response))
}

#[get("/helm-repos")]
async fn helm_repos(client: &State<Client>) -> Result<Json<Vec<HelmRepoView>>, ApiError> {
    let response = fetch_view::<HelmRepository, HelmRepoView>(client).await?;
    Ok(Json(response))
}

#[get("/helm-charts")]
async fn helm_charts(client: &State<Client>) -> Result<Json<Vec<HelmChartView>>, ApiError> {
    let response = fetch_view::<HelmChart, HelmChartView>(client).await?;
    Ok(Json(response))
}

#[get("/helm-releases")]
async fn helm_releases(client: &State<Client>) -> Result<Json<Vec<HelmReleaseView>>, ApiError> {
    let response = fetch_view::<HelmRelease, HelmReleaseView>(client).await?;
    Ok(Json(response))
}

#[get("/kustomizations")]
async fn kustomizations(client: &State<Client>) -> Result<Json<Vec<KustomizationView>>, ApiError> {
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

#[cfg(test)]
mod test {

    use super::*;
    use rocket::{
        async_test,
        http::{ContentType, Status},
        local::asynchronous::Client,
    };

    async fn make_client() -> Client {
        Client::tracked(rocket().await)
            .await
            .expect("a valid rocket instance")
    }

    /// Test that all API routes have a CORS header
    #[async_test]
    async fn test_cors_header() {
        let client = make_client().await;
        for route in client.rocket().routes() {
            let response = client.get(route.uri.path()).dispatch().await;
            assert!(response.headers().contains("Access-Control-Allow-Origin"));
        }
    }

    /// Test that all API endpoints return a
    /// JSON + success response
    #[async_test]
    async fn test_api_responses_json() {
        let client = make_client().await;
        for route in client.rocket().routes() {
            let response = client.get(route.uri.path()).dispatch().await;
            println!("{:?}", response);
            assert_eq!(
                response.content_type().expect("content-type to be set"),
                ContentType::JSON
            );
            assert_eq!(response.status(), Status::Ok);
        }
    }

    /// Test that all API routes return an error response
    /// when cluster is unreachable (e.g. when kube-context
    /// is a kind cluster and docker is stopped).
    #[ignore = "disable k8s cluster before running"]
    #[async_test]
    async fn test_cluster_unreachable() {
        let client = make_client().await;
        for route in client.rocket().routes() {
            let response = client.get(route.uri.path()).dispatch().await;
            println!("{:?}", response);
            assert_eq!(response.status(), Status::ServiceUnavailable);
            assert_eq!(
                response.content_type().expect("content-type to be set"),
                ContentType::Text
            );
        }
    }

    // #[test]
    // fn test_errors() {
    //     Err(ApiError::KubernetesError("test")).unwrap()
    // }
}
