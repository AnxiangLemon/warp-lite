use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Create Next.js App"####.into(),
        command: r####"npx create-next-app {{project_name}} --use-{{package_manager}}"####.into(),
        tags: vec![r####"react"####.into(), r####"nextjs"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Create a new Next.js application"####.into()),
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
        source_url: Some(r####"https://nextjs.org/docs/api-reference/create-next-app"####.into()),
        author: Some(r####"Lukas Varkalis"####.into()),
        author_url: Some(r####"https://github.com/lukasvarkalis"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
