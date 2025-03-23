use dioxus::prelude::*;
use serde::Deserialize;
use tracing::debug;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_signal(String::new);

    let fetch_new = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();

        img_src.set(response.message);
    };

    rsx! {
        div { id: "dogview",
            img { src: img_src }
        }
        div { id: "buttons",
            button { onclick: fetch_new, id: "skip", "skip" }
            // button { onclick: save, id: "save", "save!" }
        }
    }
}

#[derive(Deserialize)]
struct DogApi {
    message: String,
}
