use super::system_prelude::*;

pub struct HeartsSystem {}

impl<'a> System<'a> for HeartsSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, SpriteSheetHandles>,
        WriteStorage<'a, HeartsContainer>,
        WriteStorage<'a, Heart>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
    );

    fn run(
        &mut self,
        (
            entities,
            spritesheet_handles,
            mut hearts_containers,
            mut hearts,
            mut transforms,
            mut sizes,
            mut sprite_renders,
            mut transparents,
        ): Self::SystemData,
    ) {
    }
}
