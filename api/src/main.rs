#[macro_use]
extern crate rocket;

use k8s_openapi::api::core::v1::Pod;
use kube::api::Api;
use kube::{api::LogParams, Client, ResourceExt};
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
    futures::{AsyncBufReadExt, StreamExt},
    response::stream,
    serde::json::Json,
    Shutdown, State,
};

mod fluxcd;
use fluxcd::{
    utils::{fetch_view, try_get_resource, ApiError},
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

#[get("/controllers/<name>/logs")]
async fn stream_logs<'a>(
    name: &'a str,
    client: &State<Client>,
    mut shutdown: Shutdown,
) -> Result<stream::EventStream![], ApiError> {
    // Limit which controller logs are streamable
    let controllers = ["helm", "source", "kustomize"];
    if !controllers.contains(&name) {
        return Err(ApiError::ResourceNotFound(format!(
            "{} controller",
            name.to_owned()
        )));
    }

    // TODO: Handle case where there are multiple replicas
    let controller = try_get_resource::<Pod>(client, &format!("{}-controller", name)).await?;
    let api = Api::<Pod>::namespaced(
        client.inner().clone(),
        &controller.namespace().unwrap_or_default(),
    );

    let mut log_stream = api
        .log_stream(
            &controller.name_any(),
            &LogParams {
                follow: true,          // Follow is key for continual output stream
                tail_lines: Some(100), // Limit line count to avoid overloading frontend
                ..LogParams::default()
            },
        )
        .await
        .expect("log stream request to succeed")
        .lines();

    Ok(stream::EventStream! {
        while let Some(line) = log_stream.next().await {
            let text = line.expect("log event to be Ok");
            // Handle shutdown gracefully using select
            rocket::tokio::select! {
                _ = log_stream.next() => yield stream::Event::data(text),
                _ = &mut shutdown => {
                    break;
                }
            }
        }
    })
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
                oci_repos,
                stream_logs,
            ],
        )
        // Add shared state accessible from handlers
        .manage(client)
}

#[cfg(test)]
mod test {

    use super::*;
    use kube::{api::LogParams, ResourceExt};
    use rocket::{
        async_test,
        futures::{AsyncBufReadExt, StreamExt, TryStreamExt},
        http::{ContentType, Status},
        local::asynchronous::Client,
    };

    async fn make_client() -> Client {
        Client::tracked(rocket().await)
            .await
            .expect("a valid rocket instance")
    }

    /// Test that all relevant API endpoints return a
    /// successful JSON or SSE response or a plain text error
    #[async_test]
    async fn test_api_responses_json() {
        let client = make_client().await;
        for route in client.rocket().routes() {
            let path = route.uri.path();
            // Logs paths should return event stream not JSON
            if !path.contains("<name>/logs") {
                let response = client.get(path).dispatch().await;
                println!("{:?}", response);
                let expected_content_type = match response.status().code {
                    200 => ContentType::JSON,
                    _ => ContentType::Text,
                };
                assert_eq!(
                    response.content_type().expect("content-type to be set"),
                    expected_content_type
                );
            }
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

    #[async_test]
    async fn test_streaming() {
        use k8s_openapi::api::core::v1::Pod;
        use kube::{
            api::{Api, ListParams},
            Client,
        };

        let client = Client::try_default()
            .await
            .expect("Kubernetes client to be inferrable from runtime environment");
        let api = Api::namespaced(client, "default");
        let pods: Vec<Pod> = api.list(&ListParams::default()).await.unwrap().items;
        let p: &Pod = pods.first().unwrap();
        // println!("{:?}", p);

        // let logs = api.logs(&p.name_any(), &LogParams::default()).await;
        // println!("{:?}", logs);

        let mut log_stream = api
            .log_stream(&p.name_any(), &LogParams::default())
            .await
            .unwrap()
            .lines()
            .into_stream();

        while let Some(line) = log_stream.next().await {
            println!("{:?}", line)
        }
    }
}
