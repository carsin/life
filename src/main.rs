extern crate crossterm;

use crossterm::{cursor, event, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod game;
mod map;
mod util;

const TICKS_PER_SECOND: u64 = 1000;
const TICK_TIME: Duration = Duration::from_nanos(((1.0 / TICKS_PER_SECOND as f32) * 1000000000.0) as u64);

fn main() {
    let stdout = stdout();
    let map = map::Map::new(500, 500);

    let game = game::Game::new(stdout, map);

    run(game);
}

fn run(mut game: game::Game) {
    set_up_terminal(&mut game);

    game.running = true;

    let mut last_tick = Instant::now();

    while game.running {
        let current_tick = Instant::now();
        let delta_time = current_tick.duration_since(last_tick).as_nanos();

        last_tick = current_tick;

        // Handle input
        while let Ok(true) = event::poll(Duration::from_millis(10)) {
            match event::read().unwrap() {
                // Key input
                event::Event::Key(key_event) => game.process_key_input(key_event.code),
                event::Event::Mouse(mouse_event) => game.process_mouse_input(mouse_event),
                // Terminal resize
                event::Event::Resize(width, height) => {
                    game.resize_viewport(width as usize, height as usize)
                },
            }
        }

        // Update
        if !game.paused {
            game.update();
        }

        // Render
        game.stdout.queue(cursor::Hide).unwrap();
        game.render_status();
        game.render_map();
        game.stdout.flush().unwrap();

        if delta_time < TICK_TIME.as_nanos() {
            sleep(TICK_TIME - Duration::from_nanos(delta_time as u64));
            continue;
        }
    }

    stop(game);
}

fn set_up_terminal(game: &mut game::Game) {
    game.stdout.queue(terminal::EnterAlternateScreen).unwrap();
    game.stdout.queue(cursor::Hide).unwrap();
    game.stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    game.stdout.queue(event::EnableMouseCapture).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout().flush().unwrap();
}

fn stop(mut game: game::Game) {
    // Restore terminal after game is finished
    game.stdout.queue(cursor::Show).unwrap();
    game.stdout.queue(terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    game.stdout.queue(event::DisableMouseCapture).unwrap();
    game.stdout.flush().unwrap();
    println!("Game exited successfully");
}
