#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CampaignType {
    Normal,
    BonusA,
    BonusB,
}

impl Default for CampaignType {
    fn default() -> Self {
        CampaignType::Normal
    }
}
