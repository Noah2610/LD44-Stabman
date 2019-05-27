use deathframe::components::solid::SolidTag as SolidTagTrait;

#[derive(Clone)]
pub enum SolidTag {
    Default,
    Player,
    Enemy,
}

impl SolidTagTrait for SolidTag {
    fn collides_with(&self, other: &SolidTag) -> bool {
        match (self, other) {
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
