/*
 * Imgur API
 *
 * Imgur's API exposes the entire Imgur infrastructure via a standardized programmatic interface. Using Imgur's API, you can do just about anything you can do on imgur.com, while using your programming language of choice.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountUserFollow {
    #[serde(rename = "status")]
    pub status: bool,
}

impl AccountUserFollow {
    pub fn new(status: bool) -> AccountUserFollow {
        AccountUserFollow {
            status,
        }
    }
}


