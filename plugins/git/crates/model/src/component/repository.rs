use git2::build::CheckoutBuilder;
use git2::build::RepoBuilder;
use git2::AutotagOption;
use git2::FetchOptions;
use git2::RemoteCallbacks;
use git2::Repository;

use crate::TransferProgress;
use crate::NAMESPACE_GIT;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::component_behaviour_ty;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_model_file::FilePath;
use reactive_graph_model_http::Url;

properties!(
    RepositoryProperties,
    (BRANCH, "branch", ""),
    (REMOTE_NAME, "remote_name", ""),
    (REMOTE_BRANCH, "remote_branch", ""),
    (FETCH, "fetch", ""),
    (FAST_FORWARD, "fast_forward", ""),
    (PUSH, "push", "")
);

component_ty!(COMPONENT_REPOSITORY, NAMESPACE_GIT, COMPONENT_NAME_REPOSITORY, "repository");
behaviour_ty!(BEHAVIOUR_REPOSITORY, NAMESPACE_GIT, BEHAVIOUR_NAME_REPOSITORY, "repository");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_REPOSITORY, COMPONENT_REPOSITORY, BEHAVIOUR_REPOSITORY);

component_model!(
    ComponentRepository,
    data branch string,
    data remote_name string,
    data remote_branch string,
);

pub trait GitRepository: ComponentRepository + TransferProgress + FilePath + Url {
    fn get_reference_name(&self) -> Option<String> {
        self.get_remote_branch().map(|remote_branch| format!("refs/heads/{}", remote_branch))
    }

    fn exists(&self) -> bool {
        self.get_path().and_then(|path| Repository::open(path).ok()).is_some()
    }

    fn open(&self) -> Option<Repository> {
        self.get_path().and_then(|path| Repository::open(path).ok())
    }

    fn git_fetch_and_fast_forward(&self) {
        self.git_fetch();
        self.git_fast_forward();
    }

    fn git_fetch(&self) {
        let Some(remote_name) = self.get_remote_name() else {
            return;
        };
        let Some(remote_branch) = self.get_remote_branch() else {
            return;
        };
        let Some(repository) = self.open() else {
            return;
        };
        let Ok(mut remote) = repository.find_remote(&remote_name) else {
            return;
        };
        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.transfer_progress(|stats| {
            self.total_objects(stats.total_objects() as u64);
            self.received_objects(stats.received_objects() as u64);
            self.received_bytes(stats.received_bytes() as u64);
            self.local_objects(stats.local_objects() as u64);
            self.total_deltas(stats.total_deltas() as u64);
            self.indexed_deltas(stats.indexed_deltas() as u64);
            self.indexed_objects(stats.indexed_objects() as u64);
            true
        });
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(remote_callbacks).download_tags(AutotagOption::All);
        let Ok(_result) = remote.fetch(&[remote_branch], Some(&mut fetch_options), None) else {
            return;
        };
    }

    fn git_fast_forward(&self) {
        let Some(repository) = self.open() else {
            return;
        };
        let Ok(fetch_head) = repository.find_reference("FETCH_HEAD") else {
            return;
        };
        let Ok(fetch_commit) = repository.reference_to_annotated_commit(&fetch_head) else {
            return;
        };
        let Some(reference_name) = self.get_reference_name() else {
            return;
        };
        let Ok(mut reference_head) = repository.find_reference(&reference_name) else {
            return;
        };
        let name = match reference_head.name() {
            Some(s) => s.to_string(),
            None => String::from_utf8_lossy(reference_head.name_bytes()).to_string(),
        };
        let msg = format!("Fast-Forward: Setting {} to id: {}", name, fetch_commit.id());
        if reference_head.set_target(fetch_commit.id(), &msg).is_err() {
            return;
        }
        if repository.set_head(&name).is_err() {
            return;
        }
        if repository.checkout_head(Some(git2::build::CheckoutBuilder::default().force())).is_err() {
            return;
        }
    }

    fn git_clone(&self) {
        let Some(local_path) = self.get_path() else {
            return;
        };
        let Some(url) = self.get_url() else {
            return;
        };
        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.transfer_progress(|stats| {
            self.total_objects(stats.total_objects() as u64);
            self.received_objects(stats.received_objects() as u64);
            self.received_bytes(stats.received_bytes() as u64);
            self.local_objects(stats.local_objects() as u64);
            self.total_deltas(stats.total_deltas() as u64);
            self.indexed_deltas(stats.indexed_deltas() as u64);
            self.indexed_objects(stats.indexed_objects() as u64);
            true
        });
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(remote_callbacks);

        let mut checkout_builder = CheckoutBuilder::new();
        checkout_builder.progress(|_path, _cur, _total| {});

        let Ok(_repository) = RepoBuilder::new()
            .fetch_options(fetch_options)
            .with_checkout(checkout_builder)
            .clone(url.as_str(), &local_path)
        else {
            return;
        };
    }

    fn git_checkout(&self, branch_name: String) {
        let Some(repository) = self.open() else {
            return;
        };
        let Ok(head) = repository.head() else {
            return;
        };
        let Some(oid) = head.target() else {
            return;
        };
        let Ok(commit) = repository.find_commit(oid) else {
            return;
        };
        let Ok(_branch) = repository.branch(&branch_name, &commit, false) else {
            return;
        };
        let Ok(obj) = repository.revparse_single(&("refs/heads/".to_owned() + &branch_name)) else {
            return;
        };
        let _ = repository.checkout_tree(&obj, None);
        let _ = repository.set_head(&("refs/heads/".to_owned() + &branch_name));
    }
}
