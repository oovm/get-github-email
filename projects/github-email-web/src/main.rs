#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc, sync::Arc};

use dioxus::{
    events::{FormEvent, MouseEvent},
    prelude::*,
};
use log::error;

use github_email::Authors;

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
    let place_holder = "/{user}\nhttps://github.com/{user}\nhttps://github.com/{user}/{repo}";
    let github_issue = "https://github.com/oovm/get-github-email/issues";
    let authors = use_authors(&cx);

    let text = authors.get_text();
    let authors_table = authors.view();

    let on_input = {
        let handler = authors.clone();
        move |e: FormEvent| handler.set_text(&e.value)
    };
    let on_click = {
        let handler = authors.clone();
        let future = use_future(&cx, (), |_| async move { handler.click_query().await });
        move |_: MouseEvent| future.restart()
    };
    cx.render(rsx!(
        div {
            // class: "flex",
            div {
                class: "form-control flex-1",
                textarea {
                    class: "textarea h-96 textarea-bordered textarea-primary",
                    id: "editor",
                    placeholder: "{place_holder}",
                    oninput: on_input,
                    value: "{text}",
                }
            }
            div {
                class: "form-control",
                button {
                    class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                    r#type: "button",
                    onclick: on_click,
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
            authors_table
        }
    ))
}

#[derive(Clone)]
pub struct UseAuthors {
    urls: Rc<RefCell<String>>,
    authors: Rc<RefCell<Authors>>,
    updater: Arc<dyn Fn() + 'static>,
}

/// A builder for a [`UseKatex`] hook.
pub fn use_authors(cx: &ScopeState) -> &mut UseAuthors {
    let authors = UseAuthors {
        urls: Rc::new(RefCell::new(Default::default())),
        authors: Rc::new(RefCell::new(Default::default())),
        updater: cx.schedule_update(),
    };
    cx.use_hook(|_| authors)
}

impl UseAuthors {
    /// Notify the scheduler to re-render the component.
    pub fn needs_update(&self) {
        (self.updater)();
    }
    pub fn get_text(&self) -> String {
        self.urls.borrow().to_string()
    }
    pub fn set_text<S>(&self, s: S)
    where
        S: Into<String>,
    {
        *self.urls.borrow_mut() = s.into();
        self.needs_update()
    }
    pub async fn click_query(&self) {
        let urls = self.urls.borrow();
        let errors = self.authors.borrow_mut().query_many(urls.as_str()).await;
        for e in errors {
            error!("{e:?}")
        }
        self.needs_update()
    }
    pub fn view(&self) -> LazyNodes {
        let title = rsx!(
            tr {
                th {"Name"}
                th {"Email"}
                th {"Count"}
                th {"Ratio"}
            }
        );
        let authors = self.authors.borrow().items();
        let all = authors.iter().map(|v| v.count).sum();
        let rows = authors.into_iter().map(move |item| {
            let name = item.name;
            let email = item.email;
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
}

fn percent(now: usize, all: usize) -> f64 {
    100.0 * (now as f64) / (all as f64)
}
