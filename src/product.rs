use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// "id": "prd_01JPKECHV0AWWEFHSCRCABB5B7",
//       "name": "flow",
//       "description": "Created in collaboration with Raycast, Flow is a medium roast from the Sidama region of Ethiopia. Featuring notes of red berries, tropical fruits, and caramel, this is the best way to get in the flow — the perfect state of productivity.",
//       "variants": [
//         {
//           "id": "var_01JPKECQ1YA7ACAM3RJFQSDTCG",
//           "name": "Medium Roast | 12oz | Whole Beans",
//           "price": 2200
//         }
//       ],
//       "tags": {
//         "color": "#000000",
//         "featured": true,
//         "market_eu": true,
//         "market_na": true
//       }

#[derive(Serialize, Deserialize, Debug)]
struct Variant {
    id: String,
    name: String,
    price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tags {
    color: String,
    featured: bool,
    market_eu: bool,
    market_na: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    id: String,
    name: String,
    description: String,
    variants: Vec<Variant>,
    tags: Tags,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    data: Vec<Product>,
}

pub async fn list(auth_token: &str) {
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
        let product_list_data: Data = match response.json().await {
            Ok(resp_json) => resp_json,
            Err(e) => {
                eprintln! {"ERROR: failed to decode response text: {}", e};
                std::process::exit(1);
            }
        };

        for product in product_list_data.data {
            println!("Response: {:?}\n\n", product);
        }
    } else {
        eprintln!("Error: {}", response.status());
    }
}
