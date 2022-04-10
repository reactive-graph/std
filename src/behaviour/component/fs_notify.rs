use std::sync::Arc;
use std::time::Duration;

use crossbeam::channel::unbounded;
use crossbeam::channel::Sender;
use log::trace;
use notify::Event;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use serde_json::json;
use tokio::runtime::Handle;

use crate::behaviour::component::FsNotifyProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const FS_NOTIFY: &str = "fs_notify";

pub struct FsNotify {
    pub entity: Arc<ReactiveEntityInstance>,

    stopper_tx: Sender<()>,
}

impl FsNotify {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>, runtime: &Handle) -> Result<FsNotify, BehaviourCreationError> {
        let filename = e
            .properties
            .get(FsNotifyProperties::FILENAME.as_ref())
            .ok_or(BehaviourCreationError)?
            .as_string()
            .ok_or(BehaviourCreationError)?;
        let _ = e.properties.get(FsNotifyProperties::TRIGGER.as_ref()).ok_or(BehaviourCreationError)?;

        let (stopper_tx, stopper_rx) = unbounded();
        let (notify_tx, notify_rx) = unbounded();

        let mut watcher: RecommendedWatcher = RecommendedWatcher::new(move |result: std::result::Result<Event, notify::Error>| {
            let _ = notify_tx.send(result);
        })
        .map_err(|_| BehaviourCreationError)?;
        watcher
            .watch(filename.as_ref(), RecursiveMode::NonRecursive)
            .map_err(|_| BehaviourCreationError)?;

        let entity = e.clone();
        runtime.spawn(async move {
            loop {
                if let Ok(notify_result) = notify_rx.try_recv() {
                    if let Ok(_notify_event) = notify_result {
                        entity.set(FsNotifyProperties::TRIGGER, json!(true));
                    }
                }
                match stopper_rx.try_recv() {
                    // Stop thread
                    Ok(_) => break,
                    Err(_) => std::thread::sleep(Duration::from_millis(1000)),
                }
            }
            watcher.unwatch(filename.as_ref());
        });
        Ok(FsNotify {
            entity: e.clone(),
            stopper_tx: stopper_tx.clone(),
        })
    }

    pub fn unwatch(&self) {
        trace!("Stop watching {} with id {}", FS_NOTIFY, self.entity.id);
        let _ = self.stopper_tx.send(());
    }
}

impl Disconnectable for FsNotify {
    fn disconnect(&self) {
        self.unwatch();
    }
}

impl Drop for FsNotify {
    fn drop(&mut self) {
        self.disconnect();
    }
}
