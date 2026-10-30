use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"List all globally installed PNPM packages"####.into(),command: r####"pnpm ls --depth 0"####.into(),tags: vec![r####"pnpm"####.into()].into_iter().collect(),description: Some(r####"Lists all globally installed PNPM packages, avoiding including any package's dependencies in the view."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://pnpm.io/6.x/cli/list#--depth-number"####.into()),author: Some(r####"Tim Smith"####.into()),author_url: Some(r####"https://timsmith.tech"####.into()),shells: vec![].into_iter().collect(),}
}
