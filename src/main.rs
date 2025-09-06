use crate::provider::{
    Provider, ProviderKind,
    bolt::Bolt,
    car4way::{Car4way, Car4wayInput},
};
use dioxus::prelude::*;
use jiff::{RoundMode, ToSpan, Unit, Zoned, ZonedRound, civil::DateTime};
use tracing::debug;

pub mod provider;

type FormEvent = Event<FormData>;

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

    let bolt_enabled = use_signal(|| true);
    let bolt = use_signal(Bolt::default);
    let bolt = Provider::new(bolt_enabled, ProviderKind::Bolt(bolt));

    let car4way_enabled = use_signal(|| true);
    let car4way = use_signal(Car4way::default);
    let car4way = Provider::new(car4way_enabled, ProviderKind::Car4way(car4way));

    let providers = [bolt, car4way];

    rsx! {
        TripInput { input_data },
        div { id: "providers", class: "top-section",
            h2 { "Poskytovatelé" },
            for provider in providers {
                ProviderSection { provider, input_data },
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TripInputData {
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

    let km_changed = move |evt: FormEvent| {
        input_data.write().km = evt.parsed()?;
        Ok(())
    };
    let begin_changed = move |evt: FormEvent| {
        input_data.write().begin = evt.parsed()?;
        Ok(())
    };
    let end_changed = move |evt: FormEvent| {
        input_data.write().end = evt.parsed()?;
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
fn ProviderSection(provider: Provider, input_data: Signal<TripInputData>) -> Element {
    let name = provider.name();
    debug!("ProviderSection for {name} rendering...");

    let enabled_changed = move |evt: FormEvent| {
        provider.enabled.set(evt.parsed()?);
        Ok(())
    };

    // TODO(Matej): does this need a memo or something like that?
    let result = provider.calculate(input_data);

    rsx! {
        div {
            key: name,
            class: "provider",
            h3 { "{name}" },
            p {
                label { for: "provider-{name}-enabled", "Využít " },
                input { id: "provider-{name}-enabled",
                    r#type: "checkbox",
                    checked: provider.enabled,
                    onchange: enabled_changed,
                }
            }
            match provider.kind {
                ProviderKind::Bolt(_bolt) => rsx!("TODO Bolt"),
                ProviderKind::Car4way(car4way) => rsx! { Car4wayInput { car4way } },
            }
            p {
                "Result: {result}"
            }
            pre { "{provider:#?}" }
        }
    }
}
