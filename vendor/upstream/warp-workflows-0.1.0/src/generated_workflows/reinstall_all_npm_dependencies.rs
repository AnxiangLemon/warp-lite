use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Reinstall all NPM dependencies"####.into(),command: r####"rm -rf node_modules && npm install"####.into(),tags: vec![r####"npm"####.into()].into_iter().collect(),description: Some(r####"Reinstalls all dependencies by removing the node_modules folder and then reinstalling."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/12866494/how-do-you-reinstall-an-apps-dependencies-using-npm"####.into()),author: Some(r####"himanshu"####.into()),author_url: Some(r####"https://stackoverflow.com/users/737286/himanshu"####.into()),shells: vec![].into_iter().collect(),}
}
