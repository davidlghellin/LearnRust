use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub fecha_nacimiento: NaiveDate,
    pub custom_data: CustomData,
    pub creado_el: Option<DateTime<Utc>>,
    pub actualizado_el: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, birth_date_ymd: (i32, u32, u32)) -> User {
        let (year, month, day) = birth_date_ymd;
        let ui =uuid::Uuid::parse_str("af08097e-e5b2-4ee6-a5b3-d6a0aaa72140").unwrap();
        Self {
            id: ui, //uuid::Uuid::new_v4(),
            name,
            fecha_nacimiento: NaiveDate::from_ymd_opt(year, month, day).unwrap(),
            custom_data: CustomData { random: 12 },
            creado_el: Some(Utc::now()),
            actualizado_el: None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomData {
    pub random: u32,
}
