use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
#[derive(Debug, Clone)]
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
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            fecha_nacimiento: NaiveDate::from_ymd_opt(year, month, day).unwrap(),
            custom_data: CustomData { random: 12 },
            creado_el: Some(Utc::now()),
            actualizado_el: None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomData {
    pub random: u32,
}
