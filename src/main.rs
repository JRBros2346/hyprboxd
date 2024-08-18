use smithay::reexports::{
    calloop::{EventLoop, Interest, Mode, PostAction},
    wayland_server::Display,
};
use std::os::unix::io::AsRawFd;
use std::time::Duration;

struct State {
    display: Display,
}

impl State {
    fn new() -> Self {
        let display = Display::new();
        Self { display }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop: EventLoop<State> = EventLoop::try_new()?;
    let mut state = State::new();

    let display_fd = state.display.get_poll_fd().as_raw_fd();
    event_loop
        .handle()
        .insert_source(
            smithay::reexports::calloop::generic::Generic::new(
                display_fd,
                Interest::READ,
                Mode::Level,
            ),
            |_, _, state: &mut State| {
                state.display.dispatch(Duration::from_millis(0), &mut ()).unwrap();
                Ok(PostAction::Continue)
            },
        )
        .unwrap();

    let _listener = state.display.add_socket_auto()?;

    println!("Wayland socket is ready. You can now start Wayland clients.");

    event_loop.run(None, &mut state, |state| {
        state.display.flush_clients(&mut ());
    })?;

    Ok(())
}
