use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Upgrade Yarn to latest version"####.into(),command: r####"npm install --global yarn
npm upgrade --global yarn"####.into(),tags: vec![r####"yarn"####.into()].into_iter().collect(),description: Some(r####"Upgrades the current yarn version to the latest version using the npm package manager."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/49689174/yarn-how-to-upgrade-yarn-version-using-terminal"####.into()),author: Some(r####"Abdul Rahman"####.into()),author_url: Some(r####"https://stackoverflow.com/users/4613254/abdul-rahman"####.into()),shells: vec![].into_iter().collect(),}
}
