use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"List processes at port"####.into(),
        command: r####"lsof -i:{{port}}"####.into(),
        tags: vec![r####"macos"####.into(), r####"shell"####.into()]
            .into_iter()
            .collect(),
        description: Some(
            r####"List processes at a port to see if there are running processes."####.into(),
        ),
        arguments: vec![Argument {
            name: r####"port"####.into(),
            description: Some(r####"The port that should be checked"####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: None,
        author: Some(r####"Maurice Gerhardt"####.into()),
        author_url: Some(r####"https://github.com/heymage"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
