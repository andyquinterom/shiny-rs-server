use rand::Rng;

//use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use shiny_rs_derive::ShinyHandler;

pub mod traits;
use traits::*;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

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
    tick: fn(&mut Self, session: &mut <Self as Actor>::Context)
}

pub trait ShinyLogic {
    fn input(&mut self) -> &mut input_pool::InputPool;
    fn get_last_hb(&self) -> Instant;
    fn hb(&self, session: &mut ws::WebsocketContext<Self>) where Self: actix::Actor<Context = ws::WebsocketContext<Self>> {
        session.run_interval(HEARTBEAT_INTERVAL, |act, session| {
            if Instant::now().duration_since(act.get_last_hb()) > CLIENT_TIMEOUT {
                println!("Shiny client heartbeat fialed, disconnecting!");
                session.stop();
                return;
            }
            session.ping(b"");
        });
    }

    fn check_change(&mut self, msg: &ShinyMsg, key: &str) -> bool {
        let v: &serde_json::Value = &msg.data[key];
        let prev_val = self.input().get(key).unwrap_or(&serde_json::Value::Null);
        if v.is_null() & prev_val.is_null() {
            return false
        }
        if prev_val.is_null() | (v != prev_val) {
            self.input().insert(&key, v.clone());
            return true
        }
        return false
    }
}

impl ShinyLogic for ShinyServer {
    fn input(&mut self) -> &mut input_pool::InputPool{
        return &mut self.input;
    }
    fn get_last_hb(&self) -> Instant {
        return self.hb
    }
}

impl ShinyServer {
    pub fn new(
        initialize: fn(&mut Self, session: &mut <Self as Actor>::Context),
        update: fn(&mut Self, session: &mut <Self as Actor>::Context),
        tick: fn(&mut Self, session: &mut <Self as Actor>::Context)
    ) -> Self {
        Self {
            hb: Instant::now(),
            input: input_pool::InputPool::new(),
            event: String::from("init"),
            initialize,
            update,
            tick
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

