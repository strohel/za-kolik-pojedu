use crate::provider::{car4way::Car4way, Provider, ProviderKind};
use dioxus::prelude::*;

pub mod provider;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        MainView {}
    }
}

#[component]
fn Title() -> Element {
    let title = "Za kolik pojedu? 🚗🫰";

    rsx! {
        document::Title { "{title}" },
        div { id: "title",
            h1 { "{title}" }
        }
    }
}

#[component]
fn MainView() -> Element {
    let providers = use_signal(|| -> Vec<Provider> {
        vec![
            Provider::new(ProviderKind::Bolt(Default::default())),
            Provider::new(ProviderKind::Car4way(Car4way::new())),
        ]
    });

    rsx! {
        div { id: "providers",
            h2 { "Poskytovatelé" },
            for provider in providers.read().iter().cloned() {
                ProviderSection { provider },
            }
        }
    }
}

#[component]
fn ProviderSection(provider: Provider) -> Element {
    rsx! {
        div {
            key: provider.name(),
            class: "provider",
            h3 { "{provider.name()}" },
            pre { "{provider:#?}" }
        }
    }
}
