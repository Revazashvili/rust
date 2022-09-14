use crate::models::Customer;

use super::Storage;

pub struct InMemoryStorage {
    customers: Vec<Customer>,
}

impl Storage for InMemoryStorage {
    fn init() -> Self {
        InMemoryStorage {
            customers: Vec::new(),
        }
    }

    fn list(&self) -> Option<Vec<Customer>> {
        if self.customers.is_empty() {
            None
        } else {
            Some(self.customers.clone())
        }
    }

    fn get(&self, guid: String) -> Option<Customer> {
        for customer in &self.customers {
            if customer.guid == guid {
                return Some(customer.clone());
            }
        }
        None
    }

    fn create(&mut self, customer: Customer) -> bool {
        let _ = self.customers.push(customer);
        true
    }

    fn update(&mut self, guid: String, customer: Customer) -> bool {
        for c in self.customers.iter_mut() {
            if c.guid == guid {
                *c = customer;
                return true;
            }
        }
        false
    }

    fn delete(&mut self, guid: String) {
        self.customers.retain(|customer| customer.guid != guid)
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::Customer, storage::Storage};

    use super::InMemoryStorage;

    #[test]
    fn list_return_none_when_empty() {
        let storage = InMemoryStorage::init();
        let list = storage.list();
        assert_eq!(true, list.is_none())
    }

    #[test]
    fn list_return_some_when_not_empty() {
        let mut storage = InMemoryStorage::init();
        storage.customers.push(Customer {
            guid: String::from("1234"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            email: String::from("johndoe@gmail.com"),
            address: String::from("address"),
        });
        let list = storage.list();
        assert_eq!(true, list.is_some());
        assert_eq!(1, list.unwrap().len())
    }

    #[test]
    fn get_return_non_when_empty() {
        let storage = InMemoryStorage::init();
        let customer = storage.get(String::from("1234"));
        assert_eq!(true, customer.is_none());
    }

    #[test]
    fn get_return_some_when_not_empty() {
        let mut storage = InMemoryStorage::init();
        storage.customers.push(Customer {
            guid: String::from("1234"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            email: String::from("johndoe@gmail.com"),
            address: String::from("address"),
        });
        let customer = storage.get(String::from("1234"));
        assert_eq!(true, customer.is_some());
    }

    #[test]
    fn get_return_none_when_passed_empty_guid() {
        let mut storage = InMemoryStorage::init();
        storage.customers.push(Customer {
            guid: String::from("1234"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            email: String::from("johndoe@gmail.com"),
            address: String::from("address"),
        });
        let customer = storage.get(String::from(""));
        assert_eq!(true, customer.is_none());
    }

    #[test]
    fn creates_customer() {
        let mut storage = InMemoryStorage::init();
        storage.customers.push(Customer {
            guid: String::from("1234"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            email: String::from("johndoe@gmail.com"),
            address: String::from("address"),
        });
    }

    #[test]
    fn update_customer_returns_true_when_exists() {
        let mut storage = InMemoryStorage::init();
        storage.customers.push(Customer {
            guid: String::from("1234"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            email: String::from("johndoe@gmail.com"),
            address: String::from("address"),
        });

        let updated = storage.update(
            String::from("1234"),
            Customer {
                guid: String::from("1234"),
                first_name: String::from("Updated John"),
                last_name: String::from("Updated Doe"),
                email: String::from("updatedjohndoe@gmail.com"),
                address: String::from("Updated address"),
            },
        );

        assert_eq!(true, updated)
    }

    #[test]
    fn update_customer_returns_false_when_not_exists() {
        let mut storage = InMemoryStorage::init();
        let updated = storage.update(
            String::from("1234"),
            Customer {
                guid: String::from("1234"),
                first_name: String::from("Updated John"),
                last_name: String::from("Updated Doe"),
                email: String::from("updatedjohndoe@gmail.com"),
                address: String::from("Updated address"),
            },
        );

        assert_eq!(false, updated)
    }

    #[test]
    fn deletes_customer_when_exists() {
        let mut storage = InMemoryStorage::init();
        storage.customers.push(Customer {
            guid: String::from("1234"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            email: String::from("johndoe@gmail.com"),
            address: String::from("address"),
        });
        storage.delete(String::from("1234"));
        assert_eq!(0, storage.customers.len())
    }

    #[test]
    fn not_deletes_customer_when_not_exists() {
        let mut storage = InMemoryStorage::init();
        storage.customers.push(Customer {
            guid: String::from("1234"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            email: String::from("johndoe@gmail.com"),
            address: String::from("address"),
        });
        storage.delete(String::from("12345"));
        assert_eq!(1, storage.customers.len())
    }
}
