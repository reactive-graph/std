use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use inexor_rgf_core_model::ReactiveInstance;
use inexor_rgf_core_reactive::BehaviourFsm;
use inexor_rgf_core_reactive::EntityBehaviourStorage;
use log::debug;
use log::info;
use uuid::Uuid;

use crate::behaviour::component::State;
// use crate::behaviour::component::StateDebugger;
// use crate::behaviour::component::ValueDebugger;
use crate::behaviour::component::STATE_BEHAVIOURS;
// use crate::behaviour::component::STATE_DEBUGGER_BEHAVIOURS;
// use crate::behaviour::component::VALUE_DEBUGGER_BEHAVIOURS;
use crate::di::*;
use crate::model::BehaviourTypeId;
use crate::model::ComponentContainer;
use crate::model::NamespacedType;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;

// pub type XEntityBehaviourStorage<T: ReactiveInstance> = DashMap<Uuid, DashMap<BehaviourTypeId, Arc<dyn BehaviourFsm<T> + Send + Sync>>>;

// Arc<dyn BehaviourFsm<ReactiveEntityInstance>>
// pub struct EntityBehaviourStorage(DashMap<Uuid, DashMap<BehaviourTypeId, Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync>>>);
// {
//     behaviours: ,
// }

// impl EntityBehaviourStorage {
//     fn new() -> Self {
//         EntityBehaviourStorage(DashMap::new()) // { behaviours: DashMap::new() }
//     }
//
//     fn insert(
//         &self,
//         id: Uuid,
//         ty: BehaviourTypeId,
//         behaviour: Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync>,
//     ) -> Option<Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync>> {
//         if !self.0.contains_key(&id) {
//             self.0.insert(id, DashMap::new());
//         }
//         if let Some(instance_behaviours) = self.0.get(&id) {
//             return instance_behaviours.value().insert(ty, behaviour); // behaviour); // "".to_owned()
//         }
//         None
//     }
//
//     fn remove(&self, id: Uuid, ty: &BehaviourTypeId) -> Option<(BehaviourTypeId, Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync>)> {
//         if let Some(instance_behaviours) = self.0.get(&id) {
//             return instance_behaviours.value().remove(ty);
//         }
//         None
//     }
// }

#[wrapper]
pub struct StateStorage(DashMap<Uuid, Arc<State>>);

// #[wrapper]
// pub struct EntityBehaviourStorage(DashMap<Uuid, DashMap<BehaviourTypeId, Arc<dyn BehaviourFsm<ReactiveEntityInstance>>>>);

// #[wrapper]
// pub struct ValueDebuggerStorage(RwLock<HashMap<Uuid, Arc<ValueDebugger>>>);
//
// #[wrapper]
// pub struct StateDebuggerStorage(RwLock<HashMap<Uuid, Arc<StateDebugger>>>);

#[provides]
fn create_state_storage() -> StateStorage {
    StateStorage(DashMap::new())
}

// #[provides]
// fn create_entity_behaviour_storage() -> EntityBehaviourStorage {
//     EntityBehaviourStorage(DashMap::new())
// }
//
// #[provides]
// fn create_value_debugger_storage() -> ValueDebuggerStorage {
//     ValueDebuggerStorage(RwLock::new(HashMap::new()))
// }
//
// #[provides]
// fn create_state_debugger_storage() -> StateDebuggerStorage {
//     StateDebuggerStorage(RwLock::new(HashMap::new()))
// }

#[async_trait]
pub trait ValueComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_state(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId);

    // fn create_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId);
    //
    // fn create_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId);

    fn remove_state(&self, entity_instance: Arc<ReactiveEntityInstance>);

    // fn remove_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>);
    //
    // fn remove_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct ValueComponentBehaviourProviderImpl {
    states: StateStorage,
    entity_behaviour_storage: EntityBehaviourStorage,
    // value_debuggers: ValueDebuggerStorage,
    // state_debuggers: StateDebuggerStorage,
}

interfaces!(ValueComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

#[component]
impl ValueComponentBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            states: create_state_storage(),
            // entity_behaviour_storage: create_entity_behaviour_storage(),
            // value_debuggers: create_value_debugger_storage(),
            // state_debuggers: create_state_debugger_storage(),
        }
    }
}

