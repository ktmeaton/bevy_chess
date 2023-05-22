use bevy::prelude::*;
use bevy::window::*;
use enum_iterator::{all, Sequence};

fn main() {

    let primary_window = Window {
        title: String::from("Chess"),
        resolution: WindowResolution::new(720., 720.),
        ..default()
    };

    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(primary_window),
            ..default()
        }))
        .add_systems(Startup, (setup, create_board, create_pieces))
        .run();
}

fn setup(
    mut commands: Commands,
)  {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(3.5, 12., -12.,))
            .looking_at(Vec3::new(3.5, 0., 3.5), Vec3::Y),
        ..default()     
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(4., 12., 4.)),
        ..default()
    });

}

fn create_board (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.,
        subdivisions: 1,
    }));
    let white_material = materials.add(Color::WHITE.into());
    let black_material = materials.add(Color::BLACK.into());

    // Board squares (64)
    for i in 0..8 {
        for j in 0..8 {
            let board_square = PbrBundle {
                mesh: mesh.clone(),
                // Change material for alternating squares
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..default()
            };

            commands.spawn(board_square);
        }
    }

    // Board text (A1, B2, ...)
}

#[derive(Debug)]
struct BoardCoord {
    x: u8,
    y: u8,
}
#[non_exhaustive]
#[derive(Debug)]
struct BoardSquare;

impl BoardSquare {
    pub const A1: BoardCoord = BoardCoord {x: 0, y: 0};
    pub const A8: BoardCoord = BoardCoord {x: 7, y: 0};
    pub const C1: BoardCoord = BoardCoord {x: 0, y: 2};
    pub const C8: BoardCoord = BoardCoord {x: 7, y: 2};
    pub const F1: BoardCoord = BoardCoord {x: 0, y: 5};
    pub const F8: BoardCoord = BoardCoord {x: 7, y: 5};
    pub const H1: BoardCoord = BoardCoord {x: 0, y: 7};
    pub const H8: BoardCoord = BoardCoord {x: 7, y: 7};
}

#[derive(Clone, Debug, PartialEq, Sequence)]
pub enum PieceColor {
    White, 
    Black,
}

#[derive(Clone, Debug, PartialEq, Sequence)]
enum PieceShape {
    Rook,
    Bishop,
}

#[derive(Debug)]
pub struct Piece {
    color: PieceColor,
    shape: PieceShape,
    location: BoardCoord,
    in_play: bool,
}

fn create_pieces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let piece_shapes = all::<PieceShape>().collect::<Vec<_>>();
    let piece_colors = all::<PieceColor>().collect::<Vec<_>>();

    let white_material = materials.add(Color::WHITE.into());
    let black_material = materials.add(Color::BLACK.into());

    for shape in &piece_shapes {
        println!{"{shape:?}"};

        let mesh_id = match shape {
            PieceShape::Rook => 0,
            PieceShape::Bishop => 1,
        };

        let piece_mesh: Handle<Mesh> = asset_server.load(format!("models/chess_kit/pieces.gltf#Mesh{mesh_id}/Primitive0"));

        for color in &piece_colors {
            println!{"\t{color:?}"}; 

            let piece_material = match color {
                PieceColor::White => white_material.clone(),
                PieceColor::Black => black_material.clone(),
            };            

            // Ex. White rook
            let spawn_locations = match shape {
                PieceShape::Rook => match color {
                    PieceColor::White => vec!(BoardSquare::A1, BoardSquare::H1),
                    PieceColor::Black => vec!(BoardSquare::A8, BoardSquare::H8),
                    _ => vec!(BoardSquare::A1),
                },
                PieceShape::Bishop => match color {
                    PieceColor::White => vec!(BoardSquare::C1, BoardSquare::F1),
                    PieceColor::Black => vec!(BoardSquare::C8, BoardSquare::F8),
                    _ => vec!(BoardSquare::A1),
                },                
                _ => vec!(BoardSquare::A1),
            };

            for location in spawn_locations {
                println!("\t{location:?}");

                commands.spawn(PbrBundle {
                    mesh: piece_mesh.clone(),
                    material: piece_material.clone(),
                    transform: Transform::from_translation(Vec3::new(location.x as f32, 0.5, location.y as f32)).with_scale(Vec3::splat(0.75)),
                    ..default()
                }); 
            }
        }
    }

    // Spawn
    // let piece_mesh: Handle<Mesh> = asset_server.load(format!("models/chess_kit/rook.gltf#Mesh{mesh_id}/Primitive0"));    
    // commands.spawn(PbrBundle {
    //     mesh: piece_mesh,
    //     material: piece_material,
    //     transform: Transform::from_translation(Vec3::new(0., 0.5, 0.)).with_scale(Vec3::splat(0.75)),
    //     ..default()
    // });    



}
