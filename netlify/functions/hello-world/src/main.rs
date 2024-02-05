use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestEvent {
    hello: String,
}

async fn func(event: LambdaEvent<RequestEvent>) -> Result<Value, Error> {
    let response = json!({
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/json"
        },
        "body": serde_json::to_string(&json!({"message": event.payload})).unwrap(),
        "isBase64Encoded": false
    });

    Ok(response)
}
// use lambda_runtime::{service_fn, LambdaEvent, Error};
// use serde_json::{json, Value};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let func = service_fn(func);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }

// async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
//     let (event, _context) = event.into_parts();
//     let first_name = event["firstName"].as_str().unwrap_or("world");

//     Ok(json!({ "message": format!("Hello, {}!", first_name) }))
// }
