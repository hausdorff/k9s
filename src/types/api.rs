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
            apiVersion: String,
            kind: String,
            metadata: Option<meta::v1::ObjectMeta>,
            status: Option<core::v1::PodStatus>,
        }

        impl k8s::Resource for Pod {
            fn api_version(&self) -> String {
                self.apiVersion.to_string()
            }
            fn kind(&self) -> String {
                self.kind.to_string()
            }
        }

        //
        // Resource helper types.
        //

        #[derive(Clone)]
        pub struct ContainerState {
            running: Option<core::v1::ContainerStateRunning>,
            terminated: Option<core::v1::ContainerStateTerminated>,
            waiting: Option<core::v1::ContainerStateWaiting>,
        }

        #[derive(Clone)]
        pub struct ContainerStateRunning {
            startedAt: Option<String>,
        }

        #[derive(Clone)]
        pub struct ContainerStateTerminated {
            containerID: Option<String>,
            exitCode: i32,
            finishedAt: Option<String>,
            message: Option<String>,
            reason: Option<String>,
            signal: Option<i32>,
            startedAt: Option<String>,
        }

        #[derive(Clone)]
        pub struct ContainerStateWaiting {
            message: Option<String>,
            reason: Option<String>,
        }

        #[derive(Clone)]
        pub struct ContainerStatus {
            containerID: Option<String>,
            image: String,
            imageID: String,
            lastState: Option<core::v1::ContainerState>,
            name: String,
            ready: bool,
            restartCount: i32,
            state: Option<core::v1::ContainerState>,
        }

        #[derive(Clone)]
        pub struct PodCondition {
            lastProbeTime: Option<String>,
            lastTransitionTime: Option<String>,
            message: Option<String>,
            reason: Option<String>,
            status: String,
        }

        #[derive(Clone)]
        pub struct PodStatus {
            conditions: Option<Vec<core::v1::PodCondition>>,
            containerStatuses: Option<Vec<core::v1::ContainerStatus>>,
            hostIP: Option<String>,
            initContainerStatuses: Option<Vec<core::v1::ContainerStatus>>,
            message: Option<String>,
            nominatedNodeName: Option<String>,
            phase: Option<String>,
            podIP: Option<String>,
            qosClass: Option<String>,
            reason: Option<String>,
            startTime: Option<String>,
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
            apiVersion: String,
            code: Option<i32>,
            details: Option<meta::v1::StatusDetails>,
            kind: String,
            message: Option<String>,
            metadata: Option<meta::v1::ListMeta>,
            reason: Option<String>,
            status: Option<String>,
        }

        impl k8s::Resource for Status {
            fn api_version(&self) -> String {
                self.apiVersion.to_string()
            }
            fn kind(&self) -> String {
                self.kind.to_string()
            }
        }

        //
        // Resource helper types.
        //

        #[derive(Clone)]
        pub struct Initializer {
            name: String,
        }

        #[derive(Clone)]
        pub struct Initializers {
            pending: Vec<meta::v1::Initializer>,
            result: Option<meta::v1::Status>,
        }

        #[derive(Clone)]
        pub struct ListMeta {
            resourceVersion: Option<String>,
            selfLink: Option<String>,
        }

        #[derive(Clone)]
        pub struct ObjectMeta {
            annotations: Option<HashMap<String, String>>,
            clusterName: Option<String>,
            creationTimestamp: Option<String>,
            deletionGracePeriodSeconds: Option<i32>,
            deletionTimestamp: Option<String>,
            finalizers: Option<Vec<String>>,
            generateName: Option<String>,
            generation: Option<i32>,
            initializers: Option<meta::v1::Initializers>,
            labels: Option<HashMap<String, String>>,
            name: Option<String>,
            namespace: Option<String>,
            ownerReferences: Option<Vec<meta::v1::OwnerReference>>,
            resourceVersion: Option<String>,
            selfLink: Option<String>,
            uid: Option<String>,
        }

        #[derive(Clone)]
        pub struct OwnerReference {
            apiVersion: String,
            blockOwnerDeletion: Option<bool>,
            controller: Option<bool>,
            kind: String,
            name: String,
            uid: String,
        }

        #[derive(Clone)]
        pub struct StatusCause {
            field: Option<String>,
            message: Option<String>,
            reason: Option<String>,
        }

        #[derive(Clone)]
        pub struct StatusDetails {
            causes: Option<Vec<meta::v1::StatusCause>>,
            group: Option<String>,
            kind: String,
            name: Option<String>,
            retryAfterSeconds: Option<i32>,
            uid: Option<String>,
        }

    }
}
