

use amethyst::{ 
    prelude::*,
    core::transform::Transform,
    assets::{AssetStorage, Loader},
    ecs::prelude::{Component, DenseVecStorage},
    renderer::{
      Camera,Projection,SpriteSheetHandle,Texture,PngFormat,
      TextureMetadata,SpriteSheet,SpriteSheetFormat,SpriteRender
    },
};

pub struct Pong;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub heigth: f32
}

impl Paddle {
    fn new(side: Side) -> Paddle { 
        Paddle {
            side,
            width: PADDLE_WIDTH,
            heigth: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        
        world.register::<Paddle>();
        let sheet_handle = load_sprite_sheet(world);

        initialise_camera(world);
        initialise_paddles(world, sheet_handle);
        
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialise_paddles(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut left_trans = Transform::default();
    let mut right_trans = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_trans.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_trans.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);


    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world.create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_trans)
        .with(sprite_render.clone())
        .build();

    world.create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_trans)
        .with(sprite_render.clone())
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    
    let texture_handle  = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

