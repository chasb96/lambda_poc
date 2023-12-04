use std::marker::PhantomData;
use lambda_runtime::LambdaEvent;
use serde::Serialize;

use crate::Handler;

pub struct InvokeContext<S, R, H> 
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R>,
{
    pub state: S,
    pub event: LambdaEvent<H>,
    _r: PhantomData<R>,
}

impl<S, R, H> InvokeContext<S, R, H> 
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R>,
{
    pub fn new(state: S, event: LambdaEvent<H>) -> Self {
        Self {
            state,
            event,
            _r: PhantomData::default(),
        }
    }
}