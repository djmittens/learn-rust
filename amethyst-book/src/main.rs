use amethyst::{
    assets::Processor,
    ecs::{ReadExpect, Resources, SystemData},
    prelude::*,
    renderer::{
        pass::DrawFlat2DDesc, types::DefaultBackend, Factory, Format, GraphBuilder, GraphCreator,
        Kind, RenderFlat2D, RenderGroupDesc, RenderToWindow, RenderingBundle, RenderingSystem,
        SpriteSheet, SubpassBuilder,
    },
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};
mod pong;
use crate::pong::Pong;

/// Starts our app
/// This function is actually pretty cool because we get to set everything up.
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // i find it funny that rather than spell the whole thing out in 1 string with a `/` we have to type out .join
    let display_config_path = app_root.join("resources").join("display_config.ron");
    let asset_dir = app_root.join("assets");

    // we gunna open a window based on configuration we layed out.
    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)
                    .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
    )?;

    let mut game = Application::new(asset_dir, Pong, game_data)?;

    game.run();

    Ok(())
}
