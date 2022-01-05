#![allow(non_snake_case)]

use dioxus::prelude::*;
// use rsx_platform_free::Editor;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(AppWeb)
}

pub fn main_ssr() {
    let mut vdom = VirtualDom::new(AppWeb);
    let _ = vdom.rebuild();
    println!("{}", dioxus::ssr::render_vdom(&vdom));
}

pub fn AppWeb(cx: Scope) -> Element {
    let place_holder = r#"https://github.com/oovm"#;
    let github_issue = "https://github.com/oovm/get-github-email/issues";
    let text = use_state(&cx, || place_holder.to_string());
    cx.render(rsx!(
        div {
            class: "flex flex-column",
            div {
                class: "form-control flex-1",
                textarea {
                    class: "textarea h-96 textarea-bordered textarea-primary",
                    id: "editor",
                    placeholder: "{place_holder}",
                    oninput: move |e| text.set(e.value.to_owned()),
                    value: "{text}",
                }
            }
            div {
                class: "flex-1 ml-2 mr-2",
                "math"
            }
        }
        div {
            class: "form-control",
            a {
                href: "{github_issue}",
                target: "_blank",
                button {
                    class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                    r#type: "button",
                    "Report bug on github"
                }
            }
        }
    ))
}
