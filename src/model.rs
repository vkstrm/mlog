use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Artist {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Release {
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub artist: String,
    #[serde(rename = "year")]
    pub release_year: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    #[serde(with = "date_serializer")]
    pub date: String,
    pub release: String,
    pub artist: String,
}

mod date_serializer {
    use std::str::FromStr;

    use chrono::{DateTime, Local};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt: DateTime<Local> = DateTime::from_str(date).unwrap();
        let s = format!("{}", dt.format("%Y-%m-%d"));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        unimplemented!()
    }
}
