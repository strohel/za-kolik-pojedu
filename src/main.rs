use crate::provider::{car4way::Car4way, Provider, ProviderKind};
use dioxus::prelude::*;
use jiff::{civil::DateTime, RoundMode, ToSpan, Unit, Zoned, ZonedRound};
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

    let bolt = use_signal(|| Provider::new(ProviderKind::Bolt(Default::default())));
    let car4way = use_signal(|| Provider::new(ProviderKind::Car4way(Car4way::new())));
    let providers = [bolt, car4way];

    rsx! {
        TripInput { input_data },
        div { id: "providers", class: "top-section",
            h2 { "Poskytovatelé" },
            for provider in providers {
                ProviderSection { provider },
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TripInputData {
    km: f64,
    begin: DateTime,
    end: DateTime,
}

impl TripInputData {
    fn new() -> Result<Self, RenderError> {
        let in_five_mins = Zoned::now()
            .round(ZonedRound::new().smallest(Unit::Minute).mode(RoundMode::Ceil).increment(5))?;
        let hour_later = &in_five_mins + 1.hour();

        Ok(Self { km: 10.0, begin: in_five_mins.datetime(), end: hour_later.datetime() })
    }
}

#[component]
fn TripInput(input_data: Signal<TripInputData>) -> Element {
    debug!("TripInput rendering, input_data: {:?}.", input_data);

    let km_changed = move |evt: Event<FormData>| {
        input_data.write().km = evt.value().parse()?;
        Ok(())
    };
    let begin_changed = move |evt: Event<FormData>| {
        input_data.write().begin = evt.value().parse()?;
        Ok(())
    };
    let end_changed = move |evt: Event<FormData>| {
        input_data.write().end = evt.value().parse()?;
        Ok(())
    };

    let total_time = input_data.with(|input_data| input_data.end - input_data.begin);

    rsx! {
        div { id: "trip", class: "top-section",
            h2 { "Cesta" },
            p {
                label { for: "input-kilometers", "Počet km " },
                input { id: "input-kilometers",
                    r#type: "number",
                    value: input_data.read().km,
                    onchange: km_changed,
                    min: 0,
                },
            },
            p {
                label { for: "input-begin-time", "Začátek " },
                input { id: "input-begin-time",
                    r#type: "datetime-local",
                    value: input_data.read().begin.to_string(),
                    onchange: begin_changed,
                },
            },
            p {
                label { for: "input-end-time", "Konec " },
                input { id: "input-end-time",
                    r#type: "datetime-local",
                    value: input_data.read().end.to_string(),
                    onchange: end_changed,
                },
            },
            p {
                "Celkový čas: {total_time:#}"
            }
        },
    }
}

#[component]
fn ProviderSection(provider: Signal<Provider>) -> Element {
    let name = provider.read().name();
    debug!("ProviderSection for {name} rendering...");

    let enabled_changed = move |evt: Event<FormData>| {
        provider.write().enabled = evt.value().parse()?;
        Ok(())
    };

    rsx! {
        div {
            key: name,
            class: "provider",
            h3 { "{name}" },
            p {
                label { for: "provider-{name}-enabled", "Využít " },
                input { id: "provider-{name}-enabled",
                    r#type: "checkbox",
                    checked: provider.read().enabled,
                    onchange: enabled_changed,
                }
            }
            pre { "{provider:#?}" }
        }
    }
}
