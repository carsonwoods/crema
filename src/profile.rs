use reqwest::Client;
use std::time::Duration;

pub async fn get(auth_token: &str) {
    let client = Client::new();
    let url = "https://api.terminal.shop/product";

    let response = match client
        .get(url)
        .bearer_auth(auth_token)
        .timeout(Duration::from_secs(10)) // Avoid hanging
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln! {"ERROR: list product request failed: {}", e};
            std::process::exit(1);
        }
    };

    if response.status().is_success() {
        let body = match response.text().await {
            Ok(resp_txt) => resp_txt,
            Err(e) => {
                eprintln! {"ERROR: failed to decode response text: {}", e};
                std::process::exit(1);
            }
        };
        println!("Response: {}", body);
    } else {
        eprintln!("Error: {}", response.status());
    }
}
