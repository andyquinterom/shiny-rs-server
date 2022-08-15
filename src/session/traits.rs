pub extern crate actix;

pub use actix::prelude::*;
use std::time::{Duration, Instant};
use actix_web_actors::ws;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub trait ShinyLogic {
    fn input(&mut self) -> &mut super::input_pool::InputPool;
    fn get_last_hb(&self) -> Instant;
    fn get_hb_interval(&self) -> Duration;
    fn get_client_timeout(&self) -> Duration;
    fn hb(&self, session: &mut ws::WebsocketContext<Self>) where Self: actix::Actor<Context = ws::WebsocketContext<Self>> {
        let client_timeout = self.get_client_timeout();
        session.run_interval(self.get_hb_interval(), move |act, session| {
            if Instant::now().duration_since(act.get_last_hb()) > client_timeout {
                println!("Shiny client heartbeat fialed, disconnecting!");
                session.stop();
                return;
            }
            session.ping(b"");
        });
    }

    fn check_change(&mut self, msg: &super::ShinyMsg, key: &str) -> bool {
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
