use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use collisions_aabb::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsAabbPlugin))
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, player_controls)
        .add_systems(Update, update_debug_text)
        .run()
}

#[derive(Component)]
struct Player;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 16.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        DebugText,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
            mesh: meshes.add(Rectangle::new(20.0, 20.0)).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.0, 1.0, 0.0))),

            ..default()
        },
        ActorBody::new(Rectangle::new(20.0, 20.0)),
        Velocity::default(),
        Player,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(300.0, 100.0, 0.0)),
            mesh: meshes.add(Rectangle::new(100.0, 100.0)).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(1.0, 1.0, 0.0))),

            ..default()
        },
        SolidBody::new([Rectangle::new(100.0, 100.0)]),
    ));
}

fn player_controls(
    mut q: Query<(&mut Velocity, &mut ActorBody), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut velocity, mut body) = q.single_mut();

    velocity.0 = Vec2::ZERO;

    const SPEED: f32 = 200.0;

    if input.pressed(KeyCode::KeyA) {
        velocity.0.x -= SPEED;
    }
    if input.pressed(KeyCode::KeyD) {
        velocity.0.x += SPEED;
    }
    if input.pressed(KeyCode::KeyS) {
        velocity.0.y -= SPEED;
    }
    if input.pressed(KeyCode::KeyW) {
        velocity.0.y += SPEED;
    }

    if input.just_pressed(KeyCode::KeyF) {
        body.walk_to(Vec2::new(100.0, 100.0));
    }
}

#[derive(Component)]
pub struct DebugText;

fn update_debug_text(
    player_query: Query<(&Transform, &Velocity), With<Player>>,
    mut text_query: Query<&mut Text, With<DebugText>>,
) {
    let Ok((transform, vel)) = player_query.get_single() else {
        return;
    };

    text_query.single_mut().sections[0].value =
        format!("Pos: {}, vel: {}", transform.translation, vel.0);
}
