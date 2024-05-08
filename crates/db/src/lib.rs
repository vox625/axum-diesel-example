pub mod models;
pub mod schema;
pub use deadpool::managed::PoolError;
pub use deadpool_diesel::{self, Error as DieselError, InteractError};
pub use diesel;

use deadpool_diesel::{postgres::Pool, Manager, Runtime};

pub fn create_pool(database_url: &str) -> Pool {
    let manager = Manager::new(database_url, Runtime::Tokio1);
    Pool::builder(manager).build().unwrap()
}
