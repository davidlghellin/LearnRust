use std::sync::RwLock;

use chrono::Utc;
use uuid::Uuid;

use crate::user::User;

pub trait Repository: Send + Sync + 'static {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String>;
    fn create_user(&self, user_id: &User) -> Result<User, String>;
    /*
    fn create_user_mut(&mut self, user_id: &User) -> Result<User, String>;
    */
    fn update_user(&self, user_id: &User) -> Result<User, String>;
    fn delete_user(&self, user_id: &uuid::Uuid) -> Result<Uuid, String>;
}

pub struct MemoryRepository {
    users: RwLock<Vec<User>>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: RwLock::new(vec![]),
        }
    }
}

impl Repository for MemoryRepository {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String> {
        let users = self.users.read().map_err(|_| "unlock error")?;

        users
            .iter()
            .find(|u| &u.id == user_id)
            .cloned()
            .ok_or_else(|| "Invalid UUID".to_string())
    }
    /*

    fn create_user_mut(&mut self, user: &User) -> Result<User, String> {
        if self.get_user(&user.id).is_ok(){
            return Err(String::from("El usuario ya existe"));
        }
        let mut new_user=user.to_owned();
        new_user.created_at=Some(Utc::now());

        self.users.push(new_user.clone());
        Ok(new_user)
    }
     */
    fn create_user(&self, user: &User) -> Result<User, String> {
        if self.get_user(&user.id).is_ok() {
            return Err(String::from("El usuario ya existe"));
        }
        let mut new_user = user.to_owned();
        new_user.created_at = Some(Utc::now());

        let mut users = self.users.write().map_err(|_| "unlock error")?;
        users.push(new_user.clone());

        //self.users.write().unwrap().push(new_user.clone());
        Ok(new_user)
    }

    fn update_user(&self, user_id: &User) -> Result<User, String> {
        if self.get_user(&user_id.id).is_err() {
            return Err("El usuario no exite".to_string());
        }

        let mut updated_user = user_id.to_owned();
        updated_user.updated_at = Some(Utc::now());

        let mut users = self.users.write().map_err(|_| "unlock error")?;
        users.retain(|x| x.id != user_id.id);
        users.push(updated_user.clone());

        Ok(updated_user)
    }

    fn delete_user(&self, user_id: &uuid::Uuid) -> Result<Uuid, String> {
        let mut users = self.users.write().map_err(|_| "unlock error")?;
        users.retain(|x| &x.id != user_id);
        Ok(user_id.to_owned())
    }
}
