mod buffer;
mod client;
mod manager;
mod message;
mod raw;
mod server;
mod shader;
mod texture;
mod vertex;

pub use render::client::*;
pub use render::message::*;
pub use render::server::Window;

use render::server::*;
use utility;

pub fn start(
    window: Window,
    render_consumer: utility::bucket_spsc::Consumer<RenderMessage>,
    render_control: utility::control::Consumer,
) {
    let mut server = RenderServer::new(window, render_consumer, render_control);
    server.run_forever();
}
