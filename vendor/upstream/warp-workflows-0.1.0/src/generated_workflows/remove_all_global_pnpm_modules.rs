use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Remove all global pnpm modules"####.into(),
        command: r####"rm -rf ~/.pnpm-store"####.into(),
        tags: vec![r####"pnpm"####.into()].into_iter().collect(),
        description: Some(
            r####"Removes all global pnpm modules by removing the `~/.pnpm-store` folder."####
                .into(),
        ),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://pnpm.io/6.x/uninstall"####.into()),
        author: Some(r####"Tim Smith"####.into()),
        author_url: Some(r####"https://timsmith.tech"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
