use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Remove all stopped docker container, image and volumes"####.into(),command: r####"docker system prune --all -f && docker volume prune -f"####.into(),tags: vec![r####"docker"####.into()].into_iter().collect(),description: Some(r####"Remove all stopped docker containers, images, and volumes. Be careful when doing this action, all local images & volumes will be removed and can't be reverted"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://docs.docker.com/engine/reference/commandline/system_prune/"####.into()),author: Some(r####"Khanh Chau"####.into()),author_url: Some(r####"https://github.com/khanhx"####.into()),shells: vec![].into_iter().collect(),}
}
