use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_model_git::RepositoryProperties::FAST_FORWARD;

use reactive_graph_model_file::FileProperties::FILENAME;
use reactive_graph_model_git::GitRepository;
use reactive_graph_model_git::RepositoryProperties::BRANCH;
use reactive_graph_model_git::RepositoryProperties::FETCH;
// use reactive_graph_model_git::RepositoryProperties::PUSH;
use reactive_graph_model_git::RepositoryProperties::REMOTE_BRANCH;
use reactive_graph_model_git::RepositoryProperties::REMOTE_NAME;
use reactive_graph_model_http::UrlProperties::URL;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;

entity_behaviour!(Repository, RepositoryFactory, RepositoryFsm, RepositoryBehaviourTransitions, RepositoryValidator);

behaviour_validator!(
    RepositoryValidator,
    Uuid,
    ReactiveEntity,
    TRIGGER.as_ref(),
    URL.as_ref(),
    FILENAME.as_ref(),
    BRANCH.as_ref(),
    REMOTE_BRANCH.as_ref(),
    REMOTE_NAME.as_ref(),
    TRIGGER.as_ref()
);

impl BehaviourInit<Uuid, ReactiveEntity> for RepositoryBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let repository = reactive_graph_model_git::Repository::from(self.reactive_instance.clone());
        if repository.as_bool(TRIGGER).unwrap_or(false) {
            if repository.exists() {
                repository.git_fetch_and_fast_forward();
            } else {
                repository.git_clone();
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for RepositoryBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            let repository = reactive_graph_model_git::Repository::from(reactive_instance.clone());
            if repository.exists() {
                repository.git_fetch_and_fast_forward();
            } else {
                repository.git_clone();
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(FETCH.as_ref(), move |fetch: &Value| {
            if !fetch.as_bool().unwrap_or(false) {
                return;
            }
            let repository = reactive_graph_model_git::Repository::from(reactive_instance.clone());
            if repository.exists() {
                repository.git_fetch();
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(FAST_FORWARD.as_ref(), move |fast_forward: &Value| {
            if !fast_forward.as_bool().unwrap_or(false) {
                return;
            }
            let repository = reactive_graph_model_git::Repository::from(reactive_instance.clone());
            if repository.exists() {
                repository.git_fast_forward();
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(BRANCH.as_ref(), move |branch: &Value| {
            if let Some(branch) = branch.as_str().map(str::to_string) {
                let repository = reactive_graph_model_git::Repository::from(reactive_instance.clone());
                if repository.exists() {
                    repository.git_checkout(branch);
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for RepositoryBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for RepositoryBehaviourTransitions {}
