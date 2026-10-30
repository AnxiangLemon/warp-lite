use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Uninstall a local PNPM package"####.into(),
        command: r####"pnpm uninstall {{name}}"####.into(),
        tags: vec![r####"pnpm"####.into()].into_iter().collect(),
        description: Some(
            r####"Removes a module from `node_modules` and from `package.json`."####.into(),
        ),
        arguments: vec![Argument {
            name: r####"name"####.into(),
            description: Some(r####"The name of the package to uninstall"####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://pnpm.io/cli/remove"####.into()),
        author: Some(r####"Tim Smith"####.into()),
        author_url: Some(r####"https://timsmith.tech"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
