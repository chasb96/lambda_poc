# Do not use this

This is just a proof of concept to see if things could work this way, it is likely ineffectual, but it is neat to see it work.

Leverages serde deserialization magic to deserialize straight to the handler, as an alternative method of routing.

Define your actions:
```rust
#[derive(Deserialize)]
#[serde(tag = "action", content = "content")]
enum MyActions {
    ActionA(AIn),
    ActionB(BIn),
}

#[derive(Serialize)]
enum MyOutputs {
    ActionA(AOut),
    ActionB(BOut),
}

#[derive(Deserialize)]
struct AIn {
    message: String,
}

#[derive(Serialize)]
struct AOut {
    message: String,
}

#[derive(Deserialize)]
struct BIn {
    different_message: String,
}

#[derive(Serialize)]
struct BOut {
    different_message: String,
}
```

Define whatever state needed, implementing Clone:
```rust
#[derive(Clone)]
struct MyState {
    state_message: String,
}
```

Implement the handler, supplying how to get from route -> action
```rust
impl Handler<MyState, MyOutputs> for MyActions
{
    fn get_action(route: String) -> Option<String> {
        // Go from route to a corresponding enum.
        //
        // This is included to accomidate any sort of routing. If invoking this as just
        // JSON, you can just return Some(route) here. 
        Some(route)
    }

    async fn handle(context: InvokeContext<MyState, MyOutputs, Self>) -> MyOutputs {
        // Here context.event.payload will match the type according to get_action
        match context.event.payload {
            ActionA(ain) -> MyOutputs::ActionA(AOut { message: ain.message }),
            ActionB(bin) -> MyOutputs::ActionB(BOut { different_message: bin.different_message }),
        }
    }
}
```

This can then be invoked on lambda with either of the following:
```json
{
    "action": "ActionA",
    "content": {
        "mesage": "Hello World"
    }
}
// or
{
    "action": "ActionB",
    "content": {
        "different_mesage": "Hello World 2"
    }
}
```