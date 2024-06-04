use std::collections::VecDeque;
use std::sync::RwLock;

use crate::scenes::Scene;
use crate::termlib::{Term, TermEvent};
use crate::renderer::Renderer;

pub enum Event {
    Terminal(TermEvent)
}

pub type SceneRef = Box<RwLock<Box<dyn Scene + 'static>>>;

pub enum EnvAction {
    Quit,
    PushScene(SceneRef),
    PopScene
}

pub struct RootEnv {
    /** Whether to quit the game */
    pub quit: bool,

    /** When passed to a function and relevant, the amount of time since the last call */
    pub dt: i32,

    /** The target tick dt */
    pub tick_speed: i32,
    /** The target render dt */
    pub render_speed: i32,

    pub term: Term,
    pub env: Env,
    pub scenes: Vec<SceneRef>,
}

impl RootEnv {
    pub fn proces_actions(&mut self) {
        while let Some(action) = self.env.actions.pop_back() {
            match action {
                EnvAction::Quit => { 
                    self.quit = true; 
                },
                
                EnvAction::PushScene(scene) => {
                    self.push_scene(scene);
                },
                
                EnvAction::PopScene => {
                    self.pop_scene();
                },
            }
        }
    }

    pub fn push_scene(&mut self, scene: SceneRef) {
        self.env.events.clear();
        scene.write().unwrap().init(&mut self.env);
        self.scenes.push(scene);
    }
    
    pub fn pop_scene(&mut self) {
        if let Some(scene) = self.scenes.pop() {
            self.env.events.clear();
            scene.write().unwrap().cleanup(&mut self.env);
        }
    }

    pub fn send_event(&mut self, event: Event) {
        self.env.events.push_front(event);
    }
}

pub struct Env {
    pub actions: VecDeque<EnvAction>,
    pub events: VecDeque<Event>,
    pub renderer: Renderer,
}

impl Env {
    pub fn quit(&mut self) {
        self.actions.push_front(EnvAction::Quit);
    }
    
    pub fn push_scene(&mut self, scene: SceneRef) {
        self.actions.push_front(EnvAction::PushScene(scene));
    }
    
    pub fn pop_scene(&mut self) {
        self.actions.push_front(EnvAction::PopScene);
    }
}