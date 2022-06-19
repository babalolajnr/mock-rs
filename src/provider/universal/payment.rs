use crate::{provider::base::BaseTrait, PaymentTrait};

pub struct Payment;
impl PaymentTrait<'_> for Payment {}
impl BaseTrait for Payment {}
impl Payment {
    
}
