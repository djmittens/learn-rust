use amethyst :: {
    assets::Processor,
    ecs::{ReadExpect, Resources, SystemData},
    prelude::*,
    renderer::{
        pass::DrawFlat2DDesc, types::DefaultBackend, Factory, Format, GraphBuilder, GraphCreator,
        Kind, RenderGroupDesc, RenderingSystem, SpriteSheet, SubpassBuilder,
    },
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};

pub struct Pong;
impl SimpleState for Pong {}

/// Starts our app
/// This function is actually pretty cool because we get to set everything up.
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    // i find it funny that rather than spell the whole thing out in 1 string with a `/` we have to type out .join
    let display_config_path = app_root.join("resources").join("display_config.ron");

    // we gunna open a window based on configuration we layed out.
    let mut game_data = GameDataBuilder::default()
    .with_bundle(WindowBundle::from_config_path(display_config_path))?
    .with(
        Processor::<SpriteSheet>::new(), 
        "sprite_sheet_processor", 
        &[])
    .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
        ExampleGraph::default()
        ));

    let asset_dir = app_root.join("assets");
    let mut game = Application::new(asset_dir, Pong, game_data)?;
    game.run();

    Ok (())
}

#[derive(Default)]
struct ExampleGraph {
    dimensions: Option<ScreenDimensions>,
    dirty: bool,
}

impl GraphCreator<DefaultBackend> for ExampleGraph {
    fn rebuild(&mut self, res: &Resources) -> bool {
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        use std::ops::Deref;
        if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.clone());
            return false;
        }
        return self.dirty;
    }

    fn builder(
        &mut self,
        factory: &mut Factory<DefaultBackend>,
        res: &Resources,
    ) -> GraphBuilder<DefaultBackend, Resources> {
        use amethyst::renderer::rendy::{
            graph::present::PresentNode,
            hal::command::{ClearDepthStencil, ClearValue},
        };

        self.dirty = false;

        let window = <ReadExpect<'_, Window>>::fetch(res);
        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind = Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);
        let surface = factory.create_surface(&window);
        let surface_format = factory.get_surface_format(&surface);

        let mut graph_builder = GraphBuilder::new();
        let color = graph_builder.create_image(
            window_kind,
            1,
            surface_format,
            Some(ClearValue::Color([0.0, 0.0,0.0,1.0].into())),
        );

        let depth = graph_builder.create_image(
            window_kind,
            1,
            Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        let pass = graph_builder.add_node(
            SubpassBuilder::new()
            .with_group(DrawFlat2DDesc::new().builder())
            .with_color(color)
            .with_depth_stencil(depth)
            .into_pass()
        );

        let _present = graph_builder.add_node(PresentNode::builder(factory, surface, color)
        .with_dependency(pass));

        graph_builder
    }
}