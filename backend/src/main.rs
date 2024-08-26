#[macro_use]
extern crate rocket;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Test {
    hello: String,
}

async fn fetch() {
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await.unwrap();

    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let pods: Api<Pod> = Api::default_namespaced(client);
    for p in pods.list(&ListParams::default()).await.unwrap() {
        println!("found pod {}", p.name_any());
    }
}

#[get("/")]
async fn index() -> Json<Test> {
    fetch().await;

    // "Hello, world!"
    return Json(Test {
        hello: "World".to_string(),
    });
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/", routes![index])
}
