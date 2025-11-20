use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;
fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PhysicsPlugins::default()));
    app.add_systems(Startup, setup);
    app.add_systems(Update, (control_stick_1, control_stick_2));
    app.add_systems(Update, (print_started_collisions, ball_went_past_a_paddle));
    app.add_systems(PostUpdate, reset_ball);
    app.run();
}
const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
fn set_up_score_board(mut commands: Commands){
    commands.spawn((
        Text::new("Score: "),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        Node {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ),(
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        )],
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BallSpawnConfig {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
    });
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));
    commands.spawn(Camera2d);
    let shape = Rectangle::new(20.0, 100.0);
    let mesh = meshes.add(shape);
    let color = Color::Srgba(Srgba::rgb(0.5, 0.5, 0.5));
    let material = materials.add(color);
    commands.spawn(PlayingStickBundle {
        mesh: Mesh2d(mesh.clone()),
        material: MeshMaterial2d(material.clone()),
        transform: Transform::from_xyz(250.0, 0.0, 0.0),
        player_id: Player1,
        rigid_body: RigidBody::Kinematic,
        collider: Collider::rectangle(20.0, 100.0),
        game_item_type: GameItemType(GameItem::RightPaddle),
    });
    commands.spawn(PlayingStick2Bundle {
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material.clone()),
        transform: Transform::from_xyz(-250.0, 0.0, 0.0),
        player_id: Player2,
        rigid_body: RigidBody::Kinematic,
        collider: Collider::rectangle(20.0, 100.0),
        game_item_type: GameItemType(GameItem::LeftPaddle),
    });
    let shape = Rectangle::new(800.0, 10.0);
    let mesh = meshes.add(shape);
    commands.spawn(BoundaryBundle {
        mesh: Mesh2d(mesh.clone()),
        material: MeshMaterial2d(material.clone()),
        transform: Transform::from_xyz(0.0, -250.0, 0.0),
        rigid_body: RigidBody::Static,
        collider: Collider::rectangle(800.0, 10.0),
        boundary_marker: Boundary,
        game_item_type: GameItemType(GameItem::LowerBoundary),
    });

    commands.spawn(BoundaryBundle {
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material.clone()),
        transform: Transform::from_xyz(0.0, 250.0, 0.0),
        rigid_body: RigidBody::Static,
        collider: Collider::rectangle(800.0, 10.0),
        boundary_marker: Boundary,
        game_item_type: GameItemType(GameItem::UpperBoundary),
    });

    // let shape = Rectangle::new(10.0, 500.0);
    // let mesh = meshes.add(shape);
    // commands.spawn(BoundaryBundle {
    //     mesh: Mesh2d(mesh.clone()),
    //     material: MeshMaterial2d(material.clone()),
    //     transform: Transform::from_xyz( -270.0,0.0, 0.0),
    //     rigid_body: RigidBody::Static,
    //     collider: Collider::rectangle(10.0, 500.0),
    //     boundary_marker: Boundary,
    //     game_item_type: GameItemType(GameItem::LowerBoundary),
    // });

    commands.spawn(WallBundle {
        transform: Transform::from_xyz(400.0, 0.0, 0.0),
        rigid_body: RigidBody::Static,
        collider: Collider::rectangle(10.0, 500.0),
        game_item_type: GameItemType(GameItem::RightWall),
        sensor: Sensor,
        wall_marker: Wall,
    });

    commands.spawn(WallBundle {
        transform: Transform::from_xyz(-400.0, 0.0, 0.0),
        rigid_body: RigidBody::Static,
        collider: Collider::rectangle(10.0, 500.0),
        game_item_type: GameItemType(GameItem::LeftWall),
        sensor: Sensor,
        wall_marker: Wall,
    });

    let shape = Circle::new(20.0);
    let mesh = meshes.add(shape);
    let color = Color::Srgba(Srgba::rgb(1.0, 0.647, 0.0));
    let material = materials.add(color);
    commands.insert_resource(BallMeshAndMaterial {
        mesh: mesh.clone(),
        material: material.clone(),
    });
    commands.spawn(BallBundle {
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ball_id: Ball,
        rigid_body: RigidBody::Dynamic,
        collider: Collider::circle(20.0),
        velocity: LinearVelocity(Vec2::new(500.0, 0.0)),
        gravity: GravityScale(0.0),
        bounciness: Restitution::new(1.0),
        events_enabled: CollisionEventsEnabled,
    });
}
fn reset_ball(
    time: Res<Time>,
    mut commands: Commands,
    mut ball_spawn_config: ResMut<BallSpawnConfig>,
    query: Query<(), With<Ball>>,
    ball_material_and_mesh: Res<BallMeshAndMaterial>,
) {
    match query.single() {
        Ok(_) => {}
        Err(_) => {
            ball_spawn_config.timer.tick(time.delta());
            if ball_spawn_config.timer.is_finished() {
                commands.spawn(BallBundle {
                    mesh: Mesh2d(ball_material_and_mesh.mesh.clone()),
                    material: MeshMaterial2d(ball_material_and_mesh.material.clone()),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ball_id: Ball,
                    rigid_body: RigidBody::Dynamic,
                    collider: Collider::circle(20.0),
                    velocity: LinearVelocity(Vec2::new(500.0, 0.0)),
                    gravity: GravityScale(0.0),
                    bounciness: Restitution::new(1.0),
                    events_enabled: CollisionEventsEnabled,
                });
            }
        }
    }
}

