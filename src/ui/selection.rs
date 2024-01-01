use bevy::prelude::*;

#[derive(Component)]
pub struct SelectionRectangle {
    pub start: Option<Vec2>,
    pub end: Option<Vec2>,
}

impl SelectionRectangle {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn set_start(&mut self, start: Vec2) {
        self.start = Some(start);
    }

    pub fn set_end(&mut self, end: Vec2) {
        self.end = Some(end);
    }
}

pub fn create_rect_sprite(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.0, 0.2),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        SelectionRectangle::new(),
    ));
}

pub fn adjust_rect_visibility_and_size(
    mut query: Query<(&mut Transform, &mut Visibility, &mut SelectionRectangle)>,
) {
    for (mut transform, mut visible, selection) in query.iter_mut() {
        if let Some(start) = selection.start {
            if let Some(end) = selection.end {
                *visible = Visibility::Visible;
                let size = (end - start).extend(0.0);
                transform.scale = size;
                transform.translation = Vec3::new(start.x, start.y, 2.0) + size / 2.0;
            }
        } else {
            *visible = Visibility::Hidden;
        }
    }
}
