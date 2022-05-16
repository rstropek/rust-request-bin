use std::{sync::{Arc, atomic::{AtomicUsize, Ordering}}, collections::BTreeMap};

use chrono::NaiveDateTime;
use handlebars::handlebars_helper;
use rocket::{tokio::sync::RwLock, fairing::{Fairing, Info, Kind, self}, Rocket, Build, State, response::Redirect, Request, Data};
use rocket_dyn_templates::{Template, context};

use crate::request_data::RequestData;

/// Implements the thread-safe state for the request collector
#[derive(Default, Clone)]
pub struct RequestCollector {
    request_id: Arc<AtomicUsize>,
    request_map: Arc<RwLock<BTreeMap<usize, RequestData>>>,
}

#[rocket::async_trait]
impl Fairing for RequestCollector {
    fn info(&self) -> Info {
        Info {
            name: "Request collector",
            kind: Kind::Ignite | Kind::Request
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        /// Displays a list of all requests
        /// 
        /// All fairing-related requests (i.e. requests with URIs starting with /requests)
        /// are filtered out. The requests are returned in descending order of their timestamp
        /// (i.e. latest request first). Only the latest 10 requests are returned.
        #[get("/requests")]
        async fn requests(requests: &State<RequestCollector>) -> Template {
            let map = requests.request_map.read().await;
            let map: Vec<RequestData> = map.iter()
                .rev()
                .filter(|(_, v)| !v.uri.starts_with("/requests"))
                .take(10)
                .map(|(_, v)| v.clone())
                .collect();
            Template::render("request-list", context! { requests: map, parent: "base".to_string() })
        }

        /// Displays details about a single request
        #[get("/requests/<id>")]
        async fn request_details(id: usize, requests: &State<RequestCollector>) -> Option<Template> {
            let map = requests.request_map.read().await;
            map.get(&id).map(|data| Template::render("request-details", context! { request: data, parent: "base".to_string() }))
        }

        /// Clear all requests
        #[get("/requests/clear")]
        async fn requests_clear(requests: &State<RequestCollector>) -> Redirect {
            let mut map = requests.request_map.write().await;
            map.clear();
            Redirect::to("/requests")
        }

        Ok(rocket.manage(self.clone())
            .mount("/", routes![
                requests, 
                request_details, 
                requests_clear])
            .attach(Template::custom(|engines| {
                // Registers a helper for formatting a timestamp
                handlebars_helper!(format_date: |timestamp: i64| format!("{}", NaiveDateTime::from_timestamp(timestamp, 0)));
                engines.handlebars.register_helper("format_date", Box::new(format_date));
            })))
            
    }

    /// Stores the incoming request
    async fn on_request(&self, req: &mut Request<'_>, data: &mut Data<'_>) {
        let next_id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let mut map = self.request_map.write().await;
        let body_data = data.peek(512).await;
        map.insert(next_id, RequestData::new(next_id, req, Some(body_data)));
    }
}