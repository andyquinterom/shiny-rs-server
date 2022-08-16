use super::ShinyContext;

pub fn render_plot<T>(session: &mut ShinyContext<T>, output_id: &str, plot: &str)
    where T: actix::Actor<Context = ShinyContext<T>>
{
    let return_msg = serde_json::json!({
        "values": {
            output_id: {
                "src": plot,
                "height": "100%",
                "style": "object-fit:contain"
            }
        }
    }).to_string();
    session.text(return_msg);
}

pub fn render_ui<T>(session: &mut ShinyContext<T>, output_id: &str, html: &str)
    where T: actix::Actor<Context = ShinyContext<T>>
{
    let return_msg = serde_json::json!({
        "values": {
            output_id: {
                "html": html,
                "deps": [],
            },

        }
    }).to_string();
    session.text(return_msg);
}

pub fn show_notification<T>(session: &mut ShinyContext<T>, message: serde_json::Value)
    where T: actix::Actor<Context = ShinyContext<T>>
{
    let notification_msg = serde_json::json!({
        "notification": {
            "type": "show",
            "message": message
        }
    }).to_string();
    session.text(notification_msg);
}

pub fn insert_ui<T>(session: &mut ShinyContext<T>, selector: &str, _where: &str, html: &str)
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

pub fn remove_ui<T>(session: &mut ShinyContext<T>, selector: &str)
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

