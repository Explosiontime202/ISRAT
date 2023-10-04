use crate::{data::Competition, AUTO_SAVE_THREADS};
use std::{
    sync::{
        mpsc::{self, SyncSender, TryRecvError},
        RwLock, Weak,
    },
    thread::{sleep, JoinHandle},
    time::{Duration, SystemTime},
};

pub fn spawn_autosave_thread(start_interval: Duration, competition: Weak<RwLock<Competition>>) -> SyncSender<AutoSaveMsg> {
    let (tx, rx) = mpsc::sync_channel(std::mem::size_of::<AutoSaveMsg>());
    let auto_save_thread_handle = std::thread::spawn(move || {
        println!("Auto-save: Default interval: {:?}", start_interval);
        let mut cur_interval = start_interval;
        let mut skip_interval = cur_interval;
        let mut do_not_save = false;
        loop {
            loop {
                match rx.try_recv() {
                    Ok(msg) => match msg {
                        AutoSaveMsg::Interval(new_interval) => {
                            cur_interval = new_interval;
                            skip_interval = cur_interval;
                            println!("Auto-save: new interval: {:?}", cur_interval);
                        }
                        AutoSaveMsg::Stop => {
                            do_not_save = true;
                            println!("Auto-save: stop");
                        }
                        AutoSaveMsg::Continue => {
                            do_not_save = false;
                            skip_interval = cur_interval;
                            println!("Auto-save: continue");
                        }
                        AutoSaveMsg::Exit => {
                            println!("Auto-save: exit");
                            return;
                        }
                    },
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                };
            }

            let start = SystemTime::now();
            sleep(Duration::new(1, 0));
            let diff = match SystemTime::now().duration_since(start) {
                Ok(diff) => diff,
                Err(err) => {
                    eprintln!("The clock has run backwards!: {}", err.to_string());
                    continue;
                }
            };

            if skip_interval >= diff {
                skip_interval -= diff;
                continue;
            }

            skip_interval = cur_interval;

            if do_not_save {
                continue;
            }

            println!("Auto-save: Save data to file!");

            let competition_ptr = match competition.upgrade() {
                Some(comp) => comp,
                None => return,
            };

            let competition_read = match competition_ptr.read() {
                Ok(val) => val,
                Err(_) => return,
            };

            match competition_read.save_to_file() {
                Ok(_) => (),
                Err(err_msg) => eprintln!("Auto-save: failed export: {err_msg}"),
            }
        }
    });

    AUTO_SAVE_THREADS.write().expect("AUTO_SAVE_THREADS is poisoned!").push(AutoSaveThread {
        handle: auto_save_thread_handle,
        channel: tx.clone(),
    });

    tx
}

#[derive(Debug, Clone, Copy)]
pub enum AutoSaveMsg {
    Interval(Duration),
    Stop,
    Continue,
    Exit,
}

#[derive(Debug)]
pub struct AutoSaveThread {
    pub handle: JoinHandle<()>,
    pub channel: SyncSender<AutoSaveMsg>,
}

impl AutoSaveThread {
    pub fn stop(self) {
        self.channel
            .send(AutoSaveMsg::Exit)
            .expect("Sending autosave channel exit message failed!");
        self.handle.join().expect("Joining autosave thread failed!");
    }
}
