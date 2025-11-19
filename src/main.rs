use bevy::prelude::*;
use avian2d::prelude::*;
fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PhysicsPlugins::default()));
    app.add_systems(Startup, setup);
    app.add_systems(Update, (control_stick_1, control_stick_2));
    app.add_systems(Update, print_started_collisions);

    app.run();
}



fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
        rigid_body:RigidBody::Kinematic,
        collider:Collider::rectangle(20.0, 100.0),
        game_item_type:GameItemType(GameItem::RightPaddle)
    });
    commands.spawn(PlayingStick2Bundle {
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material.clone()),
        transform: Transform::from_xyz(-250.0, 0.0, 0.0),
        player_id: Player2,
        rigid_body:RigidBody::Kinematic,
        collider:Collider::rectangle(20.0, 100.0),
        game_item_type:GameItemType(GameItem::LeftPaddle)
    });
    let shape = Rectangle::new(200.0, 10.0);
    let mesh = meshes.add(shape);
    commands.spawn(BoundaryBundle {
        mesh: Mesh2d(mesh.clone()),
        material: MeshMaterial2d(material.clone()),
        transform: Transform::from_xyz(0.0,-250.0, 0.0),
        rigid_body:RigidBody::Static,
        collider:Collider::rectangle(200.0, 10.0),
        boundary_marker:Boundary,
        game_item_type:GameItemType(GameItem::LowerBoundary)
    });

    commands.spawn(BoundaryBundle {
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material),
        transform: Transform::from_xyz(0.0,250.0, 0.0),
        rigid_body:RigidBody::Static,
        collider:Collider::rectangle(200.0, 10.0),
        boundary_marker:Boundary,
        game_item_type:GameItemType(GameItem::UpperBoundary)
    });

    let shape = Circle::new(20.0);
    let mesh = meshes.add(shape);
    let color = Color::Srgba(Srgba::rgb(1.0, 0.647, 0.0));
    let material = materials.add(color);
    commands.spawn(BallBundle {
        mesh: Mesh2d(mesh),
        material: MeshMaterial2d(material),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ball_id: Ball,
        rigid_body:RigidBody::Dynamic,
        collider:Collider::circle(20.0),
        velocity:LinearVelocity(Vec2::new(100.0,0.0)),
        gravity:GravityScale(0.0),
        bounciness:Restitution::new(0.8),
        events_enabled:CollisionEventsEnabled
    });
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
    mut query:Query<&mut LinearVelocity,With<Ball>>
) {
    for event in collision_reader.read() {
        let collider1=event.collider1;
        if let Ok(mut linear_velocity)=query.get_mut(collider1){
            println!("collider 1");
            linear_velocity.y=100.0;
            linear_velocity.x=100.0;

        };
        let collider2=event.collider2;
        if let Ok(mut linear_velocity)=query.get_mut(collider2){
            println!("collider 2");
            linear_velocity.y=100.0;
            linear_velocity.x=100.0;

        };
        println!("{} and {} started colliding", event.collider1, event.collider2);
    }
}

#[derive(Bundle)]
struct PlayingStickBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    player_id: Player1,
    rigid_body:RigidBody,
    collider:Collider,
    game_item_type:GameItemType
}

#[derive(Bundle)]
struct PlayingStick2Bundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    player_id: Player2,
    rigid_body:RigidBody,
    collider:Collider,
    game_item_type:GameItemType
    
}

#[derive(Bundle)]
struct BoundaryBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    rigid_body:RigidBody,
    collider:Collider,
    boundary_marker:Boundary,
    game_item_type:GameItemType
}
#[derive(Bundle)]
struct BallBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    ball_id: Ball,
    rigid_body:RigidBody,
    collider:Collider,
    velocity:LinearVelocity,
    gravity:GravityScale,
    bounciness:Restitution,
    events_enabled:CollisionEventsEnabled
}
#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Boundary;

enum GameItem{
    RightPaddle,
    LeftPaddle,
    UpperBoundary,
    LowerBoundary
}
#[derive(Component)]
struct GameItemType(GameItem);