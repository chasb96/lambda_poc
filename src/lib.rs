use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub async fn run<S: Clone, T: Router<S>>(context: S) -> Result<(), Error> {
    let context_ref = &context;

    lambda_runtime::run(
        service_fn(
            move |event| async move {
                handle::<S, T>(context_ref.clone(), event).await
            }
        )
    ).await
}

async fn handle<S, T: Router<S>>(context: S, event: LambdaEvent<Value>) -> Result<Value, Error> {
    let router: T = serde_json::from_value(event.payload).unwrap();

    Ok(router.handle(context))
}

pub trait Router<T>: DeserializeOwned {
    fn handle(&self, context: T) -> Value;
}
