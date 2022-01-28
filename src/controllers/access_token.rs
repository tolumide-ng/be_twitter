use crate::app::client::AppClient;


// todo() - I should move all the controllers used to handle 2.0 authentication into one struct and represent them as methods within the struct
pub async fn access_token() {
    let app = AppClient::new();
    
}