#[derive(Debug, Clone)]
pub struct UiHand {
    pub cards: Vec<UiCard>,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UiCard {
    pub rank: &'static str,
    pub suit: &'static str,
}
