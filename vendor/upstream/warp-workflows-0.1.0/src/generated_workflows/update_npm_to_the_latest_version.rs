use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Update NPM to the latest version"####.into(),command: r####"npm update -g npm"####.into(),tags: vec![r####"npm"####.into()].into_iter().collect(),description: Some(r####"Updates npm to the latest version ."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/6237295/how-can-i-update-nodejs-and-npm-to-the-next-versions"####.into()),author: Some(r####"James"####.into()),author_url: Some(r####"https://stackoverflow.com/users/21677/james"####.into()),shells: vec![].into_iter().collect(),}
}
