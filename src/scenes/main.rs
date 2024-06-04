#![allow(unused_variables)]

use std::time::Instant;

use term_size::dimensions;

use crate::{
    common::{
        Env, 
        Event
    }, 
    renderer::{
        Color, 
        Style, 
        TextOptions, TextStyle,
    }, 
    termlib::TermEvent
};

use super::Scene;

pub struct MainScene {
    test: String,
    offset: (i32, i32),
    cursor_blink: Instant,
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            test: String::new(),
            offset: ( 0, 0 ),
            cursor_blink: Instant::now()
        }
    }
}

impl Scene for MainScene {
    fn init(&mut self, env: &mut Env) {
        
    }
    fn render(&mut self, env: &mut Env) {
        env.renderer.clear();
        if let Some((w, h)) = dimensions() {
            let tw = w as u32;
            let th = h as u32;
            // env.renderer.set(tw/2, th/2, Cell{ c: '@', s: Style::default() });
            // env.renderer.put_text(0, 0, self.test.clone());
            env.renderer.paint(1, 1, 48, 27,*Style::default().bg(Color::BrightBlack).fg(Color::BrightWhite));
            let text = TextOptions::builder()
                .text(self.test.clone())
                .pos((1,1))
                .offset(self.offset)
                .wrap(false)
                .max_w(48)
                .max_h(27)
            .build();
            env.renderer.put(
                &text
            );
            if (self.cursor_blink.elapsed().as_millis() / 500) %2 == 0 {
                if let Some((x,y)) = text.idx_to_xy(env.renderer.buffer.width, env.renderer.buffer.height,self.test.len()) {
                    env.renderer.at(x as u32,y as u32).s.reverse(true);
                }
            }
            /*env.renderer.fill(2, 2, 5, 5, Cell{ c: '.', s: Style::default()});
            env.renderer.paint(0, 0, 3, 3, *Style::default().bg(Color::Green).strike(true));
            env.renderer.apply(
                0, 0, 10, 10, 
                &|cell, x, y| {
                    if (x + y) % 2 == 0 {
                        cell.s.bg(Color::Blue);
                    }
                }
            );*/
        }
        env.renderer.render();
        env.renderer.flip();
    }
    fn tick (&mut self, env: &mut Env) {
        while let Some(ev) = env.events.pop_back() {
            match ev {
                Event::Terminal( term_event ) => {
                    match term_event {
                        TermEvent::Char { char } => {
                            if !char.is_control() {
                                self.test.push(char);
                            }
                        }
                        TermEvent::Enter => {
                            self.test.push('\n');
                        }
                        TermEvent::Backspace => {
                            self.test.pop();
                        }
                        TermEvent::Arrow { x, y, modifiers } => {
                            self.offset.0 -= x;
                            self.offset.1 -= y;
                        }
                        _ => {}
                    }
                    self.cursor_blink = Instant::now();
                }
            }
        }
    }
    fn cleanup (&mut self, env: &mut Env) {

    }
}