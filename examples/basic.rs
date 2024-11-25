use bevy::prelude::*;
use bevy_c3d::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(C3dPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, load_c3d)
        .add_systems(Update, markers)
        .init_resource::<State>()
        .run();
}

#[derive(Resource, Default, Debug)]
struct State {
    pub frame: usize,
}

fn load_c3d(
    mut events: EventReader<C3dLoadedEvent>,
    c3d_state: ResMut<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    if let Some(_) = events.read().last() {
        let asset = c3d_assets.get(&c3d_state.handle);
        match asset {
            Some(asset) => {
                for _ in 0..asset.c3d.points.labels.len() {
                    let matrix = Mat4::from_scale_rotation_translation(
                        Vec3::new(1.0, 1.0, 1.0),
                        Quat::from_rotation_y(0.0),
                        Vec3::new(0.0, 0.0, 0.0),
                    );
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(
                                Sphere::new(0.014).mesh(),
                            ),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgb_u8(0, 0, 127),
                                ..default()
                            }),
                            transform: Transform::from_matrix(matrix),
                            ..default()
                        },
                        Marker,
                    ));
                }
            }
            None => {}
        }
    }
}

#[derive(Component)]
struct Marker;

fn markers(
    mut state: ResMut<State>,
    mut query: Query<(&mut Transform, &Marker)>,
    c3d_state: ResMut<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
) {
    let asset = c3d_assets.get(&c3d_state.handle);
    match asset {
        Some(asset) => {
            let point_data = &asset.c3d.points;
            let num_frames = point_data.size().0;
            let mut i = 0;
            for (mut transform, _) in query.iter_mut() {
                transform.translation = Vec3::new(
                    point_data[(state.frame, i)][0] as f32 / 1000.0,
                    point_data[(state.frame, i)][1] as f32 / 1000.0,
                    point_data[(state.frame, i)][2] as f32 / 1000.0,
                );
                i += 1;
            }
            state.frame += 1;
            if state.frame >= num_frames {
                state.frame = 0;
            }
        }
        None => {}
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut c3d_state: ResMut<C3dState>) {
    c3d_state.handle = asset_server.load("walk.c3d");

    // Spawn a light and the camera
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        brightness: 0.3,
        ..Default::default()
    });

    spawn_camera(commands);
}

pub fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(0., -3.5, 1.0);

    commands.spawn((Camera3dBundle {
        camera: Camera {
            clear_color: Color::srgb(0.8, 0.8, 0.8).into(),
            ..Default::default()
        },
        transform: Transform::from_translation(translation)
            .looking_at(Vec3::new(0., 0., 1.), Vec3::Z),
        ..Default::default()
    },));
}
