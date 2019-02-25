
mod pong;
use crate::pong::Pong;

extern crate amethyst;

use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};



fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let path = app_root.join("resources/display_config.ron");

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?;

    let textures = app_root.join("assets/");

    let mut game = Application::new(textures, Pong, game_data)?;

    game.run();

    Ok(())
}
