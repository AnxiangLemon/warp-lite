use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Generate a service using Nest.js"####.into(),
        command: r####"nest g s {{serviceName}}"####.into(),
        tags: vec![r####"nestcli"####.into()].into_iter().collect(),
        description: Some(r####"Generate a service with the given name."####.into()),
        arguments: vec![Argument {
            name: r####"serviceName"####.into(),
            description: Some(r####"the name for the service to be generated"####.into()),
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
