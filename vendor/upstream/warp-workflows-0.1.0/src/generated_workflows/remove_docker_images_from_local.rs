use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Remove docker images from local system"####.into(),
        command: r####"docker rmi {{image_id}}"####.into(),
        tags: vec![r####"docker"####.into()].into_iter().collect(),
        description: Some(r####"Remove docker images currently stored on your system"####.into()),
        arguments: vec![Argument {
            name: r####"image_id"####.into(),
            description: Some(r####"The container ID of the docker container."####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://docs.docker.com/engine/reference/commandline/rmi/"####.into(),
        ),
        author: Some(r####"guel-codes"####.into()),
        author_url: Some(r####"https://github.com/guel-codes"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
