use bevy_ecs::prelude::*;

use crate::systems::prelude::*;

pub struct MainScheduler {
    scheduler: Schedule,
}

impl Default for MainScheduler {
    fn default() -> Self {
        let mut scheduler = Schedule::default();
        scheduler.add_stage(
            "update_stage",
            SystemStage::single_threaded(),
        );

        scheduler.add_system_to_stage("update_stage", scene_entity_position_update_system);
        scheduler.add_system_to_stage("update_stage", player_entity_update_system);

        MainScheduler {
            scheduler,
        }
    }
}

impl MainScheduler {
    pub fn run(&mut self, world: &mut World) {
        self.scheduler.run(world);
    }
}
