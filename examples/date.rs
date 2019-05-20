use chrono::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum TimeUnit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
}

impl TimeUnit {
    pub fn format_pattern(&self) -> &'static str {
        use TimeUnit::*;
        match self {
            Nanosecond => "%Y-%m-%dT%H:%M:%S.%.2f%z",
            Microsecond => "%Y-%m-%dT%H:%M:%S.%.2f%z",
            Millisecond => "%Y-%m-%dT%H:%M:%S.%.2f%z",
            Second => "%Y-%m-%dT%H:%M:%S%z",
            Minute => "%Y-%m-%dT%H:%M%z",
            Hour => "%Y-%m-%dT%H%z",
            Day => "%Y-%m-%d%z",
        }
    }
}

pub fn get_iso_time_string(datetime: DateTime<Local>, tu: TimeUnit) -> impl std::fmt::Display {
    datetime.format(tu.format_pattern())
}

fn main() {
    let local = Local::now();
    println!("Local: {}", local);
    println!("Local: {}", local.to_rfc3339());
    let encoded = bincode::serialize(&local).unwrap();
    println!("encoded({}): {:?}", encoded.len(), encoded);

    println!();
    println!("{}", get_iso_time_string(local, TimeUnit::Second));
    println!("{}", get_iso_time_string(local, TimeUnit::Hour));
    println!("{}", get_iso_time_string(local, TimeUnit::Day));
}
