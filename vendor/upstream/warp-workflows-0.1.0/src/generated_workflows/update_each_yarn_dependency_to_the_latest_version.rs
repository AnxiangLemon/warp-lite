use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Update each Yarn dependency to the latest version"####.into(),command: r####"yarn upgrade-interactive --latest"####.into(),tags: vec![r####"yarn"####.into()].into_iter().collect(),description: Some(r####"Upgrade each yarn dependency by using the Yarn upgrade tool in interactive mode."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/62650640/yarn-how-do-i-update-each-dependency-in-package-json-to-the-latest-version"####.into()),author: Some(r####"cybercoder"####.into()),author_url: Some(r####"https://stackoverflow.com/users/1896107/cybercoder"####.into()),shells: vec![].into_iter().collect(),}
}
