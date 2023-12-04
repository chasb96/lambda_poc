use std::marker::PhantomData;
use lambda_runtime::LambdaEvent;
use serde::de::{Visitor, self};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use crate::Handler;
use crate::invoke_context::InvokeContext;

pub struct POCMirror<S, R, H> 
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R>,
{
    path: String,
    content: Value,
    _s: PhantomData<S>,
    _r: PhantomData<R>,
    _h: PhantomData<H>,
}

impl<S, R, H> POCMirror<S, R, H>
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R>,
{
    pub fn new(path: String, content: Value) -> Self {
        Self {
            path,
            content,
            _s: PhantomData::default(),
            _r: PhantomData::default(),
            _h: PhantomData::default(),
        }
    }   
}

type MirrorResult<R> = Result<R, StatusError>;

#[derive(Serialize)]
pub struct StatusError {
    status_code: u16,
    message: String
}

impl<S, R, H> POCMirror<S, R, H>
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R>,
{
    fn into_mirrored(self) -> Result<H, StatusError> {
        let action = H::get_action(self.path)
            .ok_or(
                StatusError {
                    status_code: 404,
                    message: "Not Found".to_string()
                }
            )?;

        let reserialized = json!({
            "action": action,
            "content": self.content,
        });

        serde_json::from_value(reserialized)
            .map_err(|_|
                StatusError { 
                    status_code: 400, 
                    message: "Bad Request".to_string() 
                }
            )
    }
}

impl<S, R, H> Handler<S, MirrorResult<R>> for POCMirror<S, R, H> 
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R> + From<POCMirror<S, R, H>>,
{
    fn get_action(_: String) -> Option<String> {
        // This never gets called, as the downcast above calls it instead
        unreachable!()
    }

    async fn handle(context: crate::invoke_context::InvokeContext<S, MirrorResult<R>, Self>) -> MirrorResult<R> {
        let event = LambdaEvent {
            context: context
                .event
                .context,
            payload: context
                .event
                .payload
                .into_mirrored()?
        };

        let unwrapped_context = InvokeContext::new(context.state, event);

        let response = H::handle(unwrapped_context).await;

        Ok(response)
    }
}

const FIELDS: &'static [&'static str] = &["path", "content"];

enum Field {
    Path,
    Content,
}

struct FieldVisitor;

impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("`path` or `content`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, 
    {
        match value {
            "path" => Ok(Field::Path),
            "content" => Ok(Field::Content),
            _ => Err(de::Error::unknown_field(value, FIELDS))
        }
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

struct POCVisitor<S, R, H> {
    _s: PhantomData<S>,
    _r: PhantomData<R>,
    _h: PhantomData<H>,
}

impl<S, R, H> POCVisitor<S, R, H> 
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R> + From<POCMirror<S, R, H>>,
{
    pub fn new() -> Self {
        Self {
            _s: PhantomData::default(),
            _r: PhantomData::default(),
            _h: PhantomData::default(),
        }
    }
}

impl<'de, S, R, H> Visitor<'de> for POCVisitor<S, R, H> 
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R> + From<POCMirror<S, R, H>>,
{
    type Value = POCMirror<S, R, H>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct POCMirror")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>, 
    {
        let mut path = None;
        let mut content = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::Path => {
                    if path.is_some() {
                        return Err(de::Error::duplicate_field("path"));
                    }

                    path = Some(map.next_value()?)
                }
                Field::Content => {
                    if content.is_some() {
                        return Err(de::Error::duplicate_field("content"));
                    }

                    content = Some(map.next_value()?)
                },
            }
        }

        let path = path.ok_or_else(|| de::Error::missing_field("path"))?;
        let content = content.ok_or_else(|| de::Error::missing_field("content"))?;

        Ok(POCMirror::new(path, content))
    }
}

impl<'de, S, R, H> Deserialize<'de> for POCMirror<S, R, H>
where
    S: Clone,
    R: Serialize,
    H: Handler<S, R> + From<POCMirror<S, R, H>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        deserializer.deserialize_struct("POCMirror", FIELDS, POCVisitor::new())
    }
}