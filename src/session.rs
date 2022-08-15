use rand::Rng;

//use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use shiny_rs_derive::ShinyHandler;

pub mod traits;
use traits::*;

pub mod input_pool;

pub type ShinyContext<T> = actix_web_actors::ws::WebsocketContext<T>;
pub type ShinySession = ShinyContext<ShinyServer>;

#[derive(ShinyHandler)]
pub struct ShinyServer {
    hb: Instant,
    pub input: input_pool::InputPool,
    pub event: String,
    initialize: fn(&mut Self, session: &mut <Self as Actor>::Context),
    update: fn(&mut Self, session: &mut <Self as Actor>::Context),
    tick: fn(&mut Self, session: &mut <Self as Actor>::Context),
    hb_interval: Duration,
    client_timeout: Duration
}

impl ShinyServer {
    pub fn new(
        initialize: fn(&mut Self, session: &mut <Self as Actor>::Context),
        update: fn(&mut Self, session: &mut <Self as Actor>::Context),
        tick: fn(&mut Self, session: &mut <Self as Actor>::Context),
        hb_interval: Duration,
        client_timeout: Duration
    ) -> Self {
        Self {
            hb: Instant::now(),
            input: input_pool::InputPool::new(),
            event: String::from("init"),
            initialize,
            update,
            tick,
            hb_interval,
            client_timeout
        }
    }
}

impl Actor for ShinyServer {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, session: &mut Self::Context) {
        self.hb(session);
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShinyMsg {
    pub method: String,
    pub data: serde_json::Map<String, serde_json::Value>
}

pub fn generate_id() -> String {
    let notification_id = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();
    return notification_id
}

#[macro_export]
macro_rules! changed {
    ($shiny:expr, ($( $event:expr ),*)) => {{
        let result: bool;
        {
            let mut temp_vec: Vec<String> = vec!(
                $(
                    $event.to_string(),
                )*
            );
            result = temp_vec.contains(&$shiny.event);
            drop(temp_vec);
        }
        result
    }};
}

