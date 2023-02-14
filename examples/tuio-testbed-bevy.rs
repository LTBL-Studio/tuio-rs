use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, render::{render_resource::{PrimitiveTopology}, mesh::{Indices}}, window::PresentMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "TUIO test bed".to_string(),
                width: 1280.,
                height: 720.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut touch_zone = Mesh::new(PrimitiveTopology::LineStrip);

    let v_pos = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];

    touch_zone.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    let indices = vec![0, 1, 2, 3, 0];
    touch_zone.set_indices(Some(Indices::U32(indices)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(touch_zone)),
        transform: Transform::default().with_scale(Vec3::new(700., 700., 1.)),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    });

    commands.spawn(Camera2dBundle::default());

    // // Rectangle
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.25, 0.25, 0.75),
    //         custom_size: Some(Vec2::new(50.0, 100.0)),
    //         ..default()
    //     },
    //     ..default()
    // });

    // // Circle
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //     transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
    //     ..default()
    // });

    // // Hexagon
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
    //     transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
    //     ..default()
    // });
}