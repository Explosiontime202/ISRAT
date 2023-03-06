use std::{sync::mpsc::Receiver, path::PathBuf, fmt::Error};

use timer::{Guard, Timer};

use crate::data::Competition;

#[derive(Clone, Copy)]
pub enum ProgramStage {
    StartScreenStage,
    NewScreenStage,
    CurrentErgViewStage,
}

pub struct ProgramState {
    pub stage: ProgramStage,
    pub size: [f32; 2],
    pub competition: Competition,
    pub threads: ThreadState,
}

impl ProgramState {
    pub fn new(stage: ProgramStage, size: [f32; 2]) -> ProgramState {
        ProgramState {
            stage,
            size,
            competition: Competition::empty(),
            threads: ThreadState::new(),
        }
    }

    pub fn switch_to_stage(&mut self, new_stage: ProgramStage) {
        match new_stage {
            ProgramStage::StartScreenStage => {
                todo!("Currently not implemented StartScreenStage init!")
            }
            ProgramStage::NewScreenStage => {
            }

            ProgramStage::CurrentErgViewStage => {
                // TODO: Add more state resets if needed

                let group_count = self.competition.data.as_ref().unwrap().team_distribution[0];

                self.competition.current_interim_result = (0..group_count).map(|_| None).collect();
            }

            #[allow(unreachable_patterns)]
            _ => todo!("Implement stage switch for more stages!"),
        }
        self.stage = new_stage;
    }
}

pub struct ThreadState {
    pub save_channels: Vec<Receiver<Result<Option<PathBuf>, Error>>>,
    pub open_channels: Vec<Receiver<Result<Option<PathBuf>, Error>>>,
    pub autosave_channel: Option<Receiver<()>>,
    pub autosave_guard: Option<Guard>,
    pub timer: Option<Timer>,
}

impl ThreadState {
    pub fn new() -> Self {
        Self {
            save_channels: Vec::new(),
            open_channels: Vec::new(),
            autosave_channel: None,
            autosave_guard: None,
            timer: None,
        }
    }
}
