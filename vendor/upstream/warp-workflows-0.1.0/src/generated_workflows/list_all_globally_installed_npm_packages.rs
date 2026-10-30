use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"List all globally installed NPM packages"####.into(),command: r####"npm list -g --depth=0"####.into(),tags: vec![r####"npm"####.into()].into_iter().collect(),description: Some(r####"Lists all globally installed NPM packages, avoiding including any package's dependencies in the view."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/17937960/how-to-list-npm-user-installed-packages"####.into()),author: Some(r####"aris"####.into()),author_url: Some(r####"https://stackoverflow.com/users/2628879/aris"####.into()),shells: vec![].into_iter().collect(),}
}
