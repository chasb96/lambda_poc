#![allow(async_fn_in_trait)]

mod invoke_context;
mod poc;

use invoke_context::InvokeContext;
use lambda_runtime::{service_fn, Error};
use serde::{de::DeserializeOwned, Serialize};

pub async fn run<S, R, H>(state: S) -> Result<(), Error> 
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R>,
{
    let state_ref = &state;

    lambda_runtime::run(
        service_fn(
            move |event| async move {
                let invoke_context = InvokeContext::new(state_ref.clone(), event);

                Ok::<R, Error>(H::handle(invoke_context).await)
            }
        )
    ).await
}

pub trait Handler<S, R>: DeserializeOwned
where
    S: Clone,
    R: Serialize,
{
    fn get_action(path: String) -> Option<String>;

    async fn handle(context: InvokeContext<S, R, Self>) -> R;
}