use futures;

use crate::types::api;

#[allow(unused)]
#[derive(Clone)]
pub struct Node {}

// ------------------------------------------------------------------------------------------------
// Kubelet.
//
// * Receives Pod specs, reconciles with current known state of Pods.
// * Publishes updates back to master any time a Pod's status changes.
// * Maintains a v1/Node, publishes any updates back to master.
//
// Pod lifecycle:
// * Scheduler schedules Pod to run on some node somewhere. It updates status in the DB.
// * Dispatches to run on a node somewhere.
// * Node will publish updates until the master unschedules it/it disappears/whatever. In that
//   case, no further updates are accepted.
// ------------------------------------------------------------------------------------------------

pub trait MasterConnection {
    fn update_node<'a>(&self, node: &'a Node);
    fn update_pod<'a>(&self, pod: &'a api::core::v1::Pod);
}

#[allow(unused)]
pub enum PodPhase {
    Pending,
    Running,
    Succeeded,
    Failed,
    Unknown,
}

pub trait PodProvider {
    fn fetch(&self, pod: api::core::v1::Pod);
    fn run(&self, pod: api::core::v1::Pod);
    fn kill<'a>(&self, namespace: &'a str, name: &'a str);
}

pub trait Kubelet {
    fn run(&self) -> futures::Empty<(), ()>;
    fn register_pod(&self, pod: api::core::v1::Pod); // TODO: Return result.
    fn deregister_pod<'a>(&self, namespace: &'a str, name: &'a str);
}

// ------------------------------------------------------------------------------------------------
// State manager. Keeps track of resource definitions.
//
// * Continuously receives updates about the current state of the cluster (spec, status, etc.)
// * Updates specs/live versions in DB when requested.
// * Produces a state snapshot for, e.g., scheduling purposes.
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
#[derive(Clone)]
struct State {
    unscheduled_pods: Vec<api::core::v1::Pod>,
    snapshot: im::HashMap<String, api::core::v1::Pod>,
}

trait StateManager {
    fn get<T>(&self, namespace: Option<String>, name: String) -> T;
    fn list<T>(&self, namespace: Option<String>) -> Vec<T>;
    fn watch<T>(&self, namespace: Option<String>); // TODO: enumerable?

    fn update_spec<T>(&self, resource: Option<T>);
    fn update_live<T>(&self, resource: Option<T>);

    fn snapshot(&self, state: State);
}

// ------------------------------------------------------------------------------------------------
// Scheduler. Depends on StateManager.
//
// * On any update on v1/Node status, or on any update for v1/Pod, decide which Pods should run
//   where.
// * Scheduler passed state snapshot, makes scheduling decision optimistically. Success is a commit
//   in the DB + return, at which point Kubelet picks up change and attempts to reconcile. Failure
//   causes master to re-try scheduling.
// ------------------------------------------------------------------------------------------------

trait Scheduler {
    fn schedule(&self, state: State);
}

// ------------------------------------------------------------------------------------------------
// Master. Depends on StateManager and Scheduler. "Stateless" in the sense that the gold standard
// is whatever is in the DB, and if the master dies, it can simply come back up, re-start DB watch,
// and continue about its business.
//
// * Connects to DB.
// * Connects to Kubelets. (NOT the reverse; avoid the thundering heard.)
// * Receives updates from Kubelets (v1/Node and v1/Pod, which give us a picture of health and
//   resource utilization), pushes to StateManager.
// * Watches DB for changes, pushes updates to StateManager.
// * Consults scheduler to schedule Pods, based on state snapshot from StateManager.
//
// OUT OF SCOPE:
// * Load balancing, networking, namespace quotas, etc.
// * Scheduler schedules containers. More complexity means less reliability at scale.
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
struct Master {}
