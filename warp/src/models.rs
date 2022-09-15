use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, PartialEq, ToSchema, Serialize, Deserialize)]
pub struct Customer {
    /// A unique identifier for this customer.
    pub guid: String,
    /// First name
    pub first_name: String,
    /// Last name
    pub last_name: String,
    /// Email address
    pub email: String,
    /// Physical Address
    pub address: String,
}
