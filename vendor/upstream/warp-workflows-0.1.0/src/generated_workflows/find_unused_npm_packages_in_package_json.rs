use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Find unused npm packages in package.json"####.into(),command: r####"npx depcheck"####.into(),tags: vec![r####"npm"####.into()].into_iter().collect(),description: Some(r####"Runs depcheck to find all unused packages listed in package.json"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/22675725/find-unused-npm-packages-in-package-json"####.into()),author: Some(r####"German Attanasio"####.into()),author_url: Some(r####"https://stackoverflow.com/users/456564/german-attanasio"####.into()),shells: vec![].into_iter().collect(),}
}
