#![allow(non_snake_case)]

use dioxus::{events::onclick, prelude::*};
use std::{cell::RefCell, future::Future, rc::Rc, sync::Arc};

use github_email::{Authors, CommitAuthor};

// use rsx_platform_free::Editor;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(AppWeb)
}

pub trait UseStatePatch<T> {
    fn set_async(&self, cx: &ScopeState, future: impl Future<Output = T> + 'static);
}

impl<T> UseStatePatch<T> for UseState<T> {
    fn set_async(&self, cx: &ScopeState, future: impl Future<Output = T> + 'static) {
        let my_state = self.clone();
        cx.spawn(async move { my_state.set(future.await) })
    }
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
    let authors = use_state(&cx, || {
        let mut authors = Authors::default();
        authors.insert(CommitAuthor { name: "a".to_string(), email: "bb".to_string(), count: 1 });
        authors.insert(CommitAuthor { name: "b".to_string(), email: "bb".to_string(), count: 2 });
        authors.insert(CommitAuthor { name: "c".to_string(), email: "bb".to_string(), count: 3 });
        authors
    });
    let table = authors_table(authors);
    cx.render(rsx!(
        div {
            // class: "flex",
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
                class: "form-control",
                button {
                    class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                    r#type: "button",
                    // onclick: move |e| authors.set_async(cx, e),
                    "Find Emails"
                }
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
            table
        }
    ))
}

fn authors_table<'b>(authors: &'b Authors) -> LazyNodes<'_, 'b> {
    let title = rsx!(
        tr {
            th {"Name"}
            th {"Email"}
            th {"Count"}
            th {"Ratio"}
        }
    );
    let all = authors.count_commits();
    let rows = authors.into_iter().map(move |item| {
        let name = &item.name;
        let email = &item.email;
        let count = item.count;
        let ratio = percent(count, all);
        rsx!(
            tr {
                td {"{name}"}
                td {"{email}"}
                td {"{count}"}
                td {"{ratio:.2}%"}
            }
        )
    });
    rsx!(
        table {
            class: "flex-1 ml-2 mr-2",
            title
            rows
        }
    )
}

pub struct UseAuthors {
    urls: Rc<RefCell<String>>,
    response: Rc<RefCell<Authors>>,
    updater: Arc<dyn Fn() + 'static>,
}

/// A builder for a [`UseKatex`] hook.
pub fn use_authors(cx: &ScopeState) -> &mut UseAuthors {
    let place_holder = r#"https://github.com/oovm"#;
    let authors = UseAuthors {
        urls: Rc::new(RefCell::new(place_holder.to_string())),
        response: Rc::new(RefCell::new(Default::default())),
        updater: cx.schedule_update(),
    };
    cx.use_hook(|_| authors)
}

fn percent(now: usize, all: usize) -> f64 {
    100.0 * (now as f64) / (all as f64)
}
