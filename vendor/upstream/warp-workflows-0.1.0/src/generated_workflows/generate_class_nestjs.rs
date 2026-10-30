use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Generate a class using Nest.js"####.into(),
        command: r####"nest g cl {{className}}"####.into(),
        tags: vec![r####"nestcli"####.into()].into_iter().collect(),
        description: Some(r####"Generate a class with the given name."####.into()),
        arguments: vec![Argument {
            name: r####"className"####.into(),
            description: Some(r####"the name for the class to be generated"####.into()),
            default_value: Some(r####"foo"####.into()),
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://docs.nestjs.com/cli/overview"####.into()),
        author: Some(r####"nagauta"####.into()),
        author_url: Some(r####"https://github.com/nagauta"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
