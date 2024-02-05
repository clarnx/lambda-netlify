use lambda_runtime::{handler_fn, Context, Error};
use serde_json::Value;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(_: Value, _: Context) -> Result<Value, Error> {
    let response = json!({
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/json"
        },
        "body": serde_json::to_string(&json!({"message": "Hello, world!"})).unwrap(),
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