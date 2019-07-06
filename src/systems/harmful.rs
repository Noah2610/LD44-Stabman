use super::system_prelude::*;

pub struct HarmfulSystem;

impl<'a> System<'a> for HarmfulSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {

    }
}
