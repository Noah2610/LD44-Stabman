mod savefile;

use amethyst::ecs::{Join, ReadStorage, WriteStorage};

use super::super::state_prelude::*;
use super::level_loader::LevelLoader;

pub mod prelude {
    pub use super::LevelManager;
}

pub struct LevelManager {
    pub settings:          SettingsLevelManager,
    pub level_index:       usize,
    player_checkpoint_opt: Option<Player>,
}

impl LevelManager {
    pub fn new(settings: SettingsLevelManager) -> Self {
        let mut level_manager = Self {
            settings,
            level_index: 0,
            player_checkpoint_opt: None,
        };
        level_manager.load_from_file();
        level_manager
    }

    pub fn load_current_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        data.world.delete_all(); // Remove _ALL_ existing entities first
        let current_level_name = self
            .settings
            .level_names
            .get(self.level_index)
            .expect(&format!(
                "Level at index {} does not exist",
                self.level_index
            ));
        let level_filepath = resource(format!(
            "{}/{}",
            self.settings.levels_dir, current_level_name
        ));

        let mut level_loader = LevelLoader::new(self.settings.clone());
        level_loader.load_level(level_filepath);
        level_loader.build(data);

        // Load checkpoint / player data
        if let Some(player_checkpoint) = &self.player_checkpoint_opt {
            data.world.exec(|mut players: WriteStorage<Player>| {
                if let Some(player) = (&mut players).join().next() {
                    *player = player_checkpoint.clone();
                }
            });
        }
    }

    pub fn set_player_checkpoint(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        self.player_checkpoint_opt =
            data.world.exec(|players: ReadStorage<Player>| {
                (&players).join().next().map(Clone::clone)
            });
    }

    pub fn load_next_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
        should_save: bool,
    ) {
        if self.has_next_level() {
            self.level_index += 1;
            if should_save {
                // Save to savefile after changing the level_index but before loading the next level.
                self.save_to_savefile();
            }
            self.load_current_level(data);
        } else {
            panic!("There is no next level");
        }
    }

    pub fn has_next_level(&self) -> bool {
        self.level_index + 1 < self.settings.level_names.len()
    }

    pub fn restart_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        self.load_current_level(data);
    }

    pub fn save_to_savefile(&self) {
        let savefile_path = self.savefile_path();

        if let Some(player) = &self.player_checkpoint_opt {
            let savefile_data = savefile::SavefileData {
                player:      player.clone(),
                level_index: self.level_index,
            };

            match serde_json::to_string(&savefile_data) {
                Ok(serialized) => {
                    write_file(savefile_path, serialized).unwrap()
                }
                Err(err) => eprintln!(
                    "Couldn't save savefile data to file, an error occured \
                     while serializing save data:\n{:#?}",
                    err
                ),
            }
        }
    }

    fn load_from_file(&mut self) {
        let savefile_path = self.savefile_path();
        if let Ok(json_raw) = read_file(savefile_path) {
            match serde_json::from_str::<savefile::SavefileData>(&json_raw) {
                Ok(deserialized) => {
                    self.player_checkpoint_opt = Some(deserialized.player);
                    self.level_index = deserialized.level_index;
                }
                Err(err) => eprintln!(
                    "Couldn't load savefile data from file, an error occured \
                     while deserializing save data:\n{:#?}",
                    err
                ),
            }
        }
    }

    fn savefile_path(&self) -> String {
        use amethyst::utils::application_root_dir;
        format!("{}/{}", application_root_dir(), self.settings.savefile_path)
    }
}
