use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"List docker images from local system"####.into(),
        command: r####"docker image ls"####.into(),
        tags: vec![r####"docker"####.into()].into_iter().collect(),
        description: Some(
            r####"Lists all docker images currently stored on your system"####.into(),
        ),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://docs.docker.com/engine/reference/commandline/image_ls/"####.into(),
        ),
        author: Some(r####"guel-codes"####.into()),
        author_url: Some(r####"https://github.com/guel-codes"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
