use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Update PNPM to the latest version"####.into(),
        command: r####"pnpm update -g pnpm"####.into(),
        tags: vec![r####"pnpm"####.into()].into_iter().collect(),
        description: Some(r####"Updates pnpm to the latest version."####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://pnpm.io/cli/update"####.into()),
        author: Some(r####"Tim Smith"####.into()),
        author_url: Some(r####"https://timsmith.tech"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