fn control_stick_1(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player1>>,
) {
    let mut player1 = query.single_mut().unwrap();

    if keys.pressed(KeyCode::ArrowUp) {
        player1.translation.y += 5.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        player1.translation.y -= 5.0;
    }
}

fn control_stick_2(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player2>>,
) {
    let mut player2 = query.single_mut().unwrap();

    if keys.pressed(KeyCode::KeyW) {
        player2.translation.y += 5.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        player2.translation.y -= 5.0;
    }
}
fn print_started_collisions(
    mut collision_reader: MessageReader<CollisionStart>,
    mut ball_query: Query<&mut LinearVelocity, With<Ball>>,
    query: Query<&GameItemType, Without<Wall>>,
    mut commands: Commands,
    sound: Res<CollisionSound>,
) {
    for event in collision_reader.read() {
        let mut rng = rand::rng();
        let collider1 = event.collider1;
        if let Ok(mut linear_velocity) = ball_query.get_mut(collider1) {
            let collider2 = event.collider2;
            if let Ok(game_item_type) = query.get(collider2) {
                commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));

                match game_item_type.0 {
                    GameItem::RightPaddle => {
                        linear_velocity.y = rng.random_range(-200.0..200.0);
                        linear_velocity.x = -200.0;
                    }
                    GameItem::LeftPaddle => {
                        linear_velocity.y = rng.random_range(-200.0..200.0);
                        linear_velocity.x = 200.0;
                    }
                    GameItem::UpperBoundary => {
                        if linear_velocity.x >= 0.0 {
                            linear_velocity.x = 200.0;
                        } else {
                            linear_velocity.x = -200.0;
                        }
                        linear_velocity.y = rng.random_range(-200.0..=-1.0);
                    }
                    GameItem::LowerBoundary => {
                        if linear_velocity.x >= 0.0 {
                            linear_velocity.x = 200.0;
                        } else {
                            linear_velocity.x = -200.0;
                        }
                        linear_velocity.y = rng.random_range(0.0..=200.0);
                    }
                    _ => {}
                };
            };
        };
        let collider2 = event.collider2;
        if let Ok(mut linear_velocity) = ball_query.get_mut(collider2)
            && let Ok(game_item_type) = query.get(collider1)
        {
            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
            match game_item_type.0 {
                GameItem::RightPaddle => {
                    linear_velocity.y = rng.random_range(-200.0..200.0);
                    linear_velocity.x = -200.0;
                }
                GameItem::LeftPaddle => {
                    linear_velocity.y = rng.random_range(-200.0..200.0);
                    linear_velocity.x = 200.0;
                }
                GameItem::UpperBoundary => {
                    if linear_velocity.x >= 0.0 {
                        linear_velocity.x = 200.0;
                    } else {
                        linear_velocity.x = -200.0;
                    }
                    linear_velocity.y = rng.random_range(-200.0..=-1.0);
                }
                GameItem::LowerBoundary => {
                    if linear_velocity.x >= 0.0 {
                        linear_velocity.x = 200.0;
                    } else {
                        linear_velocity.x = -200.0;
                    }
                    linear_velocity.y = rng.random_range(0.0..=200.0);
                }
                _ => {}
            }
        };
    }
}

fn ball_went_past_a_paddle(
    mut collision_reader: MessageReader<CollisionStart>,
    mut ball_query: Query<Entity, With<Ball>>,
    query: Query<&GameItemType, With<Wall>>,
    mut commands: Commands,
) {
    let ball_entity = match ball_query.single_mut(){
        Ok(val) => val,
        Err(_) => return,
    };
    for event in collision_reader.read() {
        let collider1 = event.collider1;
        let collider2 = event.collider2;
        if query.get(collider2).is_ok() {
            commands.entity(ball_entity).despawn();
            break;
        };
        if query.get(collider1).is_ok() {
            commands.entity(ball_entity).despawn();
            break;
        };
    }
}

#[derive(Bundle)]
struct PlayingStickBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    player_id: Player1,
    rigid_body: RigidBody,
    collider: Collider,
    game_item_type: GameItemType,
}

#[derive(Bundle)]
struct PlayingStick2Bundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    player_id: Player2,
    rigid_body: RigidBody,
    collider: Collider,
    game_item_type: GameItemType,
}

#[derive(Bundle)]
struct BoundaryBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    boundary_marker: Boundary,
    game_item_type: GameItemType,
}

#[derive(Bundle)]
struct WallBundle {
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    sensor: Sensor,
    game_item_type: GameItemType,
    wall_marker: Wall,
}
#[derive(Bundle)]
struct BallBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    ball_id: Ball,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: LinearVelocity,
    gravity: GravityScale,
    bounciness: Restitution,
    events_enabled: CollisionEventsEnabled,
}
#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Boundary;

#[derive(Component)]
struct Wall;

enum GameItem {
    RightPaddle,
    LeftPaddle,
    UpperBoundary,
    LowerBoundary,
    LeftWall,
    RightWall,
}
#[derive(Component)]
struct GameItemType(GameItem);

#[derive(Resource, Deref)]
struct CollisionSound(Handle<AudioSource>);

#[derive(Resource)]
struct BallSpawnConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

#[derive(Resource)]
struct BallMeshAndMaterial {
    /// How often to spawn a new bomb? (repeating timer)
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Component)]
struct ScoreboardUi;


#[derive(Resource)]
struct PlayerScores {
    /// How often to spawn a new bomb? (repeating timer)
    player1: usize,
    player2: usize
}


