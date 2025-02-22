use bevy::prelude::*;

const OFF_SCREEN_BOUNDS: f32 = 60.0;

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
    pub update_rotation: fn(&Transform, &Time) -> Vec3,
    pub update_translation: fn(&Transform, &Time) -> Vec3,
    pub update_scale: fn(&Transform, &Time) -> Vec3,
}

impl Default for AnimatedObject {
    fn default() -> Self {
        AnimatedObject {
            rotation: Vec3::ZERO,
            scale: Vec3::ZERO,
            translation: Vec3::ZERO,
            speed: 1.0,
            update_rotation: |_transform, _time| Vec3::ZERO,
            update_translation: |_transform, _time| Vec3::ZERO,
            update_scale: |_transform, _time| Vec3::ZERO,
        }
    }
}

fn animate_objects(
    mut commands: Commands,
    mut animated_objects: Query<(Entity, &mut Transform, &mut AnimatedObject), With<AnimatedObject>>,   
    time: Res<Time>,
) {
    for (entity, mut transform, mut animated_object) in animated_objects.iter_mut() {
        let t = time.delta_secs();
        // first update the translation
        transform.translation += animated_object.translation * t * animated_object.speed;

        //then check if object is outside the screen
        let is_off_screen = transform.translation.x > OFF_SCREEN_BOUNDS || transform.translation.x < -OFF_SCREEN_BOUNDS || transform.translation.y > OFF_SCREEN_BOUNDS || transform.translation.y < -OFF_SCREEN_BOUNDS;
        if is_off_screen {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        //animates the object
        transform.rotate_y(animated_object.rotation.y * t * animated_object.speed);
        transform.rotate_x(animated_object.rotation.x * t * animated_object.speed);
        transform.rotate_z(animated_object.rotation.z * t * animated_object.speed);

        transform.scale = transform.scale + animated_object.scale * t * animated_object.speed;

        // Store updates in local variables first
        let new_rotation = (animated_object.update_rotation)(&transform, &time);
        let new_translation = (animated_object.update_translation)(&transform, &time);
        let new_scale = (animated_object.update_scale)(&transform, &time);

        // Then apply the updates
        animated_object.rotation += new_rotation;
        animated_object.translation += new_translation;
        animated_object.scale += new_scale;
    }
}
