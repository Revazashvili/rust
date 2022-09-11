use super::models::Customer;

mod in_memory_storage;

/// Type to access Customer storage
pub trait Storage {
    /// Initializes new Storage
    fn init() -> Self;
    /// Returns list of Customer
    ///
    /// Returns list of customer if exists.
    fn list(&self) -> Option<&Vec<Customer>>;
    /// Gets a single customer
    ///
    /// Returns single customer if exists.
    ///
    /// # Arguments
    ///
    /// * guid - `String' -> the id of the customer to retrieve.
    fn get(&self, guid: String) -> Option<&Customer>;
    /// Creates a new customer
    ///
    /// Returns `True` if successfully creates `Customer` or otherwise false.
    /// # Arguments
    ///
    /// * customer - `Customer` type
    fn create(&mut self, customer: Customer);
    /// Updates customers
    ///
    /// Returns `True` if successfully updates `Customer` or otherwise false.
    /// # Arguments
    ///
    /// * guid - `String' -> the id of the customer to update.
    /// * `updated_customer` - `Customer` -> updated customer information
    fn update(&mut self, guid: String, customer: Customer) -> bool;
    /// Deletes a customer
    ///
    /// Returns `True` if successfully deletes `Customer` or otherwise false.
    /// # Arguments
    ///
    /// * guid - `String' -> the id of the customer to delete.
    fn delete(&mut self, guid: String);
}
