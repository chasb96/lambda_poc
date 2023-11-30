use enum_assoc::Assoc;
use lambda_runtime::Error;
use lambda_web_framework::Router;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let context = MyContext { };

    lambda_web_framework::run::<MyContext, MyRouter>(context).await
}

#[derive(Clone)]
pub struct MyContext {

}

#[derive(Deserialize)]
#[derive(Assoc)]
#[func(pub fn handle<T>(&self, context: T) -> Value { serde_json::to_value(_0.handle(context)).unwrap() })]
pub enum Actions {
    Save(SaveAction),
    Get(GetAction),
}

#[derive(Deserialize)]
#[serde(tag = "action")]
pub struct MyRouter {
    #[allow(dead_code)]
    action: String,
    handles: Actions,
}

impl<T> Router<T> for MyRouter {
    fn handle(&self, context: T) -> Value {
        self.handles.handle(context)
    }
}

#[derive(Deserialize)]
pub struct SaveAction {
    s: String,
}

#[derive(Serialize)]
pub struct SaveResult {
    s: String,
}

impl SaveAction {
    fn handle<T>(&self, _: T) -> SaveResult {
        SaveResult { s: self.s.to_string() }
    }
}

#[derive(Deserialize)]
pub struct GetAction {
    #[allow(dead_code)]
    id: String,
}

#[derive(Serialize)]
pub struct GetResult {
    
}

impl GetAction {
    fn handle<T>(&self, _: T) -> GetResult {
        GetResult {}
    }
}