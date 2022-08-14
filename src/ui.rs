pub async fn show_notification(session: &mut super::session::ShinySession, message: serde_json::Value) {
    let notification_msg = serde_json::json!({
        "notification": {
            "type": "show",
            "message": message
        }
    }).to_string();
    session.ws.text(notification_msg).await.unwrap();
}

pub async fn render_plot(session: &mut super::session::ShinySession, output_id: String, plot: String) {
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

pub async fn render_ui(session: &mut super::session::ShinySession, output_id: String, html: String) {
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
