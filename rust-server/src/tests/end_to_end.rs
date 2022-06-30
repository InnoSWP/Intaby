use super::*;

pub async fn setup_end_to_end() -> Client {
    let web_client = Box::new(crate::web_client::reqwest_client::ReqwestClient::new());
    setup(web_client).await
}
