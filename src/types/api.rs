use std::collections::HashMap;

pub mod k8s {
    pub trait Resource {
        fn api_version(&self) -> String;
        fn kind(&self) -> String;
    }
}

pub mod core {
    pub mod v1 {
        use super::super::*;

        //
        // Top-level resource types.
        //

        #[derive(Clone)]
        pub struct Pod {
            pub apiVersion: String,
            pub kind: String,
            pub metadata: Option<meta::v1::ObjectMeta>,
            pub status: Option<core::v1::PodStatus>,
        }

        impl k8s::Resource for Pod {
            fn api_version(&self) -> String {self.apiVersion.to_string()}
            fn kind(&self) -> String {self.kind.to_string()}
        }

        //
        // Resource helper types.
        //

        #[derive(Clone)]
        pub struct ContainerState {
            pub running: Option<core::v1::ContainerStateRunning>,
            pub terminated: Option<core::v1::ContainerStateTerminated>,
            pub waiting: Option<core::v1::ContainerStateWaiting>,
        }

        #[derive(Clone)]
        pub struct ContainerStateRunning {
            pub startedAt: Option<String>,
        }

        #[derive(Clone)]
        pub struct ContainerStateTerminated {
            pub containerID: Option<String>,
            pub exitCode: i32,
            pub finishedAt: Option<String>,
            pub message: Option<String>,
            pub reason: Option<String>,
            pub signal: Option<i32>,
            pub startedAt: Option<String>,
        }

        #[derive(Clone)]
        pub struct ContainerStateWaiting {
            pub message: Option<String>,
            pub reason: Option<String>,
        }

        #[derive(Clone)]
        pub struct ContainerStatus {
            pub containerID: Option<String>,
            pub image: String,
            pub imageID: String,
            pub lastState: Option<core::v1::ContainerState>,
            pub name: String,
            pub ready: bool,
            pub restartCount: i32,
            pub state: Option<core::v1::ContainerState>,
        }

        #[derive(Clone)]
        pub struct PodCondition {
            pub condition_type: String,
            pub lastProbeTime: Option<String>,
            pub lastTransitionTime: Option<String>,
            pub message: Option<String>,
            pub reason: Option<String>,
            pub status: String,
        }

        #[derive(Clone)]
        pub struct PodStatus {
            pub conditions: Option<Vec<core::v1::PodCondition>>,
            pub containerStatuses: Option<Vec<core::v1::ContainerStatus>>,
            pub hostIP: Option<String>,
            pub initContainerStatuses: Option<Vec<core::v1::ContainerStatus>>,
            pub message: Option<String>,
            pub nominatedNodeName: Option<String>,
            pub phase: Option<String>,
            pub podIP: Option<String>,
            pub qosClass: Option<String>,
            pub reason: Option<String>,
            pub startTime: Option<String>,
        }

    }
}

pub mod meta {
    pub mod v1 {
        use super::super::*;

        //
        // Top-level resource types.
        //

        #[derive(Clone)]
        pub struct Status {
            pub apiVersion: String,
            pub code: Option<i32>,
            pub details: Option<meta::v1::StatusDetails>,
            pub kind: String,
            pub message: Option<String>,
            pub metadata: Option<meta::v1::ListMeta>,
            pub reason: Option<String>,
            pub status: Option<String>,
        }

        impl k8s::Resource for Status {
            fn api_version(&self) -> String {self.apiVersion.to_string()}
            fn kind(&self) -> String {self.kind.to_string()}
        }

        //
        // Resource helper types.
        //

        #[derive(Clone)]
        pub struct Initializer {
            pub name: String,
        }

        #[derive(Clone)]
        pub struct Initializers {
            pub pending: Vec<meta::v1::Initializer>,
            pub result: Option<meta::v1::Status>,
        }

        #[derive(Clone)]
        pub struct ListMeta {
            pub resourceVersion: Option<String>,
            pub selfLink: Option<String>,
        }

        #[derive(Clone)]
        pub struct ObjectMeta {
            pub annotations: Option<HashMap<String, String>>,
            pub clusterName: Option<String>,
            pub creationTimestamp: Option<String>,
            pub deletionGracePeriodSeconds: Option<i32>,
            pub deletionTimestamp: Option<String>,
            pub finalizers: Option<Vec<String>>,
            pub generateName: Option<String>,
            pub generation: Option<i32>,
            pub initializers: Option<meta::v1::Initializers>,
            pub labels: Option<HashMap<String, String>>,
            pub name: Option<String>,
            pub namespace: Option<String>,
            pub ownerReferences: Option<Vec<meta::v1::OwnerReference>>,
            pub resourceVersion: Option<String>,
            pub selfLink: Option<String>,
            pub uid: Option<String>,
        }

        #[derive(Clone)]
        pub struct OwnerReference {
            pub apiVersion: String,
            pub blockOwnerDeletion: Option<bool>,
            pub controller: Option<bool>,
            pub kind: String,
            pub name: String,
            pub uid: String,
        }

        #[derive(Clone)]
        pub struct StatusCause {
            pub field: Option<String>,
            pub message: Option<String>,
            pub reason: Option<String>,
        }

        #[derive(Clone)]
        pub struct StatusDetails {
            pub causes: Option<Vec<meta::v1::StatusCause>>,
            pub group: Option<String>,
            pub kind: String,
            pub name: Option<String>,
            pub retryAfterSeconds: Option<i32>,
            pub uid: Option<String>,
        }

    }
}

