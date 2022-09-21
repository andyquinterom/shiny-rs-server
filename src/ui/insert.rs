use super::ShinyContext;

pub fn insert_html<T>(session: &mut ShinyContext<T>, selector: &str, _where: &str, html: &str)
    where T: actix::Actor<Context = ShinyContext<T>>
{
    let return_msg = serde_json::json!({
        "shiny-insert-ui": {
            "selector": selector,
            "multiple": false,
            "where": _where,
            "content": {
                "html": html,
                "deps": []
            }
        }
    }).to_string();
    session.text(return_msg);
}

pub fn remove_html<T>(session: &mut ShinyContext<T>, selector: &str)
    where T: actix::Actor<Context = ShinyContext<T>>
{
    let return_msg = serde_json::json!({
        "shiny-remove-ui": {
            "selector": selector,
            "multiple": false
        }
    }).to_string();
    session.text(return_msg);
}

pub fn run_js<T>(session: &mut ShinyContext<T>, code: &str)
    where T: actix::Actor<Context = ShinyContext<T>>
{
    let return_msg = serde_json::json!({
        "javascript": code
    }).to_string();
    session.text(return_msg);
}

