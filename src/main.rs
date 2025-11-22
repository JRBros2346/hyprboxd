mod state;

use smithay::reexports::{
    ash::ext::display_control,
    calloop::EventLoop,
    wayland_server::{Display, DisplayHandle},
};
use state::HyprBoxd;

pub struct CalloopData {
    state: HyprBoxd,
    display_handle: DisplayHandle,
}

fn main() -> Result<(), _> {
    if let On(env) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().init();
    } else {
        tracing_subscriber::fmt().init();
    }
    let event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;
    let display: Display<HyprBoxd> = Display::new()?;
    let display_handle = display.handle();
    let state = HyprBoxd::new(&mut event_loop, display);
    let mut data = CalloopData {
        state,
        display_handle,
    };
    crate::winit::init_winit(&mut event_loop, &mut data)?;
    let mut args = std::env::args().skip(1);
    let flag = args.next();
    let arg = args.next();

    match (flag.as_deref(), arg) {
        (Some("-c") | Some("--command"), Some(command)) => {
            std::process::Command::new(command).spawn().ok();
        }
        _ => {
            std::process::Command::new("weston-terminal").spawn().ok();
        }
    }

    event_loop.run(None, &mut data, move |_| {})?;

    Ok(())
}
