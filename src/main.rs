use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;
use std::{fs::File, time::Duration};
use uuid::Uuid;
use std::io::Read;
use chrono::Utc;
use chrono::DateTime;
use toml::to_string as to_toml;
use serde_yaml::to_string as to_yaml;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    birth_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}
 
#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}
 
#[derive(Debug, Serialize, Deserialize)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    req_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}


fn main() {
    let mut file = File::open("request.json").expect("Failed to open file");
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).expect("Failed to read file");

    let request: Request = serde_json::from_str(&json_str).expect("Failed to parse JSON");
    let yaml_str = to_yaml(&request).unwrap();
    print!("{}", yaml_str);

    let toml_str = to_toml(&request).unwrap();
    print!("{}", toml_str);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use serde_json;

    fn get_request() -> Request {
        let mut file = File::open("request.json").expect("Cannot open request.json");
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).expect("Cannot read file");
        serde_json::from_str(&json_str).expect("Failed to parse JSON")
    }

    #[test]
    fn test_deserialization_from_json() {
        let request = get_request();

        assert_eq!(request.stream.public_tariff.id, 1);
        assert_eq!(request.stream.private_tariff.client_price, 250);
        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].description, "Gift 1");
    }

    #[test]
    fn test_serialization_to_json() {
        let request = get_request();

        let serialized = serde_json::to_string_pretty(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"type\": \"success\""));
        assert!(serialized.contains("\"Gift 1\""));
        assert!(serialized.contains("\"client_price\": 250"));
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_request = get_request();

        let json_data = serde_json::to_string_pretty(&original_request).expect("Failed to serialize");

        let parsed_request: Request = serde_json::from_str(&json_data).expect("Failed to parse back");

        assert_eq!(parsed_request.stream.public_tariff.id, original_request.stream.public_tariff.id);
        assert_eq!(parsed_request.stream.private_tariff.client_price, original_request.stream.private_tariff.client_price);
        assert_eq!(parsed_request.gifts.len(), original_request.gifts.len());
        assert_eq!(parsed_request.gifts[1].description, original_request.gifts[1].description);
    }
}
