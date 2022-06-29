use super::*;

pub async fn setup_end_to_end() -> Client {
    let database = Box::new(
        crate::database::sql::SqlAccess::new("postgresql://test:test@localhost:5432/test")
            .await
            .expect("Failed to access the database"),
    );
    let web_client = Box::new(crate::web_client::reqwest_client::ReqwestClient::new());
    setup(database, web_client).await
}
