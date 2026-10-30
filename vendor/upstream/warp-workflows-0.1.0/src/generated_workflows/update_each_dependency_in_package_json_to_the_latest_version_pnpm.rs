use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Update each dependency in package.json to the latest version (PNPM)"####.into(),command: r####"npm config --global set strict-peer-dependencies=false && pnpm update --latest"####.into(),tags: vec![r####"pnpm"####.into()].into_iter().collect(),description: Some(r####"Sets strict-peer-dependencies to false globally and updates each dependency in package.json to the latest version."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/pnpm/pnpm/issues/4651"####.into()),author: Some(r####"Tim Smith"####.into()),author_url: Some(r####"https://timsmith.tech"####.into()),shells: vec![].into_iter().collect(),}
}
