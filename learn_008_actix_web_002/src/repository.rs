use std::sync::{PoisonError, RwLock};

use chrono::Utc;
use uuid::Uuid;

use crate::user::User;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("PoissonError: `{0}`")]
    LockError(String),
    #[error("This entity already exits")]
    AlreadyExits,
    #[error("This entity not exits")]
    DoesNotExits,
    #[error("The id format is not valid")]
    IvalidId,
}

pub trait Repository: Send + Sync + 'static {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, RepositoryError>;
    fn create_user(&self, user_id: &User) -> Result<User, RepositoryError>;
    /*
    fn create_user_mut(&mut self, user_id: &User) -> Result<User, String>;
    */
    fn update_user(&self, user_id: &User) -> Result<User, RepositoryError>;
    fn delete_user(&self, user_id: &uuid::Uuid) -> Result<Uuid, RepositoryError>;
}

impl<T> From<PoisonError<T>> for RepositoryError {
    fn from(poisson_error: PoisonError<T>) -> Self {
        RepositoryError::LockError(poisson_error.to_string())
    }
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
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, RepositoryError> {
        let users = self.users.read()?;

        users
            .iter()
            .find(|u| &u.id == user_id)
            .cloned()
            .ok_or_else(|| RepositoryError::IvalidId)
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
    fn create_user(&self, user: &User) -> Result<User, RepositoryError> {
        if self.get_user(&user.id).is_ok() {
            return Err(RepositoryError::AlreadyExits);
        }
        let mut new_user = user.to_owned();
        new_user.created_at = Some(Utc::now());

        let mut users = self.users.write()?;
        users.push(new_user.clone());

        //self.users.write().unwrap().push(new_user.clone());
        Ok(new_user)
    }

    fn update_user(&self, user_id: &User) -> Result<User, RepositoryError> {
        if self.get_user(&user_id.id).is_err() {
            return Err(RepositoryError::DoesNotExits);
        }

        let mut updated_user = user_id.to_owned();
        updated_user.updated_at = Some(Utc::now());

        let mut users = self.users.write()?;
        users.retain(|x| x.id != user_id.id);
        users.push(updated_user.clone());

        Ok(updated_user)
    }

    fn delete_user(&self, user_id: &uuid::Uuid) -> Result<Uuid, RepositoryError> {
        let mut users = self.users.write()?;
        users.retain(|x| &x.id != user_id);
        Ok(user_id.to_owned())
    }
}
