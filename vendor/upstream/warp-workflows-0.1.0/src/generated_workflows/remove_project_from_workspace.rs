use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Remove project from Nx workspace"####.into(),
        command: r####"npx nx generate @nrwl/workspace:remove --projectName={{name}}"####.into(),
        tags: vec![r####"nx"####.into()].into_iter().collect(),
        description: Some(r####"Remove a project from your Nx workspace."####.into()),
        arguments: vec![Argument {
            name: r####"name"####.into(),
            description: Some(r####"The name of the project to remove."####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://nx.dev/packages/workspace/generators/remove#nrwlworkspaceremove"####
                .into(),
        ),
        author: Some(r####"mandarini"####.into()),
        author_url: Some(r####"https://github.com/mandarini"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
