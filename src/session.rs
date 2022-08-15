use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShinyMsg {
    pub method: String,
    pub data: serde_json::Map<String, serde_json::Value>
}

pub struct InputPool {
    pub pool: std::collections::HashMap<String, serde_json::Value>
}

impl InputPool {
    pub fn new() -> InputPool {
        InputPool {
            pool: std::collections::HashMap::<String, serde_json::Value>::new()
        }
    }
    pub fn contains(&self, key: &str) -> bool {
        return self.pool.contains_key(key)
    }
    pub fn insert(&mut self, key: &str, value: serde_json::Value) {
        self.pool.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Result<&serde_json::Value, Box<dyn std::error::Error>> {
        let val: &serde_json::Value =  self.pool.get(key).ok_or("Value not stored in inputs")?;
        return Ok(val)
    }
    pub fn get_string(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let val_res = self.get(key)?
            .as_str()
            .ok_or("Value could not be parsed")?
            .to_string();
        return Ok(val_res)
    }
    pub fn get_u64(&self, key: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let val_res = self.get(key)?;
        let val_u64 = val_res.as_u64().ok_or("Value could not be converted to u64")?;
        return Ok(val_u64)
    }
    pub fn get_f64(&self, key: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let val_res = self.get(key)?;
        let val_f64 = val_res.as_f64().ok_or("Value could not be converted to f64")?;
        return Ok(val_f64)
    }
    pub fn get_i64(&self, key: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let val_res = self.get(key)?;
        let val_i64 = val_res.as_i64().ok_or("Value could not be converted to i64")?;
        return Ok(val_i64)
    }
}

pub fn check_change(session: &mut ShinySession, msg: &ShinyMsg, key: &str) -> bool {
    let v: &serde_json::Value = &msg.data[key];
    let prev_val = session.input.get(key).unwrap_or(&serde_json::Value::Null);
    if v.is_null() & prev_val.is_null() {
        return false
    }
    if prev_val.is_null() | (v != prev_val) {
        session.input.insert(&key, v.clone());
        return true
    }
    return false
}


pub fn generate_id() -> String {
    let notification_id = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();
    return notification_id
}

pub struct ShinySession {
    pub ws: actix_ws::Session,
    pub input: InputPool,
    pub msg_stream: actix_ws::MessageStream,
    pub event: String
}

impl ShinySession {
    pub fn new(session: actix_ws::Session, msg_stream: actix_ws::MessageStream) -> ShinySession {
        ShinySession {
            ws: session,
            input: InputPool::new(),
            msg_stream: msg_stream,
            event: "init".to_string()
        }
    }
}

#[macro_export]
macro_rules! bind_event {
    (session: $session:expr, event:($( $event:expr ),*), expr: $expr:expr) => {
        {
            let mut temp_vec: Vec<String> = vec!();
            $(
                temp_vec.push($event.to_string());
            )*
            if temp_vec.contains(&$session.event) {
                $expr;
            }
        }
    }
}

#[macro_export]
macro_rules! shiny_server {
    ($session:expr, init:($( $init:expr ),*), update: ($( $update:expr ),*), tick: ($( $tick:expr ),*)) => {
        {
            let mut interval = tokio::time::interval(HEARTBEAT_INTERVAL);
            let mut last_heartbeat = std::time::Instant::now();
            loop {
                // create "next client timeout check" future
                let tick = interval.tick();
                // required for select()
                tokio::pin!(tick);

                // waits for either `msg_stream` to receive a message from the client or the heartbeat
                // interval timer to tick, yielding the value of whichever one is ready first
                match futures_util::future::select($session.msg_stream.next(), tick).await {
                    // received message from WebSocket client
                    futures_util::future::Either::Left((Some(Ok(msg)), _)) => {
                        log::debug!("msg: {msg:?}");
                        match msg {
                            actix_ws::Message::Text(text) => {
                                let msg: ShinyMsg = serde_json::from_str(&text).expect("Invalid websocket message");
                                match msg.method.as_str() {
                                    "init" => {
                                        let initial_inputs = msg.data.keys();
                                        for key in initial_inputs {
                                            $session.input.insert(key, msg.data[key].clone());
                                        }
                                        $(
                                            $init;
                                        )*
                                    },
                                    "update" => {
                                        let changed_inputs = msg.data.keys();
                                        for key in changed_inputs {
                                            if shiny_rs::session::check_change(&mut $session, &msg, &key) {
                                                $session.event = key.to_string();
                                                $(
                                                    $update;
                                                )*
                                            }
                                        }
                                    },
                                    _ => {
                                        continue;
                                    }
                                }
                            }

                            actix_ws::Message::Binary(bin) => {
                                $session.ws.binary(bin).await.unwrap();
                            }

                            actix_ws::Message::Close(reason) => {
                                break reason;
                            }

                            actix_ws::Message::Ping(bytes) => {
                                last_heartbeat = std::time::Instant::now();
                                let _ = $session.ws.pong(&bytes).await;
                            }

                            actix_ws::Message::Pong(_) => {
                                last_heartbeat = std::time::Instant::now();
                            }

                            actix_ws::Message::Continuation(_) => {
                                log::warn!("no support for continuation frames");
                            }

                            // no-op; ignore
                            actix_ws::Message::Nop => {}
                        };
                    }

                    // client WebSocket stream error
                    futures_util::future::Either::Left((Some(Err(err)), _)) => {
                        log::error!("{}", err);
                        break None;
                    }

                    // client WebSocket stream ended
                    futures_util::future::Either::Left((None, _)) => break None,

                    // heartbeat interval ticked
                    futures_util::future::Either::Right((_inst, _)) => {
                        // if no heartbeat ping/pong received recently, close the connection
                        if std::time::Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                            log::info!(
                                "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                            );

                            break None;
                        }

                        $(
                            $tick;
                        )*

                        // send heartbeat ping
                        let _ = $session.ws.ping(b"").await;
                    }
                }
            }
        }
    }
}
