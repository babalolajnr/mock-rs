use crate::{MiscellaneousTrait, PaymentTrait};

pub struct Payment;
impl PaymentTrait<'_> for Payment {}
impl MiscellaneousTrait for Payment {}
impl Payment {}
