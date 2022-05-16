use chrono::Utc;
use rocket::{serde::Serialize, http::Method, Request, request::{FromRequest, self}};

/// Stores data about a HTTP request
#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct RequestData {
    pub id: usize,
    pub timestamp: i64,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub uri: String,
    pub body: String,
}

impl RequestData {
    pub fn new(id: usize, req: &Request, data: Option<&[u8]>) -> Self {
        let body = if let Some(data) = data {
            String::from_utf8_lossy(data).into_owned()
        } else {
            String::new()
        };
        RequestData {
            id,
            timestamp: Utc::now().timestamp(),
            headers: req.headers().iter().map(|h| (h.name.into_string(), h.value.into_owned())).collect(),
            uri: req.uri().to_string(),
            method: req.method(),
            body,
        }
    }
}

/// Implements a request guard to build a request data object in Rocket
#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestData {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(RequestData::new(1, req, None))
    }
}