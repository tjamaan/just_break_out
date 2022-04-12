use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Persistent;

pub(crate) fn destroy_nonpersistent_entities(
    mut commands: Commands,
    query: Query<(Entity, Without<Persistent>)>,
) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
