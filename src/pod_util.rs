use crate::types::api::core::v1;

impl Default for v1::PodStatus {
    fn default() -> v1::PodStatus {
        v1::PodStatus {
            conditions: None,
            containerStatuses: None,
            hostIP: None,
            initContainerStatuses: None,
            message: None,
            nominatedNodeName: None,
            phase: None,
            podIP: None,
            qosClass: None,
            reason: None,
            startTime: None,
        }
    }
}

#[allow(unused)]
fn pod_initialized_condition(status: String) -> v1::PodCondition {
    v1::PodCondition {
        condition_type: "Initialized".to_string(),
        lastProbeTime: None,
        lastTransitionTime: None, // NOTE: Filled in by the master.
        message: None,
        reason: None,
        status: status,
    }
}

#[allow(unused)]
fn pod_ready_condition(
    status: String,
    reason: Option<String>,
    message: Option<String>,
) -> v1::PodCondition {
    v1::PodCondition {
        condition_type: "Ready".to_string(),
        lastProbeTime: None,
        lastTransitionTime: None, // NOTE: Filled in by the master.
        message: message,
        reason: reason,
        status: status,
    }
}

#[allow(unused)]
fn pod_containers_ready_condition(
    status: String,
    reason: Option<String>,
    message: Option<String>,
) -> v1::PodCondition {
    v1::PodCondition {
        condition_type: "ContainersReady".to_string(),
        lastProbeTime: None,
        lastTransitionTime: None, // NOTE: Filled in by the master.
        message: message,
        reason: reason,
        status: status,
    }
}

// ------------------------------------------------------------------------------------------------
// Getters.
// ------------------------------------------------------------------------------------------------

pub fn get_id<'a>(pod: &'a v1::Pod) -> Option<String> {
    let meta = try_opt!(&pod.metadata);
    let ns = try_opt!(&meta.namespace);
    let name = try_opt!(&meta.name);
    Some(format!("{}/{}", ns, name))
}

// ------------------------------------------------------------------------------------------------
// Transformations.
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
pub fn set_initial_pod_status<'a>(pod: &'a mut v1::Pod) {
    let status = &mut v1::PodStatus {
        ..Default::default()
    };
    let status: &mut v1::PodStatus = match &mut pod.status {
        None => status,
        Some(status) => status,
    };

    let conditions = &mut Vec::new();
    let conditions: &mut Vec<v1::PodCondition> = match &mut status.conditions {
        None => conditions,
        Some(conditions) => conditions,
    };

    // Filter out relevant conditions so that we can replace them.
    conditions.retain(|cond| {
        let cond_type = &cond.condition_type;
        cond_type != "Initialized" && cond_type != "Ready" && cond_type != "ContainersReady"
    });
    conditions.append(&mut vec![
        pod_initialized_condition("True".to_string()),
        pod_ready_condition(
            "False".to_string(),
            Some("ContainersNotReady".to_string()),
            Some("containers with unready status: [nginx]".to_string()),
        ),
        pod_containers_ready_condition(
            "False".to_string(),
            Some("ContainersNotReady".to_string()),
            Some("containers with unready status: [nginx]".to_string()),
        ),
    ]);
}

// pub fn set_initial_pod_status(pod: v1::Pod) {
//     let status = pod.status.unwrap_or(null_pod_status());

//     let conditions = status.conditions.unwrap_or(Vec::new());
//     // let container_statuses = status.conditions.unwrap_or(Vec::new());

//     conditions
//         .iter_mut()
//         .filter(|&condition| {
//             let cond_type = condition.condition_type;
//             cond_type != "Initialized" && cond_type != "Ready" && cond_type != "ContainersReady"
//         })
//         .collect::<Vec<_>>()
//         .append(&mut vec![
//             &mut pod_initialized_condition("True".to_string()),
//             &mut pod_ready_condition(
//                 "False".to_string(),
//                 Some("ContainersNotReady".to_string()),
//                 Some("containers with unready status: [nginx]".to_string()),
//             ),
//             &mut pod_containers_ready_condition(
//                 "False".to_string(),
//                 Some("ContainersNotReady".to_string()),
//                 Some("containers with unready status: [nginx]".to_string()),
//             ),
//         ]);
// }

// {
//   "lastProbeTime": null,
//   "lastTransitionTime": "2018-12-31T05:35:29Z",
//   "status": "True",
//   "type": "Initialized"
// }
// {
//   "lastProbeTime": null,
//   "lastTransitionTime": "2018-12-31T05:35:29Z",
//   "message": "containers with unready status: [nginx]",
//   "reason": "ContainersNotReady",
//   "status": "False",
//   "type": "Ready"
// }
// {
//   "lastProbeTime": null,
//   "lastTransitionTime": null,
//   "message": "containers with unready status: [nginx]",
//   "reason": "ContainersNotReady",
//   "status": "False",
//   "type": "ContainersReady"
// }
