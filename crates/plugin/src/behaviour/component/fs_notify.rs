use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use crossbeam::channel::unbounded;
use crossbeam::channel::Sender;
use log::error;
use log::trace;
use notify::Config;
use notify::Event;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use serde_json::json;
use serde_json::Value;
use tokio::runtime::Handle;

use crate::model::*;
use crate::model_file::FileProperties::FILENAME;
use crate::model_file::FsNotifyProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(FsNotify, FsNotifyFactory, FsNotifyFsm, FsNotifyBehaviourTransitions, FsNotifyValidator);

behaviour_validator!(FsNotifyValidator, ReactiveEntityInstance, TRIGGER.as_ref(), FILENAME.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for FsNotifyBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for FsNotifyBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        if let Some(filename) = self.reactive_instance.get(FILENAME).and_then(|v| v.as_str().map(String::from)) {
            let filename = shellexpand::tilde(&filename);
            let path = Path::new(filename.as_ref()).to_owned();

            // TODO: Dieses Behaviour ist entscheidend für die Asset-Library-Funktionalität
            // TODO: Zu lösende Probleme:
            // TODO: - Die Tokio-Runtime muss in die Factory übergeben werden können
            // TODO: - Thread-Stopper muss während des Disconnects aufgerufen werden, den Disconnect kann man aber noch nicht überschreiben.

            // let (stopper_tx, stopper_rx) = unbounded();
            // let (notify_tx, notify_rx) = unbounded();
            //
            // let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
            //     move |result: Result<Event, notify::Error>| {
            //         let _ = notify_tx.send(result);
            //     },
            //     Config::default(),
            // )
            // .map_err(|_| BehaviourConnectFailed {})?;
            // watcher.watch(&path, RecursiveMode::NonRecursive).map_err(|_| BehaviourConnectFailed {})?;
            //
            // TODO: woher bekommt man die runtime?
            //     let reactive_instance = self.reactive_instance.clone();
            //     runtime.spawn(async move {
            //         loop {
            //             if let Ok(Ok(_notify_event)) = notify_rx.try_recv() {
            //                 trace!("{:?} has changed", &path);
            //                 reactive_instance.set(TRIGGER, json!(true));
            //             }
            //             match stopper_rx.try_recv() {
            //                 // Stop thread
            //                 Ok(_) => break,
            //                 Err(_) => std::thread::sleep(Duration::from_millis(1000)),
            //             }
            //         }
            //         if let Err(err) = watcher.unwatch(&path) {
            //             error!("Failed to unwatch {:?}: {:?}", &path, err);
            //         }
            //     });
        }

        // let reactive_instance = self.reactive_instance.clone();
        // self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |v: &Value| {
        //     if !v.is_boolean() {
        //         // Invalid type
        //         return;
        //     }
        //     if !v.as_bool().unwrap() {
        //         // Counter only on true (= high)
        //         return;
        //     }
        //     match reactive_instance.get(RESULT).and_then(|v| v.as_i64()) {
        //         Some(current_value) => {
        //             reactive_instance.set(RESULT, json!(current_value + 1));
        //         }
        //         None => {
        //             reactive_instance.set(RESULT, json!(0));
        //         }
        //     }
        // });
        Ok(())
    }
}

// impl BehaviourDisconnect<ReactiveEntityInstance> for FsNotifyBehaviourTransitions {
//     fn disconnect(&self) -> Result<(), BehaviourDisconnectFailed> {
//         // TODO: stopper -> stop thread
//         OK(())
//     }
// }

impl BehaviourShutdown<ReactiveEntityInstance> for FsNotifyBehaviourTransitions {}

impl BehaviourTransitions<ReactiveEntityInstance> for FsNotifyBehaviourTransitions {}

//
// use crate::behaviour::component::FsNotifyProperties;
// use crate::model::PropertyInstanceSetter;
// use crate::model::ReactiveEntityInstance;
// use crate::reactive::entity::Disconnectable;
// use crate::reactive::BehaviourCreationError;
//
// pub const FS_NOTIFY: &str = "fs_notify";
//
// pub struct FsNotify {
//     pub entity: Arc<ReactiveEntityInstance>,
//
//     stopper_tx: Sender<()>,
// }
//
// impl FsNotify {
//     pub fn new(e: Arc<ReactiveEntityInstance>, runtime: &Handle) -> Result<FsNotify, BehaviourCreationError> {
//         let filename = e
//             .properties
//             .get(FsNotifyProperties::FILENAME.as_ref())
//             .ok_or(BehaviourCreationError)?
//             .as_string()
//             .ok_or(BehaviourCreationError)?;
//         let filename = shellexpand::tilde(&filename);
//         let path = Path::new(filename.as_ref()).to_owned();
//         let _ = e.properties.get(FsNotifyProperties::TRIGGER.as_ref()).ok_or(BehaviourCreationError)?;
//
//         let (stopper_tx, stopper_rx) = unbounded();
//         let (notify_tx, notify_rx) = unbounded();
//
//         let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
//             move |result: std::result::Result<Event, notify::Error>| {
//                 let _ = notify_tx.send(result);
//             },
//             Config::default(),
//         )
//         .map_err(|_| BehaviourCreationError)?;
//         watcher.watch(&path, RecursiveMode::NonRecursive).map_err(|_| BehaviourCreationError)?;
//
//         let entity = e.clone();
//         runtime.spawn(async move {
//             loop {
//                 if let Ok(Ok(_notify_event)) = notify_rx.try_recv() {
//                     trace!("{:?} has changed", &path);
//                     entity.set(FsNotifyProperties::TRIGGER, json!(true));
//                 }
//                 match stopper_rx.try_recv() {
//                     // Stop thread
//                     Ok(_) => break,
//                     Err(_) => std::thread::sleep(Duration::from_millis(1000)),
//                 }
//             }
//             if let Err(err) = watcher.unwatch(&path) {
//                 error!("Failed to unwatch {:?}: {:?}", &path, err);
//             }
//         });
//         Ok(FsNotify { entity: e, stopper_tx })
//     }
//
//     pub fn unwatch(&self) {
//         trace!("Stop watching {} with id {}", FS_NOTIFY, self.entity.id);
//         let _ = self.stopper_tx.send(());
//     }
// }
//
// impl Disconnectable for FsNotify {
//     fn disconnect(&self) {
//         self.unwatch();
//     }
// }
//
// impl Drop for FsNotify {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }
