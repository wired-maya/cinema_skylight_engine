use cinema_skylight_engine;
use cinema_skylight_engine::WindowConfig;

static WIDTH: u32 = 1280;
static HEIGHT: u32 = 720;

fn main() {
    let window_config = WindowConfig {
        width: WIDTH,
        height: HEIGHT,
        title: String::from("Test Game")
    };

    cinema_skylight_engine::init(window_config);
}
