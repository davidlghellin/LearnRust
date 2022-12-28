use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub birth_date: NaiveDate,
    pub custom_data: CustomData,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, birth_date_ymd: (i32, u32, u32)) -> Self {
        let (year, month, day) = birth_date_ymd;
        let id = uuid::Uuid::parse_str("af08097e-e5b2-4ee6-a5b3-d6a0aaa72140").unwrap();
        Self {
            id,
            name,
            birth_date: NaiveDate::from_ymd_opt(year, month, day).unwrap(),
            custom_data: CustomData { random: 1 },
            created_at: Some(Utc::now()),
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomData {
    pub random: u32,
}
