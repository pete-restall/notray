use crate::{OnStimuli, Result, Stimuli};
use crate::raycasting::*;
use super::Scene;

pub trait HasCameraMut {
    type EngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry;

    fn camera_mut(&mut self) -> &mut Camera<Self::EngineParameters>;
}

pub trait CameraStimuli {
    fn should_move_forward(&self) -> bool;

    fn should_move_backward(&self) -> bool;

    fn should_turn_left(&self) -> bool;

    fn should_turn_right(&self) -> bool;

    fn is_fast(&self) -> bool;
}

impl<TEngineParameters, TWorld, TStimuli> OnStimuli<TStimuli> for Scene<TEngineParameters, TWorld>
    where
        TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry,
        TWorld: World,
        TStimuli: Stimuli + CameraStimuli {

    fn on_stimuli(&mut self, stimuli: &TStimuli) -> Result<()> {
        let left = Angle::from_raw(0x0400);
        if stimuli.should_turn_left() {
            self.camera_mut().turn(left); // TODO: angle needs to be determined based on frame rate, and whether 'is_fast()'
        } else if stimuli.should_turn_right() {
            self.camera_mut().turn(-left); // TODO: angle needs to be determined based on frame rate, and whether 'is_fast()'
        }

        let forward = WorldRelativeCoordinate::lit("0.125");
        if stimuli.should_move_forward() {
            self.camera_mut().move_relative(forward); // TODO: distance needs to be determined based on frame rate, and whether 'is_fast()'
        } else if stimuli.should_move_backward() {
            self.camera_mut().move_relative(-forward); // TODO: distance needs to be determined based on frame rate, and whether 'is_fast()'
        }

        Ok(())
    }
}
