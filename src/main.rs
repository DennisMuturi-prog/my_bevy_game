use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use rand::seq::IndexedRandom;
fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PhysicsPlugins::default()));
    app.add_systems(Startup, setup);
    app.add_systems(Startup, set_up_score_board);
    app.add_systems(Update, (control_stick_1, control_stick_2));
    app.add_systems(Update, (started_collisions, ball_went_past_a_paddle));
    app.add_systems(PostUpdate, reset_ball);
    app.run();
}
const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const SPEEDS:[f32;8]=[-200.0,-150.0,-100.0,-50.0,50.0,100.0,150.0,200.0];
fn set_up_score_board(mut commands: Commands) {
    commands.insert_resource(PlayerScores {
        player1: 0,
        player2: 0,
    });
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
            // flex_direction:FlexDirection::Row,
            // column_gap:Val::Px(10.0),
            ..default()
        },
        children![
            (
                TextSpan("Player1: 0".to_string()),
                TextFont {
                    font_size: SCOREBOARD_FONT_SIZE,
                    ..default()
                },
                TextColor(SCORE_COLOR),
            ),
            (
                TextSpan("Player2: 0".to_string()),
                TextFont {
                    font_size: SCOREBOARD_FONT_SIZE,
                    ..default()
                },
                TextColor(SCORE_COLOR),
            )
        ],
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
    let shape = Rectangle::new(5.0, 500.0);
    let mesh = meshes.add(shape);

    commands.spawn(WallBundle {
        mesh: Mesh2d(mesh.clone()),
        material: MeshMaterial2d(material.clone()),
        transform: Transform::from_xyz(400.0, 0.0, 0.0),
        rigid_body: RigidBody::Static,
        collider: Collider::rectangle(5.0, 500.0),
        game_item_type: GameItemType(GameItem::RightWall),
        sensor: Sensor,
        wall_marker: Wall,
    });

    commands.spawn(WallBundle {
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material),
        transform: Transform::from_xyz(-400.0, 0.0, 0.0),
        rigid_body: RigidBody::Static,
        collider: Collider::rectangle(5.0, 500.0),
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
        // bounciness: Restitution::new(1.0),
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
                    // bounciness: Restitution::new(1.0),
                    events_enabled: CollisionEventsEnabled,
                });
            }
        }
    }
}

fn control_stick_1(keys: Res<ButtonInput<KeyCode>>, mut player: Single<&mut Transform, With<Player1>>) {

    if keys.pressed(KeyCode::ArrowUp) {
        let movement=player.translation.y+5.0;
        player.translation.y=movement.min(200.0);
    }
    if keys.pressed(KeyCode::ArrowDown) {
        let movement=player.translation.y-5.0;
        player.translation.y=movement.max(-200.0);
    }
}

fn control_stick_2(keys: Res<ButtonInput<KeyCode>>, mut player: Single<&mut Transform, With<Player2>>) {
    if keys.pressed(KeyCode::KeyW) {
        let movement=player.translation.y+5.0;
        player.translation.y=movement.min(200.0);
    }
    if keys.pressed(KeyCode::KeyS) {
        let movement=player.translation.y-5.0;
        player.translation.y=movement.max(-200.0);
    }
}
fn started_collisions(
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

                let choice = *(SPEEDS.choose(&mut rng).unwrap());
                println!("choice is {}",choice);
                match game_item_type.0 {
                    GameItem::RightPaddle => {
                        linear_velocity.y = choice;
                        linear_velocity.x = -200.0;
                    }
                    GameItem::LeftPaddle => {
                        linear_velocity.y = choice;
                        linear_velocity.x = 200.0;
                    }
                    GameItem::UpperBoundary => {
                        if linear_velocity.x >= 0.0 {
                            linear_velocity.x = 200.0;
                        } else {
                            linear_velocity.x = -200.0;
                        }
                        linear_velocity.y = *(SPEEDS[0..=3].choose(&mut rng).unwrap());
                    }
                    GameItem::LowerBoundary => {
                        if linear_velocity.x >= 0.0 {
                            linear_velocity.x = 200.0;
                        } else {
                            linear_velocity.x = -200.0;
                        }
                        linear_velocity.y = *(SPEEDS[4..].choose(&mut rng).unwrap());
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
            let choice = *(SPEEDS.choose(&mut rng).unwrap());
            println!("choice is {}",choice);
            match game_item_type.0 {
                GameItem::RightPaddle => {
                    linear_velocity.y = choice;
                    linear_velocity.x = -200.0;
                }
                GameItem::LeftPaddle => {
                    linear_velocity.y = choice;
                    linear_velocity.x = 200.0;
                }
                GameItem::UpperBoundary => {
                    if linear_velocity.x >= 0.0 {
                        linear_velocity.x = 200.0;
                    } else {
                        linear_velocity.x = -200.0;
                    }
                    linear_velocity.y = *(SPEEDS[0..=3].choose(&mut rng).unwrap());
                }
                GameItem::LowerBoundary => {
                    if linear_velocity.x >= 0.0 {
                        linear_velocity.x = 200.0;
                    } else {
                        linear_velocity.x = -200.0;
                    }
                    linear_velocity.y = *(SPEEDS[4..].choose(&mut rng).unwrap());
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
    mut score: ResMut<PlayerScores>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    let ball_entity = match ball_query.single_mut() {
        Ok(val) => val,
        Err(_) => return,
    };
    for event in collision_reader.read() {
        let collider1 = event.collider1;
        let collider2 = event.collider2;
        if let Ok(game_item_type) = query.get(collider2) {
            match game_item_type.0 {
                GameItem::LeftWall => {
                    commands.entity(ball_entity).despawn();
                    score.player2 += 1;
                    *writer.text(*score_root, 2) = format!("Player2: {}", score.player2);
                    break;
                }
                GameItem::RightWall => {
                    commands.entity(ball_entity).despawn();
                    score.player1 += 1;
                    *writer.text(*score_root, 1) = format!("Player1: {}", score.player1);
                    break;
                }
                _ => {}
            }
        };

        if let Ok(game_item_type) = query.get(collider1) {
            match game_item_type.0 {
                GameItem::LeftWall => {
                    commands.entity(ball_entity).despawn();
                    score.player2 += 1;
                    *writer.text(*score_root, 2) = format!("Player2: {}", score.player2);
                    break;
                }
                GameItem::RightWall => {
                    commands.entity(ball_entity).despawn();
                    score.player1 += 1;
                    *writer.text(*score_root, 1) = format!("Player1: {}", score.player1);
                    break;
                }
                _ => {}
            }
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
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
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
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Component)]
struct ScoreboardUi;

#[derive(Resource)]
struct PlayerScores {
    player1: usize,
    player2: usize,
}
