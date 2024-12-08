use std::sync::Arc;

use serde::Serialize;

pub mod auth_middleware;
pub mod is_owner_middleware;
pub mod path_object_middleware;

pub trait Userable {
    fn get_user_id(&self) -> Arc<i32>;
}

pub trait Findable<T> where T: Serialize {
    fn find_by_id(
        &self,
        id: Arc<i32>
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
}
