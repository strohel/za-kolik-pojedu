use crate::provider::{car4way::Car4way, Provider, ProviderKind};
use dioxus::prelude::*;
use time::{Duration, OffsetDateTime, PrimitiveDateTime, UtcDateTime, UtcOffset};
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
    let input_data = use_signal(|| TripInputData::new().expect("can construct TripInputData"));

    let providers = use_signal(|| -> Vec<Provider> {
        vec![
            Provider::new(ProviderKind::Bolt(Default::default())),
            Provider::new(ProviderKind::Car4way(Car4way::new())),
        ]
    });

    rsx! {
        TripInput { input_data },
        div { id: "providers", class: "top-section",
            h2 { "Poskytovatelé" },
            for provider in providers.read().iter().cloned() {
                ProviderSection { provider },
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TripInputData {
    km: f64,
    begin: PrimitiveDateTime,
    end: PrimitiveDateTime,
}

impl TripInputData {
    fn new() -> Result<Self, RenderError> {
        const FIVE_MINUTES: i64 = 5 * 60;

        let now = UtcDateTime::now().unix_timestamp();

        // Round to next five minutes
        let in_five_mins = ((now / FIVE_MINUTES) + 1) * FIVE_MINUTES;
        // Convert to OffsetDateTime in local TZ
        let in_five_mins = OffsetDateTime::from_unix_timestamp(in_five_mins)?
            .to_offset(UtcOffset::current_local_offset()?);
        // Convert to PrimitiveDateTime
        let in_five_mins = PrimitiveDateTime::new(in_five_mins.date(), in_five_mins.time());

        Ok(Self { km: 10.0, begin: in_five_mins, end: in_five_mins + Duration::HOUR })
    }
}

#[component]
fn TripInput(input_data: Signal<TripInputData>) -> Element {
    debug!("TripInput rendering...");

    let kilometers_changed = move |evt: Event<FormData>| {
        debug!("Kilometers changed: {evt:?}");
        let value: f64 = evt.value().parse()?;
        input_data.write().km = value;

        Ok(())
    };

    rsx! {
        div { id: "trip", class: "top-section",
            h2 { "Cesta" },
            p {
                label { for: "input-kilometers", "Počet km " },
                input { id: "input-kilometers",
                    r#type: "number",
                    value: input_data().km,
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
