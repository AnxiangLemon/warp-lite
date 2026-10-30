use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Run a Turborepo pipeline with a package filter"####.into(),
        command: r####"npx turbo run {{pipelines}} --filter={{filter}}"####.into(),
        tags: vec![r####"turborepo"####.into()].into_iter().collect(),
        description: None,
        arguments: vec![
            Argument {
                name: r####"pipelines"####.into(),
                description: Some(r####"The pipelines to run."####.into()),
                default_value: Some(r####"start"####.into()),
            },
            Argument {
                name: r####"filter"####.into(),
                description: Some(r####"The package filter to use."####.into()),
                default_value: Some(r####"*"####.into()),
            },
        ]
        .into_iter()
        .collect(),
        source_url: None,
        author: Some(r####"Wyatt-Stanke"####.into()),
        author_url: Some(r####"https://github.com/Wyatt-Stanke"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
