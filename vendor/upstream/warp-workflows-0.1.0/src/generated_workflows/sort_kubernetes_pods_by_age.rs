use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Sort Kubernetes pods by age"####.into(),command: r####"kubectl get po --sort-by=.status.startTime"####.into(),tags: vec![r####"kubernetes"####.into()].into_iter().collect(),description: Some(r####"Sorts all Kubernetes pods by the pod's start time. To sort by the pod's creation time, specify `.metadata.creationTimestamp` as the value for `--sort-by`."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/45310287/kubernetes-sort-pods-by-age"####.into()),author: Some(r####"vjdhama"####.into()),author_url: Some(r####"https://stackoverflow.com/users/1214341/vjdhama"####.into()),shells: vec![].into_iter().collect(),}
}