// #[async_trait]
#[provides]
impl ValueComponentBehaviourProvider for ValueComponentBehaviourProviderImpl {
    fn create_state(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) {
        let id = entity_instance.id;
        if STATE_BEHAVIOURS.contains(behaviour_ty) {
            if let Ok(behaviour) = State::new(entity_instance.clone(), behaviour_ty.clone()) {
                // self.states.0.insert(id, Arc::new(behaviour));
                self.entity_behaviour_storage.insert(id, behaviour_ty.clone(), Arc::new(behaviour));
                // entity_instance.add_behaviour(behaviour_ty.clone());
                debug!("Added behaviour {} to entity instance {}", behaviour_ty, entity_instance);
            }
        }
    }

    // fn create_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) {
    //     let id = entity_instance.id;
    //     if let Some(behaviour) = VALUE_DEBUGGER_BEHAVIOURS
    //         .get(behaviour_ty)
    //         .and_then(|function| ValueDebugger::new(entity_instance.clone(), behaviour_ty.clone(), *function).ok())
    //     {
    //         self.value_debuggers.0.write().unwrap().insert(id, Arc::new(behaviour));
    //         entity_instance.add_behaviour(behaviour_ty.clone());
    //         debug!("Added behaviour {} to entity instance {}", &behaviour_ty, entity_instance);
    //     }
    // }
    //
    // fn create_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) {
    //     let id = entity_instance.id;
    //     if let Some(behaviour) = STATE_DEBUGGER_BEHAVIOURS
    //         .get(behaviour_ty)
    //         .and_then(|function| StateDebugger::new(entity_instance.clone(), behaviour_ty.clone(), *function).ok())
    //     {
    //         self.state_debuggers.0.write().unwrap().insert(id, Arc::new(behaviour));
    //         entity_instance.add_behaviour(behaviour_ty.clone());
    //         debug!("Added behaviour {} to entity instance {}", &behaviour_ty, entity_instance);
    //     }
    // }

    fn remove_state(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some((_, behaviour)) = self.entity_behaviour_storage.0.remove(&entity_instance.id) {
            // entity_instance.remove_behaviour(&behaviour.ty);
            debug!("Removed behaviour {} from entity instance {}", behaviour.ty(), entity_instance);
        }
    }

    // fn remove_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>) {
    //     if let Some(behaviour) = self.value_debuggers.0.write().unwrap().remove(&entity_instance.id) {
    //         entity_instance.remove_behaviour(&behaviour.ty);
    //         debug!("Removed behaviour {} from entity instance {}", &behaviour.ty, entity_instance);
    //     }
    // }
    //
    // fn remove_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>) {
    //     if let Some(behaviour) = self.state_debuggers.0.write().unwrap().remove(&entity_instance.id) {
    //         entity_instance.remove_behaviour(&behaviour.ty);
    //         debug!("Removed behaviour {} from entity instance {}", &behaviour.ty, entity_instance);
    //     }
    // }

    fn remove_by_id(&self, id: Uuid) {
        if self.states.0.contains_key(&id) {
            if let Some((_, behaviour)) = self.states.0.remove(&id) {
                // behaviour.entity.remove_behaviour(&behaviour.ty);
                debug!("Removed behaviour {} from entity instance {}", behaviour.ty(), behaviour.get_reactive_instance());
            }
        }
        // if self.value_debuggers.0.write().unwrap().contains_key(&id) {
        //     if let Some(behaviour) = self.value_debuggers.0.write().unwrap().remove(&id) {
        //         behaviour.entity.remove_behaviour(&behaviour.ty);
        //         debug!("Removed behaviour {} from entity instance {}", &behaviour.ty, behaviour.entity);
        //     }
        // }
        // if self.state_debuggers.0.write().unwrap().contains_key(&id) {
        //     if let Some(behaviour) = self.state_debuggers.0.write().unwrap().remove(&id) {
        //         behaviour.entity.remove_behaviour(&behaviour.ty);
        //         debug!("Removed behaviour {} from entity instance {}", &behaviour.ty, behaviour.entity);
        //     }
        // }
    }
}

