use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Reinstall all PNPM dependencies"####.into(),command: r####"rm -rf node_modules && pnpm install"####.into(),tags: vec![r####"pnpm"####.into()].into_iter().collect(),description: Some(r####"Reinstalls all dependencies by removing the node_modules folder and then reinstalling."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://pnpm.io/cli/install"####.into()),author: Some(r####"Tim Smith"####.into()),author_url: Some(r####"https://timsmith.tech"####.into()),shells: vec![].into_iter().collect(),}
}
