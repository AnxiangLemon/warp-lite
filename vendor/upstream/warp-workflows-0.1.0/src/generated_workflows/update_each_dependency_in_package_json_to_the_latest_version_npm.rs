use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Update each dependency in package.json to the latest version (NPM)"####.into(),command: r####"npm i -g npm-check-updates
ncu -u
npm install"####.into(),tags: vec![r####"npm"####.into()].into_iter().collect(),description: Some(r####"Installs the `npm-check-updates` tool, and runs it to update every package to the latest version."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/16073603/how-to-update-each-dependency-in-package-json-to-the-latest-version"####.into()),author: Some(r####"josh3736"####.into()),author_url: Some(r####"https://stackoverflow.com/users/201952/josh3736"####.into()),shells: vec![].into_iter().collect(),}
}
