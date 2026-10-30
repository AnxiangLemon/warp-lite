use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Opens a Bash shell into Kubernetes pod"####.into(),
        command: r####"kubectl -n {{namespace}} exec -it {{pod_name}} -- /bin/sh"####.into(),
        tags: vec![r####"kubectl"####.into()].into_iter().collect(),
        description: None,
        arguments: vec![
            Argument {
                name: r####"namespace"####.into(),
                description: Some(r####"The namespace of the pod"####.into()),
                default_value: Some(r####"default"####.into()),
            },
            Argument {
                name: r####"pod_name"####.into(),
                description: Some(r####"The pod into which to exec"####.into()),
                default_value: None,
            },
        ]
        .into_iter()
        .collect(),
        source_url: None,
        author: None,
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}
