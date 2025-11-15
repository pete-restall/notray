use super::{Canvas, FrameRenderer, OnStimuli, Pollable, QuitStimuli, Result, Stimuli};

pub struct GameLoop<'gl, TScene, TSceneStimuli, TFrameUpdater, TCanvas>
    where
        TScene: FrameRenderer<TCanvas> + OnStimuli<TSceneStimuli>,
        TSceneStimuli: Stimuli + QuitStimuli,
        TFrameUpdater: Pollable,
        TCanvas: Canvas {

    scene: &'gl mut TScene,
    stimuli: &'gl TSceneStimuli,
    frame_updater: &'gl mut TFrameUpdater,
    canvas: &'gl mut TCanvas
}

impl<'gl, TScene, TSceneStimuli, TFrameUpdater, TCanvas> GameLoop<'gl, TScene, TSceneStimuli, TFrameUpdater, TCanvas>
    where
        TScene: FrameRenderer<TCanvas> + OnStimuli<TSceneStimuli>,
        TSceneStimuli: Stimuli + QuitStimuli,
        TFrameUpdater: Pollable,
        TCanvas: Canvas {

    pub fn new(
        scene: &'gl mut TScene,
        stimuli: &'gl TSceneStimuli,
        frame_updater: &'gl mut TFrameUpdater,
        canvas: &'gl mut TCanvas) -> Self {

        Self {
            scene,
            stimuli,
            frame_updater,
            canvas
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.stimuli.should_quit() {
            self.scene.render_frame_onto(&mut self.canvas)?;
            self.frame_updater.poll()?;
            self.scene.on_stimuli(&self.stimuli)?;
        }

        Ok(())
    }
}
