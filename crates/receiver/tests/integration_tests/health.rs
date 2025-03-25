use poem::test::TestClient;

use saasaparilla_notification_receiver::app;

#[tokio::test]
async fn health() {
    let test_client = TestClient::new(app());

    let response = test_client.get("/healthz").send().await;

    response.assert_status_is_ok();
    response.assert_bytes("").await;
}
