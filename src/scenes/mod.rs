use crate::common::Env;

pub mod main;

pub trait Scene {
    fn init (&mut self, env: &mut Env) -> ();
    fn render (&mut self, env: &mut Env) -> ();
    fn tick (&mut self, env: &mut Env) -> ();
    fn cleanup (&mut self, env: &mut Env) -> ();
}