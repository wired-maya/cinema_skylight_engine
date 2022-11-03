use cinema_skylight_engine::*;

static WIDTH: u32 = 1280;
static HEIGHT: u32 = 720;

fn main() {
    let window_config = WindowConfig {
        width: WIDTH,
        height: HEIGHT,
        title: String::from("Test Game")
    };

    let engine = CinemaSkylightEngine::init(window_config);

    engine.wait_for_advance();
}
