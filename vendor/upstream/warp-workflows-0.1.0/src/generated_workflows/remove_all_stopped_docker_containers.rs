use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Remove all stopped Docker containers"####.into(),command: r####"docker container prune"####.into(),tags: vec![r####"docker"####.into()].into_iter().collect(),description: Some(r####"Removes all stopped containers. To clean up all unused containers, networks, images and volumes in one command, run `docker system prune`."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/17236796/how-to-remove-old-docker-containers"####.into()),author: Some(r####"Ken Cochrane"####.into()),author_url: Some(r####"https://stackoverflow.com/users/356788/ken-cochrane"####.into()),shells: vec![].into_iter().collect(),}
}
