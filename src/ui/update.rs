use super::ShinyContext;
pub use serde_json::json as args;

pub fn update_text_input<T>(
    session: &mut ShinyContext<T>,
    input_id: &str,
    args: serde_json::Value)
    where T: actix::Actor<Context = ShinyContext<T>>
{
    let return_msg = serde_json::json!({
        "errors": {},
        "values": {},
        "inputMessages": [{
            "id": input_id,
            "message": args
        }]
    }).to_string();
    session.text(return_msg);
}

pub fn update_numeric_input<T>(
    session: &mut ShinyContext<T>,
    input_id: &str,
    args: serde_json::Value)
    where T: actix::Actor<Context = ShinyContext<T>>
{
    let return_msg = serde_json::json!({
        "errors": {},
        "values": {},
        "inputMessages": [{
            "id": input_id,
            "message": args
        }]
    }).to_string();
    session.text(return_msg);
}
