use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Persistent;

pub(crate) fn destroy_nonpersistent_entities(
    mut commands: Commands,
    query: Query<(Entity, Without<Persistent>, Without<Parent>)>,
) {
    for (entity, _, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
