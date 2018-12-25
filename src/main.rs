mod types;

use crate::types::api;
use crate::types::interfaces;

#[allow(unused)]
struct LocalKubelet<'a> {
    conn: &'a (interfaces::MasterConnection + 'a),
    docker: &'a (interfaces::PodProvider + 'a),
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
        }
    }
}

impl<'a> interfaces::Kubelet for LocalKubelet<'a> {
    #[allow(unused_variables)]
    fn create(&self, pod: api::core::v1::Pod) {
        // - Set pod phase to pending.
        // - Then, for each container:
        //   - Download container.
        //   - Start pod, mark pod as running.
        //   - Subscribe to updates.

        self.conn.update_pod(pod)
    }

    #[allow(unused_variables)]
    fn update(&self, live: Option<api::core::v1::Pod>, spec: Option<api::core::v1::Pod>) {}
}

// ------------------------------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    struct MockPodProvider {}

    impl interfaces::PodProvider for MockPodProvider {
        fn fetch(&self, pod: api::core::v1::Pod) {}
        fn run(&self, pod: api::core::v1::Pod) {}
    }

    #[allow(unused)]
    struct MockMasterConnection {}

    impl interfaces::MasterConnection for MockMasterConnection {
        fn update_node(&self, node: interfaces::Node) {}
        fn update_pod(&self, pod: api::core::v1::Pod) {}
    }

    // #[test]
    // fn test_add() {
    //     assert_eq!(add(1, 2), 3);
    // }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}

// ------------------------------------------------------------------------------------------------
// Main loop.
// ------------------------------------------------------------------------------------------------

fn main() {
    println!("Hello")
}
