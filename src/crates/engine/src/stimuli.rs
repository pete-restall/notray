use super::Result;

pub trait Stimuli { }

pub trait QuitStimuli {
    fn should_quit(&self) -> bool;
}

pub trait OnStimuli<T: Stimuli> {
    fn on_stimuli(&mut self, stimuli: &T) -> Result<()>;
}
