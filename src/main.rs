use dioxus::document::eval;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

// Bindings to the JavaScript functions in index.html
#[wasm_bindgen]
extern "C" {
    fn initializeTwilio(token: &str);
    fn makeCall(to: &str);
    fn hangUp();
}

fn main() {
    launch(app);
}

pub fn app() -> Element {
    let mut phone_number = use_signal(|| "".to_string());
    let mut status = use_signal(|| "Initializing...".to_string());

    // Effect: Fetch token from Tauri and Init Twilio
    use_effect(move || {
        spawn(async move {
            // Using Dioxus 0.7 eval to call Tauri 2.0 Invoke
            let tauri_eval = eval(r#"
                if (window.__TAURI__) {
                    try {
                        const token = await window.__TAURI__.core.invoke("get_token");
                        window.initializeTwilio(token);
                        return { success: true };
                    } catch (e) {
                        return { success: false, error: e.toString() };
                    }
                } else {
                    return { success: false, error: "Global Tauri not enabled" };
                }
            "#);

            if let Ok(res) = tauri_eval.await {
                if res["success"].as_bool().unwrap_or(false) {
                    status.set("Ready".to_string());
                } else {
                    status.set(res["error"].as_str().unwrap_or("Error").to_string());
                }
            }
        });
    });

    rsx! {
        style { {include_str!("../style.css")} }
        main {
            h1 { "Twilio Dialer" }
            div { class: "status-badge", "{status}" }

            input {
                placeholder: "+",
                value: "{phone_number}",
                oninput: move |e| phone_number.set(e.value())
            }

            div { class: "controls",
                button { class: "btn-call", onclick: move |_| makeCall(&phone_number()), "Call" }
                button { class: "btn-hangup", onclick: move |_| hangUp(), "Hangup" }
            }
        }
    }
}