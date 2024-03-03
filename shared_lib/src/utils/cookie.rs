use std::{collections::HashMap, env};

use cookie::{Cookie, CookieJar, Key};
use lambda_runtime::LambdaEvent;
use serde_json::Value;

use crate::RequestPayload;

pub fn parse_cookie(event: &LambdaEvent<RequestPayload>) -> Option<String> {
    let cookie_secret = env::var("COOKIE_SECRET").unwrap_or_default();
    let cookie_name = env::var("COOKIE_NAME").unwrap_or_default();

    let default_headers_hash_map: HashMap<String, Value> = HashMap::new();
    let cookie_string = if let Some(cookie) = event
        .payload
        .headers
        .as_ref()
        .unwrap_or(&default_headers_hash_map)
        .get("cookie")
    {
        cookie.as_str().unwrap_or_default().to_owned()
    } else {
        "".to_owned()
    };

    match Cookie::parse(cookie_string) {
        Ok(token) => {
            let mut jar = CookieJar::new();
            jar.add_original(token);
            let key = Key::from(cookie_secret.as_bytes());
            let cookie = jar
                .private(&key)
                .get(&cookie_name)
                .unwrap_or_else(|| Cookie::new("", ""));
            let session_token = cookie.value();
            return Some(session_token.to_owned());
        }
        Err(_) => return None,
    }
}
