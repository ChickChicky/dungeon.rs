use std::{
    sync::RwLock, 
    time::Instant
};

use ctrlc;

use termlib::{Term, TermEvent};
use renderer::Renderer;
use common::{Env,Event, RootEnv};

mod termlib;
mod renderer;
mod common;
mod scenes;

fn quit(env: &mut RootEnv) {
    env.term.disable_mouse();
    env.term.disable_alternate_buffer();
}

// Handles keypresses
fn handle_input(env: &mut RootEnv) {
    if let Some(event) = env.term.consume_event() {
        match event {
            TermEvent::Char { char } => {
                if char == 'q' {
                    env.quit = true;
                    return;
                }
            }
            _ => {}
        }
        env.send_event(Event::Terminal(event));
    }
}

fn main() {   
    ctrlc::set_handler(|| {}).unwrap();

    let mut env = RootEnv {
        quit: false,

        dt: 0,

        tick_speed: 10,
        render_speed: 32,
        
        term: Term::new(),
        env: Env {
            actions: vec![].into(),
            events: vec![].into(),
            renderer: Renderer::new(),
        },
        scenes: vec![],
    };

    env.term.enable_alternate_buffer();
    env.term.enable_mouse();
    env.term.init();

    env.push_scene(Box::new(RwLock::new(Box::new(scenes::main::MainScene::new()))));

    let mut tick_clock = Instant::now();
    let mut render_clock = Instant::now();

    while !env.quit {
        
        if let Some(scene_ref) = env.scenes.last() {
            let mut scene = scene_ref.write().unwrap();

            if tick_clock.elapsed().as_millis() as i32 >= env.tick_speed {
                env.dt = tick_clock.elapsed().as_millis() as i32;
                scene.tick(&mut env.env);
                tick_clock = Instant::now();
            }

            if render_clock.elapsed().as_millis() as i32 >= env.render_speed {
                env.dt = render_clock.elapsed().as_millis() as i32;
                scene.render(&mut env.env);
                render_clock = Instant::now();
            }
        } else {
            tick_clock = Instant::now();
            render_clock = Instant::now();
        }

        env.proces_actions();

        handle_input(&mut env);
    }

    quit(&mut env);
}
