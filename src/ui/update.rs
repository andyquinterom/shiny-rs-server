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

pub fn select_options(options: Vec<(String, String)>) -> String {
    let mut html_options = String::default();
    for (name, value) in options {
        html_options.push_str(&format!(r#"<option value="{}">{}</option>"#, value, name));
    }
    html_options
}

pub fn update_select_input<T>(
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
