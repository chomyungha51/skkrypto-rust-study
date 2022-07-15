mod front_of_hospital;

pub use crate::front_of_hospital::receiption;

pub fn open_hospital() {
    receiption::add_customer_to_waitlist();
}