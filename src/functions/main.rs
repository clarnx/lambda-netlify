use lambda_runtime::{handler_fn, Context, Error};
use serde_json::Value;

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
