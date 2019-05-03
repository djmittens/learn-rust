extern crate amethyst;

mod pong;
mod systems;

use amethyst::{
    core::transform::bundle::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
    LoggerConfig,
    StdoutLog,
    LogLevelFilter,
};


use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    // have to silence some logs, because amethyst keeps creating gl buffers and spams the std out.
    // https://github.com/amethyst/amethyst/issues/1217
    // amethyst::start_logger(Default::default());
    amethyst::start_logger(LoggerConfig {
        stdout: StdoutLog::Colored,
        level_filter: LogLevelFilter::Warn,
        log_file: None,
        allow_env_override: true,
    });

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    // Some random rendering code, but aparantly its too early to understand it yet.
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(input_bundle)?
        .with_bundle(TransformBundle::new())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);

    let mut game = Application::new("./", Pong {}, game_data)?;
    game.run();

    Ok(())
}
