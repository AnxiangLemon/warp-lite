use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Generate an app using Nest.js"####.into(),
        command: r####"nest g co {{appName}}"####.into(),
        tags: vec![r####"nestcli"####.into()].into_iter().collect(),
        description: Some(r####"Generate a app with the given name."####.into()),
        arguments: vec![Argument {
            name: r####"appName"####.into(),
            description: Some(r####"the name for the app to be generated"####.into()),
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
