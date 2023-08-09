#![allow(non_snake_case)]

mod components;
mod models;

use std::{sync::Arc, time::Duration};

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dns::CustomResolver;
use log::LevelFilter;
use reqwest::{Client, ClientBuilder};

use crate::{components::Button, models::ButtonType};

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let test_dns = move |url: String| {
        cx.spawn({
            async move {
                // 创建自定义的 reqwest 客户端
                let client: Client = ClientBuilder::new()
                    .dns_resolver(Arc::new(CustomResolver))
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap();

                // 发起 HTTP 请求
                let response = client.get(url).send().await.unwrap();
                println!("{}", response.status());
            }
        });
    };

    let name = use_state(cx, || "bob".to_string());

    cx.render(rsx! {
        link { rel: "stylesheet", href: "../public/tailwind.css" },

        main {
            class: "container mx-auto",

            div {
                "Hello, world!"
            }

            input {
                class: "border-2",
                // we tell the component what to render
                value: "{name}",
                // and what to do when the value changes
                oninput: move |evt| name.set(evt.value.clone()),
            }

            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| {
                    test_dns("https://www.baidu.com".to_owned());
                },
                "分析"
            }

        }
    })
}
