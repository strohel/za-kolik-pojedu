use anyhow::{bail, Context, Result};
use csv::{ReaderBuilder, Trim};
use dioxus::prelude::*;
use enum_map::{enum_map, Enum, EnumMap};
use jiff::civil::{Time, Weekday};
use regex::{Captures, Regex};
use serde::{de::Error, Deserialize, Deserializer};
use std::{collections::BTreeSet, mem, sync::LazyLock, time::Duration};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use tracing::debug;

const BASIC: &[u8] = include_bytes!("../../provider-data/car4way/basic.tsv");
const ACTIVE: &[u8] = include_bytes!("../../provider-data/car4way/active.tsv");
const BUSINESS: &[u8] = include_bytes!("../../provider-data/car4way/business.tsv");

static TARIFFS: LazyLock<Vec<Tariff>> = LazyLock::new(load_tariffs);

#[derive(Debug, Clone, PartialEq)]
pub struct Car4way {
    tariff: TariffKind,
    car_types: BTreeSet<CarType>,
}

impl Car4way {
    pub fn name(&self) -> &'static str {
        "car4way"
    }
}

impl Default for Car4way {
    fn default() -> Self {
        Self { tariff: TariffKind::default(), car_types: CarType::iter().collect() }
    }
}

#[component]
pub fn Car4wayInput(car4way: Signal<Car4way>) -> Element {
    let name = car4way.read().name();

    let tariff_changed = move |evt: FormEvent| {
        car4way.write().tariff = evt.parsed()?;
        Ok(())
    };

    let mut car_type_changed = move |car_type, evt: FormEvent| {
        if evt.checked() {
            car4way.write().car_types.insert(car_type);
        } else {
            car4way.write().car_types.remove(&car_type);
        }
    };

    rsx! {
        p {
                label { for: "provider-{name}-tariff", "Tarif: " },
                select { id: "provider-{name}-tariff",
                    onchange: tariff_changed,
                    for tariff_kind in TariffKind::iter() {
                        option { value: "{tariff_kind}",
                            selected: car4way.read().tariff == tariff_kind,
                            "{tariff_kind}"
                        }
                    }
                }
        }
        p {
                "Kategorie aut: ",
                for car_type in CarType::iter() {
                    input { id: "provider-{name}-cartype-{car_type}",
                        r#type: "checkbox",
                        checked: car4way.read().car_types.contains(&car_type),
                        onchange: move |evt| car_type_changed(car_type, evt),
                    }
                    label { for: "provider-{name}-cartype-{car_type}", "{car_type} " },
                }
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display, EnumString,
)]
enum TariffKind {
    #[default]
    Basic,
    Active,
    Business,
}

fn load_tariffs() -> Vec<Tariff> {
    [(TariffKind::Basic, BASIC), (TariffKind::Active, ACTIVE), (TariffKind::Business, BUSINESS)]
        .iter()
        .map(|(name, data)| {
            debug!("Loading {name:?}...");
            load_tariff(*name, data)
                .with_context(|| format!("loading {name:?} Car4way tariff"))
                .expect("unit tested, should not fail")
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
struct Tariff {
    kind: TariffKind,
    // NB(Matej): maybe better to transpose this?
    per_cartype: EnumMap<CarType, PerCarTariff>,
    per_km_czk: f64,
    airport_enter_czk: f64,
    airport_leave_czk: f64,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Enum, EnumIter, Display, EnumString,
)]
enum CarType {
    Legend,
    Fancy,
    Boss,
}

#[derive(Debug, Clone, PartialEq)]
struct PerCarTariff {
    per_minute: Vec<PerMinuteTariff>,
    packages: Vec<Package>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PerMinuteTariff {
    start: Time,
    end: Time,
    per_minute_czk: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Package {
    name: String,
    duration: Duration,
    kilometers: f64,
    czk: f64,
    time_limitation: Option<TimeLimitation>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TimeLimitation {
    from: WeekdayTime,
    to: WeekdayTime,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct WeekdayTime {
    weekday: Weekday,
    time: Time,
}

fn load_tariff(kind: TariffKind, data: &[u8]) -> Result<Tariff> {
    // Keep the times and regexes in sync!
    const DAY_START: Time = Time::constant(6, 0, 0, 0);
    const NIGHT_START: Time = Time::constant(20, 0, 0, 0);
    const WEEKEND_START: Time = Time::constant(16, 0, 0, 0);
    const WEEKEND_END: Time = Time::constant(10, 0, 0, 0);

    static DAY_MINUTE_TARIFF_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("Denní: 6:00 - 20:00 Po-Ne").unwrap());
    static NIGHT_MINUTE_TARIFF_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("Noční: 20:00 - 6:00 Po-Ne").unwrap());
    static HOUR_PACKAGE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("([0-9]+) hodiny? \\+ ([0-9]+) km").unwrap());
    static DAY_PACKAGE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("([0-9]+) dn[yí] \\+ ([0-9]+) km").unwrap());

    let mut day_tariff = EnumMap::default();
    let mut night_tariff = EnumMap::default();
    let mut packages: EnumMap<CarType, Vec<Package>> = EnumMap::default();
    let mut per_km_czk = None;
    let mut airport_enter_czk = None;
    let mut airport_leave_czk = None;

    let mut rdr = ReaderBuilder::new().delimiter(b'\t').trim(Trim::All).from_reader(data);
    // For debugging, one can use `for result in rdr.records() {`
    for result in rdr.deserialize() {
        let row: TariffRow = result?;
        debug!("{row:?}");

        if DAY_MINUTE_TARIFF_RE.is_match(&row.item) {
            extract_minute_tariff(&row, &mut day_tariff, DAY_START, NIGHT_START)?;
        } else if NIGHT_MINUTE_TARIFF_RE.is_match(&row.item) {
            extract_minute_tariff(&row, &mut night_tariff, NIGHT_START, DAY_START)?;
        } else if row.item == "Výhodné balíčky" {
            // Pass.
        } else if let Some(matches) = HOUR_PACKAGE_RE.captures(&row.item) {
            extract_package(&row, &mut packages, matches, Duration::from_secs(60 * 60))?;
        } else if let Some(matches) = DAY_PACKAGE_RE.captures(&row.item) {
            extract_package(&row, &mut packages, matches, Duration::from_secs(24 * 60 * 60))?;
        } else if row.item == "Víkend + 200 km" {
            let time_limitation = Some(TimeLimitation {
                from: WeekdayTime { weekday: Weekday::Friday, time: WEEKEND_START },
                to: WeekdayTime { weekday: Weekday::Monday, time: WEEKEND_END },
            });
            extract_package_inner(
                &row,
                &mut packages,
                Duration::from_secs((8 + 24 + 24 + 10) * 60 * 60),
                200.0,
                time_limitation,
            )?;
        } else if row.item == "Km nad rámec balíčků" {
            per_km_czk = Some(row.only().context("expected exactly one value for per km price")?);
        } else if row.item == "Letiště Praha - příjezd" {
            airport_enter_czk =
                Some(row.only().context("expected single value for airport entry")?);
        } else if row.item == "Letiště Praha - výjezd" {
            airport_leave_czk =
                Some(row.only().context("expected single value for airport leave")?);
        } else {
            bail!("The item {:?} doesn't match any pattern.", row.item);
        }
    }

    Ok(Tariff {
        kind,
        per_cartype: enum_map! { car_type => {
            PerCarTariff {
                per_minute: vec![
                    day_tariff[car_type]
                        .with_context(|| format!("no day minute tariff price for {car_type:?}"))?,
                    night_tariff[car_type]
                        .with_context(|| format!("no night minute tariff price for {car_type:?}"))?,
                ],
                packages: mem::take(&mut packages[car_type]),
            }
        }},
        per_km_czk: per_km_czk.context("per km price not parsed")?,
        airport_enter_czk: airport_enter_czk.context("czk to enter airport not parsed")?,
        airport_leave_czk: airport_leave_czk.context("czk to leave airport not parsed")?,
    })
}

fn extract_minute_tariff(
    row: &TariffRow,
    tariff: &mut EnumMap<CarType, Option<PerMinuteTariff>>,
    start: Time,
    end: Time,
) -> Result<()> {
    for (car_type, per_minute_czk) in
        [(CarType::Legend, row.legend), (CarType::Fancy, row.fancy), (CarType::Boss, row.boss)]
    {
        let Some(per_minute_czk) = per_minute_czk else {
            bail!("All columns should have valid price valid for item {}", row.item);
        };
        tariff[car_type] = Some(PerMinuteTariff { start, end, per_minute_czk });
    }

    Ok(())
}

fn extract_package(
    row: &TariffRow,
    packages: &mut EnumMap<CarType, Vec<Package>>,
    matches: Captures,
    duration_unit: Duration,
) -> Result<()> {
    let duration: u32 = matches
        .get(1)
        .expect("has first match")
        .as_str()
        .parse()
        .context("parsing duration as integer")?;
    let duration = duration_unit * duration;

    let kilometers: f64 = matches
        .get(2)
        .expect("has second match")
        .as_str()
        .parse()
        .context("parsing kilometers as float")?;

    extract_package_inner(row, packages, duration, kilometers, None)
}

fn extract_package_inner(
    row: &TariffRow,
    packages: &mut EnumMap<CarType, Vec<Package>>,
    duration: Duration,
    kilometers: f64,
    time_limitation: Option<TimeLimitation>,
) -> Result<()> {
    for (car_type, czk) in
        [(CarType::Legend, row.legend), (CarType::Fancy, row.fancy), (CarType::Boss, row.boss)]
    {
        let Some(czk) = czk else {
            bail!("All columns should have valid price valid for item {}", row.item);
        };
        let package =
            Package { name: row.item.to_string(), duration, kilometers, czk, time_limitation };
        packages[car_type].push(package);
    }

    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct TariffRow {
    #[serde(alias = "Minutový tarif  (km v ceně)")]
    item: String,
    #[serde(alias = "Legend Fabia", deserialize_with = "deserialize_decimal_comma")]
    legend: Option<f64>,
    #[serde(
        alias = "Fancy  Scala, Karoq, Octavia, Caddy Van",
        deserialize_with = "deserialize_decimal_comma"
    )]
    fancy: Option<f64>,
    #[serde(alias = "Boss Superb / Kodiaq", deserialize_with = "deserialize_decimal_comma")]
    boss: Option<f64>,
}

impl TariffRow {
    /// Return the single value from columns or None there is less or more values.
    fn only(&self) -> Option<f64> {
        let values = [self.legend, self.fancy, self.boss];
        let mut non_null_iter = values.iter().filter_map(|x| *x);

        let first_non_null = non_null_iter.next();
        if non_null_iter.next().is_none() {
            first_non_null
        } else {
            None
        }
    }
}

fn deserialize_decimal_comma<'de, D: Deserializer<'de>>(des: D) -> Result<Option<f64>, D::Error> {
    let string = String::deserialize(des)?;

    if string.is_empty() {
        return Ok(None);
    }

    // Replace decimal commas with decimal points.
    let string = string.replace(',', ".");
    // Trim spaces
    let string = string.replace(' ', "");

    string.parse().map_err(D::Error::custom).map(Option::Some)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_load_tariffs() {
        dbg!(load_tariffs());
    }
}
