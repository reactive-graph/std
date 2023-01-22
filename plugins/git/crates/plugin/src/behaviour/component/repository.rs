use inexor_rgf_model_git::RepositoryProperties::FAST_FORWARD;
use serde_json::Value;

use crate::model::*;
use crate::model_file::FileProperties::FILENAME;
use crate::model_git::GitRepository;
use crate::model_git::RepositoryProperties::BRANCH;
use crate::model_git::RepositoryProperties::FETCH;
use crate::model_git::RepositoryProperties::PUSH;
use crate::model_git::RepositoryProperties::REMOTE_BRANCH;
use crate::model_git::RepositoryProperties::REMOTE_NAME;
use crate::model_http::UrlProperties::URL;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(Repository, RepositoryFactory, RepositoryFsm, RepositoryBehaviourTransitions, RepositoryValidator);

behaviour_validator!(
    RepositoryValidator,
    ReactiveEntityInstance,
    TRIGGER.as_ref(),
    URL.as_ref(),
    FILENAME.as_ref(),
    BRANCH.as_ref(),
    REMOTE_BRANCH.as_ref(),
    REMOTE_NAME.as_ref(),
    TRIGGER.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for RepositoryBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let repository = crate::model_git::Repository::from(self.reactive_instance.clone());
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

impl BehaviourConnect<ReactiveEntityInstance> for RepositoryBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            let repository = crate::model_git::Repository::from(reactive_instance.clone());
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
            let repository = crate::model_git::Repository::from(reactive_instance.clone());
            if repository.exists() {
                repository.git_fetch();
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(FAST_FORWARD.as_ref(), move |fast_forward: &Value| {
            if !fast_forward.as_bool().unwrap_or(false) {
                return;
            }
            let repository = crate::model_git::Repository::from(reactive_instance.clone());
            if repository.exists() {
                repository.git_fast_forward();
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(BRANCH.as_ref(), move |branch: &Value| {
            if let Some(branch) = branch.as_str().map(str::to_string) {
                let repository = crate::model_git::Repository::from(reactive_instance.clone());
                if repository.exists() {
                    repository.git_checkout(branch);
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for RepositoryBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RepositoryBehaviourTransitions {}
