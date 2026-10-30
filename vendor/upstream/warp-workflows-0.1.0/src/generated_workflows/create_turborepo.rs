use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Create a new Turborepo"####.into(),
        command: r####"npx -y create-turbo@latest {{directory}} --use-{{package_manager}}"####
            .into(),
        tags: vec![r####"turborepo"####.into()].into_iter().collect(),
        description: None,
        arguments: vec![
            Argument {
                name: r####"package_manager"####.into(),
                description: Some(r####"The package manager to use for the Turborepo."####.into()),
                default_value: Some(r####"npm"####.into()),
            },
            Argument {
                name: r####"directory"####.into(),
                description: Some(r####"The directory to use for the Turborepo."####.into()),
                default_value: Some(r####"./"####.into()),
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
