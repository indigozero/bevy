use crate::renderer::RenderResourceBindings;

use super::Camera;
use bevy_ecs::{
    entity::Entity,
    system::{Query, ResMut},
};
use bevy_utils::HashMap;

#[derive(Debug, Default)]
pub struct ActiveCamera {
    pub entity: Option<Entity>,
    pub bindings: RenderResourceBindings,
}

#[derive(Debug, Default)]
pub struct ActiveCameras {
    cameras: HashMap<String, ActiveCamera>,
}

impl ActiveCameras {
    pub fn add(&mut self, name: &str) {
        self.cameras
            .insert(name.to_string(), ActiveCamera::default());
    }

    pub fn get(&self, name: &str) -> Option<&ActiveCamera> {
        self.cameras.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut ActiveCamera> {
        self.cameras.get_mut(name)
    }
}

pub fn active_cameras_system(
    mut active_cameras: ResMut<ActiveCameras>,
    query: Query<(Entity, &Camera)>,
) {
    for (name, active_camera) in active_cameras.cameras.iter_mut() {
        if active_camera.entity.is_none() {
            for (camera_entity, camera) in query.iter() {
                if let Some(ref current_name) = camera.name {
                    if current_name == name {
                        active_camera.entity = Some(camera_entity);
                    }
                }
            }
        }
    }
}
