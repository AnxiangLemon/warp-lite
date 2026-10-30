use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Create Nuxt app"####.into(),
        command: r####"npx nuxi init {{project_name}} --use-{{package_manager}}"####.into(),
        tags: vec![r####"vuejs"####.into(), r####"nuxt"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Create Nuxt application"####.into()),
        arguments: vec![
            Argument {
                name: r####"project_name"####.into(),
                description: Some(r####"Project name"####.into()),
                default_value: Some(r####"my-app"####.into()),
            },
            Argument {
                name: r####"package_manager"####.into(),
                description: Some(r####"The package manager to use for the application"####.into()),
                default_value: Some(r####"npm"####.into()),
            },
        ]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://nuxt.com/docs/getting-started/installation"####.into()),
        author: Some(r####"Richard Manzoli"####.into()),
        author_url: Some(r####"https://github.com/manzolidev"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
