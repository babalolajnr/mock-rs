use crate::{provider::base::BaseTrait, MiscellaneousTrait, PaymentTrait};

pub struct Payment;
impl PaymentTrait<'_> for Payment {}
impl BaseTrait for Payment {}
impl MiscellaneousTrait for Payment {}
impl Payment {}
    