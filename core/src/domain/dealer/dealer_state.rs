use crate::domain::{dealer::DealerId, hand::Hand};

#[derive(Debug)]
pub struct DealerState {
    pub dealer_id: DealerId,
    pub hand: Hand,
}

impl DealerState {
    pub fn new(dealer_id: DealerId) -> Self {
        Self {
            dealer_id,
            hand: Hand::new(),
        }
    }
}
