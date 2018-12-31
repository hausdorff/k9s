#[macro_use]
extern crate try_opt;

use futures::{Async, Future, Poll};
use std::collections::HashMap;
use std::thread;

mod pod_util;
mod types;

use crate::types::api::core;
use crate::types::interfaces;

// ------------------------------------------------------------------------------------------------
// Kubelet event loop implementation.
// ------------------------------------------------------------------------------------------------

struct KubeletEvents;

impl Future for KubeletEvents {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::NotReady)
    }
}

// ------------------------------------------------------------------------------------------------
// Kubelet implementation.
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
struct LocalKubelet<'a> {
    conn: &'a (interfaces::MasterConnection + 'a),
    docker: &'a (interfaces::PodProvider + 'a),

    pods: HashMap<String, core::v1::Pod>,
}

impl<'a> LocalKubelet<'a> {
    #[allow(unused)]
    fn new(
        conn: &'a interfaces::MasterConnection,
        docker: &'a interfaces::PodProvider,
    ) -> LocalKubelet<'a> {
        LocalKubelet {
            conn: conn,
            docker: docker,

            pods: HashMap::new(),
        }
    }

    fn create(&self, pod: core::v1::Pod) {
        let mut pod = pod;

        //
        // Attempt to get pod ID. Log if no ID.
        //

        let pod_id = match pod_util::get_id(&pod) {
            None => return, // TODO Log.
            Some(pod_id) => pod_id,
        };

        //
        // MUT: Set pod init status if this is a new pod.
        //

        if self.pods.contains_key(&pod_id) {
            pod_util::set_initial_pod_status(&mut pod);
            self.conn.update_pod(&pod);
        };

        // - Pod phase already set to pending.
        // - Then, for each container:
        //   - Download container.
        //   - Start pod, mark pod as running.
        //   - Subscribe to updates.
        // - Increment resource version.

        // self.conn.update_pod(pod)
    }

    fn delete<'b>(&self, namespace: &'b str, name: &'b str) {
        // self.docker.kill(namespace, name);
    }
}

impl<'a> interfaces::Kubelet for LocalKubelet<'a> {
    fn run(&self) -> futures::Empty<(), ()> {
        thread::spawn(move || {
            // * Maintain a set of specs, and a set of live objects.
            // * Continuously loop over all specs, checking whether there is a difference between live
            //   and spec.
        });

        futures::empty()
    }
    fn register_pod(&self, pod: core::v1::Pod) {}
    fn deregister_pod<'b>(&self, namespace: &'b str, name: &'b str) {}
}

// ------------------------------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPodProvider {}

    impl interfaces::PodProvider for MockPodProvider {
        fn fetch(&self, pod: core::v1::Pod) {}
        fn run(&self, pod: core::v1::Pod) {}
        fn kill<'a>(&self, namespace: &'a str, name: &'a str) {}
    }

    struct MockMasterConnection {}

    impl interfaces::MasterConnection for MockMasterConnection {
        fn update_node<'a>(&self, node: &'a interfaces::Node) {}
        fn update_pod<'a>(&self, pod: &'a core::v1::Pod) {}
    }

    // #[test]
    // fn test_add() {
    //     let conn = MockMasterConnection {};
    //     let pod_provider = MockPodProvider {};
    //     let _ = LocalKubelet::new(&conn, &pod_provider);
    //     // assert_eq!(add(1, 2), 3);
    // }
}

// ------------------------------------------------------------------------------------------------
// Main loop.
// ------------------------------------------------------------------------------------------------

fn main() {
    println!("Hello")
}
