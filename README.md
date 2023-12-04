# Do not use this

This is just a proof of concept to see if things could work this way, it is likely ineffectual, but it is neat to see it work.

Leverages serde deserialization magic to deserialize straight to the handler, as an alternative method of routing.

Define your actions:
```rust
#[derive(Deserialize)]
#[serde(tag = "action", content = "content")]
#[derive(Assoc)]
enum MyActions {
    ActionA(AIn),
    ActionB(BIn),
}

#[derive(Serialize)]
enum MyOutputs {
    ActionA(AOut),
    ActionB(BOut),
}


impl MyActions {
    fn do_thing(actions: MyActions) -> MuOutputs {
        match actions {
            ActionA(a) => MyOutputs::ActionA(a.do_thing(a)),
            ActionB(b) => MyOutputs::ActionB(b.do_thing(b)),
        }
    }
}

#[derive(Deserialize)]
struct AIn {
    message: String,
}

#[derive(Serialize)]
struct AOut {
    message: String,
}

impl AIn {
    fn do_thing(ain: AIn) -> AOut {
        AOut { message: ain.message }
    }
}

#[derive(Deserialize)]
struct BIn {
    different_message: String,
}

#[derive(Serialize)]
struct BOut {
    different_message: String,
}

impl BIn {
    fn do_thing(bin: BIn) -> BOut {
        AOut { different_message: bin.different_message }
    }
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
        MyActions::do_thing(context.event.payload)
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