use bevy::prelude::*;

const OFF_SCREEN_BOUNDS: f32 = 50.0;

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_objects);
    }
}

#[derive(Component)]
pub struct AnimatedObject {
    pub rotation: Vec3, 
    pub scale: Vec3,
    pub translation: Vec3,
    pub speed: f32,
}

fn animate_objects(
    mut commands: Commands,
    mut animated_objects: Query<(Entity, &mut Transform, &AnimatedObject), With<AnimatedObject>>,   
    time: Res<Time>,
) {
    for (entity, mut transform, animated_object) in animated_objects.iter_mut() {
        //checks if object is outside the screen
        if transform.translation.x > OFF_SCREEN_BOUNDS || transform.translation.x < -OFF_SCREEN_BOUNDS || transform.translation.y > OFF_SCREEN_BOUNDS || transform.translation.y < -OFF_SCREEN_BOUNDS  {
            commands.entity(entity).despawn_recursive();
        }
        //animates the object
        transform.rotate(Quat::from_rotation_y( animated_object.rotation.y * time.delta_secs() * animated_object.speed));
        transform.rotate(Quat::from_rotation_x( animated_object.rotation.x * time.delta_secs() * animated_object.speed));
        transform.rotate(Quat::from_rotation_z( animated_object.rotation.z * time.delta_secs() * animated_object.speed));
        transform.scale = transform.scale + animated_object.scale * time.delta_secs() * animated_object.speed;
        transform.translation = transform.translation + animated_object.translation * time.delta_secs() * animated_object.speed;
    }
}
