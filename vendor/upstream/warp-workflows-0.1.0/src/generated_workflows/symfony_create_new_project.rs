use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Create a new Symfony project"####.into(),
        command: r####"symfony new {{project_name}}"####.into(),
        tags: vec![r####"symfony"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Create a new Symfony project"####.into()),
        arguments: vec![Argument {
            name: r####"project_name"####.into(),
            description: Some(r####"Name of project"####.into()),
            default_value: Some(r####"my-app"####.into()),
        }]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://symfony.com/doc/current/setup.html#creating-symfony-applications"####
                .into(),
        ),
        author: Some(r####"Łukasz Jakutowicz"####.into()),
        author_url: Some(r####"https://github.com/lukaszjakutowicz"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
