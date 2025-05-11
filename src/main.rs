use crate::provider::{car4way::Car4way, Provider, ProviderKind};
use dioxus::prelude::*;
use tracing::debug;

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
        div { id: "title", class: "top-section",
            h1 { "{title}" }
        }
    }
}

#[component]
fn MainView() -> Element {
    debug!("MainView rendering...");
    let providers = use_signal(|| -> Vec<Provider> {
        vec![
            Provider::new(ProviderKind::Bolt(Default::default())),
            Provider::new(ProviderKind::Car4way(Car4way::new())),
        ]
    });

    rsx! {
        TripInput {},
        div { id: "providers", class: "top-section",
            h2 { "Poskytovatelé" },
            for provider in providers.read().iter().cloned() {
                ProviderSection { provider },
            }
        }
    }
}

#[component]
fn TripInput() -> Element {
    debug!("TripInput rendering...");
    let mut input_kilometers = use_signal(|| 0);

    let kilometers_changed = move |evt: Event<FormData>| {
        debug!("Kilometers changed: {evt:?}");
        let value: i32 = evt.value().parse()?;
        input_kilometers.set(value);

        Ok(())
    };

    rsx! {
        div { id: "trip", class: "top-section",
            h2 { "Cesta" },
            p {
                label { for: "input-kilometers", "Počet km " },
                input { id: "input-kilometers",
                    r#type: "number",
                    value: input_kilometers,
                    onchange: kilometers_changed,
                    min: 0,
                },
            }
            p {
                label { for: "input-begin-time", "Začátek " },
                input { id: "input-begin-time", r#type: "datetime-local", }
            }
            p {
                label { for: "input-end-time", "Konec " },
                input { id: "input-end-time", r#type: "datetime-local", }
            }
        },
    }
}

#[component]
fn ProviderSection(provider: Provider) -> Element {
    debug!("ProviderSection rendering...");
    rsx! {
        div {
            key: provider.name(),
            class: "provider",
            h3 { "{provider.name()}" },
            pre { "{provider:#?}" }
        }
    }
}
