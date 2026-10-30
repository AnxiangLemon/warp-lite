use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Update Composer dependencies to their latest versions"####.into(),
        command: r####"composer update"####.into(),
        tags: vec![r####"composer"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(
            r####"Updating Composer dependencies to their latest versions."####.into(),
        ),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://getcomposer.org/doc/01-basic-usage.md#installing-dependencies"####.into(),
        ),
        author: Some(r####"Rob Mellett"####.into()),
        author_url: Some(r####"https://robmellett.com"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