impl ComponentBehaviourProvider for ValueComponentBehaviourProviderImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        info!("add_behaviours_to_entity {}", &entity_instance);
        for behaviour_ty in STATE_BEHAVIOURS.iter() {
            let namespaced_ty: NamespacedType = behaviour_ty.into();
            let component_ty = namespaced_ty.into();
            info!("add_behaviours_to_entity {} {}", &entity_instance, &component_ty);
            if entity_instance.is_a(&component_ty) && !entity_instance.behaves_as(behaviour_ty) {
                info!("add_behaviours_to_entity -> create_state");
                self.create_state(entity_instance.clone(), behaviour_ty);
            }
        }
        // for behaviour_ty in VALUE_DEBUGGER_BEHAVIOURS.keys() {
        //     let namespaced_ty: NamespacedType = behaviour_ty.into();
        //     let component_ty = namespaced_ty.into();
        //     if entity_instance.is_a(&component_ty) && !entity_instance.behaves_as(behaviour_ty) {
        //         self.create_value_debugger(entity_instance.clone(), behaviour_ty);
        //     }
        // }
        // for behaviour_ty in STATE_DEBUGGER_BEHAVIOURS.keys() {
        //     let namespaced_ty: NamespacedType = behaviour_ty.into();
        //     let component_ty = namespaced_ty.into();
        //     if entity_instance.is_a(&component_ty) && !entity_instance.behaves_as(behaviour_ty) {
        //         self.create_state_debugger(entity_instance.clone(), behaviour_ty);
        //     }
        // }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        info!("add_behaviours_to_entity_component {} {}", &entity_instance, &component.ty);
        let behaviour_ty = NamespacedType::from(&component.ty).into();
        if entity_instance.is_a(&component.ty) && !entity_instance.behaves_as(&behaviour_ty) {
            if STATE_BEHAVIOURS.contains(&behaviour_ty) {
                info!("add_behaviours_to_entity_component -> create_state");
                self.create_state(entity_instance.clone(), &behaviour_ty);
            }
            // if VALUE_DEBUGGER_BEHAVIOURS.contains_key(&behaviour_ty) {
            //     self.create_value_debugger(entity_instance.clone(), &behaviour_ty);
            // }
            // if STATE_DEBUGGER_BEHAVIOURS.contains_key(&behaviour_ty) {
            //     self.create_state_debugger(entity_instance, &behaviour_ty);
            // }
        }
    }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        info!("remove_behaviours_from_entity {}", &entity_instance);
        for behaviour_ty in STATE_BEHAVIOURS.iter() {
            info!("behaviour_ty {}", &behaviour_ty);
            if entity_instance.behaves_as(behaviour_ty) {
                info!("behaviour_ty {} -> remove_state", &behaviour_ty);
                self.remove_state(entity_instance.clone());
            }
        }
        // for behaviour_ty in VALUE_DEBUGGER_BEHAVIOURS.keys() {
        //     if entity_instance.behaves_as(behaviour_ty) {
        //         self.remove_value_debugger(entity_instance.clone());
        //     }
        // }
        // for behaviour_ty in STATE_DEBUGGER_BEHAVIOURS.keys() {
        //     if entity_instance.behaves_as(behaviour_ty) {
        //         self.remove_state_debugger(entity_instance.clone());
        //     }
        // }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        info!("remove_behaviours_from_entity_component {} {}", &entity_instance, &component.ty);
        let behaviour_ty = NamespacedType::from(&component.ty).into();
        if STATE_BEHAVIOURS.contains(&behaviour_ty) && entity_instance.behaves_as(&behaviour_ty) {
            info!("remove_behaviours_from_entity_component -> remove_state");
            self.remove_state(entity_instance.clone());
        }
        // if VALUE_DEBUGGER_BEHAVIOURS.contains_key(&behaviour_ty) && entity_instance.behaves_as(&behaviour_ty) {
        //     self.remove_value_debugger(entity_instance.clone());
        // }
        // if STATE_DEBUGGER_BEHAVIOURS.contains_key(&behaviour_ty) && entity_instance.behaves_as(&behaviour_ty) {
        //     self.remove_state_debugger(entity_instance);
        // }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
