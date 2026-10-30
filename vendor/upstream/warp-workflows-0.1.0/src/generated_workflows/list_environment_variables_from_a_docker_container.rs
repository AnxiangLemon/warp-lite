use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"List environment variables from a Docker container"####.into(),command: r####"docker exec container env"####.into(),tags: vec![r####"docker"####.into()].into_iter().collect(),description: Some(r####"Lists all environment variables from a Docker container."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/34051747/get-environment-variable-from-docker-container"####.into()),author: Some(r####"aisbaa"####.into()),author_url: Some(r####"https://stackoverflow.com/users/698512/aisbaa"####.into()),shells: vec![].into_iter().collect(),}
}
