use rpi_client::run;
use tokio;


#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::{test, web, App};
    use actix_web::error::UrlencodedError::ContentType;

    #[actix_web::test]
    async fn health_check_works() {
        let app = test::init_service(App::new().service(rpi_client::health)).await;

        let req = test::TestRequest::get()
            .uri("/status/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

}

#[tokio::test]
async fn health_check_works(){
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/status/health")
        .send()
        .await
        .expect("Failed to execute request.");


    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app(){
    let server = rpi_client::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}



