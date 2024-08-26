#[macro_use]
extern crate rocket;

use kube::Client;
use rocket::{http::Header, serde::json::Json, State};

mod fluxcd;
use fluxcd::{HelmReleaseView, KustomizationView};

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

// NOTE: VSCode highlights a macro-error here when state is incldued
// which seems to be a bug / red-herring and can be ignored.
#[get("/helmreleases")]
async fn helm_releases(client: &State<Client>) -> CorsResponder<Vec<HelmReleaseView>> {
    CorsResponder::new(Json(HelmReleaseView::fetch(client).await), "*")
}

#[get("/kustomizations")]
async fn kustomizations(client: &State<Client>) -> CorsResponder<Vec<KustomizationView>> {
    CorsResponder::new(Json(KustomizationView::fetch(client).await), "*")
}

#[launch]
async fn rocket() -> _ {
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default()
        .await
        .expect("Kubernetes client to be inferrable from runtime environment");

    rocket::build()
        .manage(client)
        .mount("/api/", routes![helm_releases, kustomizations])
}
