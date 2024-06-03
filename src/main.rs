use std::{cell, time::Instant};

use ctrlc;
use termsize;

use termlib::{Term, TermEvent};
use renderer::{Renderer,Cell,cell_style};

mod termlib;
mod renderer;

struct Env<'a> {
    quit: bool,

    dt: i32,
    term_event: Option<TermEvent>,

    tick_speed: i32,   // The target tick dt
    render_speed: i32, // The target render dt

    term: &'a mut Term,
    renderer: &'a mut Renderer,
    player: (i32, i32),
}

fn quit(env: &mut Env) {
    env.term.disable_mouse();
    env.term.disable_alternate_buffer();
}

fn handle_input(env: &mut Env) {
    if let Some(event) = env.term.consume_event() {
        match event {
            TermEvent::Char { char } => {
                if char == 'q' {
                    env.quit = true;
                }
            }
            _ => {}
        }
        env.term_event = Some(event);
    }
}

// Renders the game
fn render(env: &mut Env) {
    env.renderer.clear();
    if let Some(size) = termsize::get() {
        let tw = size.cols as u32;
        let th = size.rows as u32;
        env.renderer.set(tw/2, th/2, Cell{c:'@',s:0});
        env.renderer.put(0, 0, "Hello".to_string(), cell_style::bg::RED);
        env.renderer.fill(2, 2, 5, 5, Cell{c:'.',s:0})
    }
    env.renderer.render();
    env.renderer.flip();
}

// Ticks the game
fn tick(env: &mut Env) {

}

fn main() {   
    ctrlc::set_handler(|| {}).unwrap();

    let mut env: Env = Env {
        quit: false,

        dt: 0,
        term_event: None,

        tick_speed: 10,
        render_speed: 32,

        term: &mut Term::new(),
        renderer: &mut Renderer::new(),
        player: (0, 0),
    };

    env.term.enable_alternate_buffer();
    env.term.enable_mouse();
    env.term.init();

    let mut tick_clock = Instant::now();
    let mut render_clock = Instant::now();

    while !env.quit {
        if tick_clock.elapsed().as_millis() as i32 >= env.tick_speed {
            env.dt = tick_clock.elapsed().as_millis() as i32;
            tick(&mut env);
            tick_clock = Instant::now();
        }

        if render_clock.elapsed().as_millis() as i32 >= env.render_speed {
            env.dt = render_clock.elapsed().as_millis() as i32;
            render(&mut env);
            render_clock = Instant::now();
        }

        env.term_event = None;
        handle_input(&mut env);
    }

    quit(&mut env);
}
