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

    fn list(&self) -> Option<&Vec<Customer>> {
        if self.customers.is_empty() {
            Some(&self.customers)
        } else {
            None
        }
    }

    fn get(&self, guid: String) -> Option<&Customer> {
        for customer in &self.customers {
            if customer.guid == guid {
                return Some(customer);
            }
        }
        None
    }

    fn create(&mut self, customer: Customer) {
        let _ = self.customers.push(customer);
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
