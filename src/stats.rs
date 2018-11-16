use std::borrow::Borrow;
use api::{Bool, Collection, Id, OwnerId};
use serde::de;
use std::fmt::Debug;
use std::str::FromStr;
use chrono::offset::local::Local;
pub use chrono::naive::date::NaiveDate;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Period {
    pub day: NaiveDate,
    pub views: u32,
    pub visitors: u32,
    pub reach: u32,
    pub reach_subscribers: u32,
    pub subscribed: u32,
    pub unsubscribed: u32,
    pub sex: Vec<Demography<DemoSex>>,
    pub age: Vec<Demography<DemoAgeRange>>,
    pub sex_age: Vec<Demography<DemoSexAge>>,
    pub cities: Vec<Demography<DemoCity>>,
    pub countries: Vec<Country>,
}

enum_str! { DemoSex {
    Male = "m",
    Female = "f",
}}

enum_str! { DemoAgeRange {
    _12_18 = "12-18",
    _18_21 = "18-21",
    _21_24 = "21-24",
    _24_27 = "24-27",
    _27_30 = "27-30",
    _30_35 = "30-35",
    _35_45 = "35-45",
    _45_100 = "45-100",
}}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DemoSexAge {
    sex: DemoSex,
    age: DemoAgeRange,
}

impl FromStr for DemoSexAge {
    type Err = ();
    fn from_str(s: &str) -> Result<DemoSexAge, ()> {
        let mut split = s.split(";");
        let sex = try!(split.next().unwrap_or("").parse());
        let age = try!(split.next().unwrap_or("").parse());
        Ok(DemoSexAge {
            sex: sex,
            age: age,
        })
    }
}

impl de::Deserialize for DemoSexAge {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<DemoSexAge, D::Error> {
        struct TempVisitor;

        impl de::Visitor for TempVisitor {
            type Value = DemoSexAge;
            fn visit_str<E: de::Error>(&mut self, value: &str) -> Result<DemoSexAge, E> {
                match value.parse() {
                    Ok(temp_value) => Ok(temp_value),
                    _ => Err(de::Error::invalid_value("unexpected value")),
                }
            }
        }

        d.deserialize(TempVisitor)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub enum DemoCity {
    CityId(Id),
    Other,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Demography<T: de::Deserialize + Copy + Debug + Eq> {
    pub visitors: u32,
    pub value: T,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Country {
    pub visitors: u32,
    pub value: Id,
    pub code: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct PostReach {
    pub reach_subscribers: u32,
    pub reach_total: u32,
    pub links: u32,
    pub to_group: u32,
    pub join_group: u32,
    pub report: u32,
    pub hide: u32,
    pub unsubscribe: u32,
}

request! {
    #[derive(Copy, Eq)]
    struct Get for ["stats.get"](v => 5.44) -> Collection<Period> {
        group_id: Option<Id> = () => {Option},
        app_id: Option<Id> = () => {Option},
        date_from: NaiveDate = (Local::today().naive_local()) => {},
        date_to: NaiveDate = (Local::today().succ().naive_local()) => {},
    }
}

request! {
    struct TrackVisitor for ["stats.trackVisitor"](v => 5.44) -> Bool;
}

request! {
    #[derive(Copy, Eq)]
    struct GetPostReach for ["stats.getPostReach"](v => 5.44) -> PostReach {
        owner_id: OwnerId = () => {},
        post_id: Id = () => {},
    }
}
