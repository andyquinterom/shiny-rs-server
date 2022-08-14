pub async fn show_notification(session: &mut super::session::ShinySession, message: serde_json::Value) {
    let notification_msg = serde_json::json!({
        "notification": {
            "type": "show",
            "message": message
        }
    }).to_string();
    session.ws.text(notification_msg).await.unwrap();
}

pub async fn render_plot(session: &mut super::session::ShinySession, output_id: &str, plot: &str) {
    let return_msg = serde_json::json!({
        "values": {
            output_id: {
                "src": plot,
                "height": "100%",
                "style": "object-fit:contain"
            }
        }
    }).to_string();
    session.ws.text(return_msg).await.unwrap();
}

pub async fn render_ui(session: &mut super::session::ShinySession, output_id: &str, html: &str) {
    let return_msg = serde_json::json!({
        "values": {
            output_id: {
                "html": html,
                "deps": [],
            },

        }
    }).to_string();
    session.ws.text(return_msg).await.unwrap();
}

pub async fn insert_ui(session: &mut super::session::ShinySession, selector: &str, _where: &str, html: &str) {
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
    session.ws.text(return_msg).await.unwrap();
}

pub async fn remove_ui(session: &mut super::session::ShinySession, selector: &str) {
    let return_msg = serde_json::json!({
        "shiny-remove-ui": {
            "selector": selector,
            "multiple": false
        }
    }).to_string();
    session.ws.text(return_msg).await.unwrap();
}

