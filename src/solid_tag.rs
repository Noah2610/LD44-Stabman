use deathframe::components::solid::SolidTag as SolidTagTrait;

#[derive(Clone)]
pub enum SolidTag {
    Default,
    Player,
    Enemy,
    Noclip,
}

impl SolidTagTrait for SolidTag {
    fn collides_with(&self, other: &SolidTag) -> bool {
        match (self, other) {
            (SolidTag::Noclip, _) | (_, SolidTag::Noclip) => false,
            (SolidTag::Default, _) | (_, SolidTag::Default) => true,
            _ => false,
        }
    }
}

impl Default for SolidTag {
    fn default() -> Self {
        SolidTag::Default
    }
}
