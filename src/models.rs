// Defining tbe Database Model

// helps in working with dates and times.
use chrono::prelude::*;
// These traits are used for serialization and deserialization of data
use serde::{Deserialze,Serialize};

// Traits are the collection of methods defined for unknown type self
// the rust compiler provides basic implementation for some traits which can accessed via #[derive]
#[derive(Debug,Deserialze,Serialize,Clone,Copy,PartialEq,sqlx::Type)]
// This line defines the column name of the database to be user_role and the data will all be in lower case
#[sqlx(type_name="user_role",rename_all="lowercase")]
pub enum UserRole{
    Admin,
    Moderator,
    User
}

/*
Difference between the String and the &str is that String is mutable and has ownership for the data and it will be stored in heap
where &str is immutablle as it is pointing to the memory location as reference
*/
impl UserRole {
    pub fn to_str(&self) -> &str{
        match self {
            UserRole::Admin => "admin",
            UserRole::Moderator => "moderator",
            UserRole::User => "user"
        }
    }
}

/*
sqlx::Type This trait allows you to define how a custom Rust struct or type can be mapped to a database column or data type
sqlx::FromRow This trait allows you to automatically convert a row retrieved from a database query into a Rust struct instance
*/
#[derive(Serialize,Deserialze,Clone,Debug,sqlx::Type,sqlx::FromRow)]
pub struct User{
    // here the name field id will be storing the data using the Universally Unique Identifier using uuid crate in rust
    pub id : uuid::Uuid,
    pub name : String,
    pub email : String,
    pub password : String,
    pub role : UserRole,
    pub photo : String,
    pub verified : bool,
    // These annotations are used with the serde trait for serialization and deserialization. 
    // They specify how the field names in the User struct should be mapped to JSON field names during data exchange
    #[serde(rename = "createdAt")]
    // Stores the creation time of the user record as an optional DateTime<Utc> from the chrono library.
    // The Option type allows for the possibility of a missing creation time
    pub created_at : Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at : Option<DateTime<Utc>>
}

