use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use std::borrow::ToOwned;


pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .add_systems(Startup, setup_board);
    }
}


#[derive(Component)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Resource, Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|mat| StandardMaterial {
        base_color: mat.base_color
            .mix(&Color::srgb(0.8, 0.3, 0.3), 1.0),
        ..mat.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|mat| StandardMaterial {
        base_color: mat.base_color
            .mix(&Color::srgb(0.9, 0.1, 0.1), 1.0),
        ..mat.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|mat| StandardMaterial {
        base_color: mat.base_color
            .mix(&Color::srgb(0.9, 0.1, 0.1), 1.0),
        ..mat.to_owned()
    })),
};

fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));

    // build board
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j + 1) % 2 == 0 {
                        materials.add(Color::srgb(1.0, 0.9, 0.9))
                    } else {
                        materials.add(Color::srgb(0.0, 0.1, 0.1))
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    ..default()
                },
                PickableBundle::default(),
                Square {
                    x: i,
                    y: j,
                },
                HIGHLIGHT_TINT,
                On::<Pointer<Select>>::run(select_square),
            ));
        }
    }
}

fn select_square(event: Listener<Pointer<Select>>, mut selected_square: ResMut<SelectedSquare>) {
    selected_square.entity = Some(event.target);
}