use bevy::prelude::*;

#[derive(Component)]
pub struct EntitySelectionRectangle {
    pub start: Option<Vec2>,
    pub end: Option<Vec2>,
}

impl EntitySelectionRectangle {
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

    pub fn get_area(&self) -> Option<f32> {
        if self.start.is_some() && self.end.is_some() {
            let dim = self.start.unwrap() - self.end.unwrap();

            Some((dim.x * dim.y).abs())
        } else {
            None
        }
    }
}

pub fn create_rect_sprite(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        EntitySelectionRectangle::new(),
    ));
}

pub fn adjust_rect_visibility_and_size(
    mut query: Query<(
        &mut Transform,
        &mut Visibility,
        &mut EntitySelectionRectangle,
    )>,
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
