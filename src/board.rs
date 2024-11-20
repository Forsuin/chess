use bevy::prelude::*;

pub fn setup_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mesh = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));
    let white_material = materials.add(Color::srgb(1.0, 0.9, 0.9));
    let black_material = materials.add(Color::srgb(0.0, 0.1, 0.1));

    // build board
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                }
                else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                ..default()
            });
        }
    }
}