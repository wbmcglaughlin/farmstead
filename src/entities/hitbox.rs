use bevy::prelude::*;

#[derive(Component)]
pub struct HitBox {
    pub width: f32,
    pub height: f32,
}

impl HitBox {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Component)]
pub struct HitBoxSprite;

pub fn collision_aabb(translation: Vec3, hitbox: &HitBox, point: Vec2) -> bool {
    if translation.x - hitbox.width / 2.0 <= point.x
        && translation.x + hitbox.width / 2.0 >= point.x
        && translation.y - hitbox.height / 2.0 <= point.y
        && translation.y + hitbox.height / 2.0 >= point.y
    {
        return true;
    }
    false
}

pub fn toggle_hitbox(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    hitbox_query: Query<(&HitBox, &Transform, Entity, &Children)>,
    mut hitbox_sprite_query: Query<&mut Visibility, With<HitBoxSprite>>,
) {
    if !keyboard_input.just_pressed(KeyCode::T) {
        return;
    }

    for (hitbox, transform, entity, children) in hitbox_query.iter() {
        let mut contains_child = false;
        for child in children.iter() {
            if let Ok(mut visibility) = hitbox_sprite_query.get_mut(*child) {
                contains_child = true;
                *visibility = match *visibility {
                    Visibility::Inherited => todo!(),
                    Visibility::Hidden => Visibility::Visible,
                    Visibility::Visible => Visibility::Hidden,
                };
            }
        }
        if !contains_child {
            let highlight = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 0.5, 1.0, 0.2),
                            custom_size: Some(Vec2::new(hitbox.width, hitbox.height)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    HitBoxSprite,
                ))
                .id();

            commands.entity(entity).add_child(highlight);
        }
    }
}
