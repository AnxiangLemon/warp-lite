use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"List all installed Composer packages including their latest version"####.into(),
        command: r####"composer show --latest"####.into(),
        tags: vec![r####"composer"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(
            r####"List all installed Composer packages including their latest version."####.into(),
        ),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://getcomposer.org/doc/03-cli.md#show"####.into()),
        author: Some(r####"Rob Mellett"####.into()),
        author_url: Some(r####"https://robmellett.com"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
