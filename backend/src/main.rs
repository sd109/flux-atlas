#[macro_use]
extern crate rocket;

use kube::Client;
use rocket::{futures::io::Cursor, http::Header, response, serde::json::Json, Response, State};

mod fluxcd;
use fluxcd::{HelmReleaseView, KustomizationView};

#[derive(Responder)]
#[response(content_type = "json")]
struct CustomResponder<T> {
    inner: Json<T>,
    cors: Header<'static>,
}

// NOTE: VSCode highlights a macro-error here when state is incldued
// which seems to be a bug / red-herring and can be ignored.
// #[get("/helmreleases")]
// async fn helm_releases(client: &State<Client>) -> Json<Vec<HelmReleaseView>> {
//     Json(HelmReleaseView::fetch(client).await)
// }

#[get("/helmreleases")]
async fn helm_releases(client: &State<Client>) -> CustomResponder<Vec<HelmReleaseView>> {
    CustomResponder {
        inner: Json(HelmReleaseView::fetch(client).await),
        cors: Header::new("Access-Control-Allow-Origin", "*"),
    }
}

// #[get("/helmreleases")]
// async fn helm_releases(client: &State<Client>) -> Response {
//     let json =
//         rocket::serde::json::serde_json::to_string(&HelmReleaseView::fetch(client).await).unwrap();
//     Response::build()
//         .raw_header("Access-Control-Allow-Origin", "*")
//         .sized_body(json.len(), Cursor::new(json))
//         .finalize()
// }

#[get("/kustomizations")]
async fn kustomizations(client: &State<Client>) -> Json<Vec<KustomizationView>> {
    Json(KustomizationView::fetch(client).await)
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
