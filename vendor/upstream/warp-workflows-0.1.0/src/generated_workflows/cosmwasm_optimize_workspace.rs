use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Optimize CosmWasm Workspace Project"####.into(),command: r####"docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.10
"####.into(),tags: vec![r####"cosmwasm"####.into()].into_iter().collect(),description: Some(r####"Optimize a CosmWasm project that is organized as a workspace with multiple member contracts. Requires Docker to be installed and running.
"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/CosmWasm/rust-optimizer"####.into()),author: Some(r####"chadoh"####.into()),author_url: Some(r####"https://chadoh.com"####.into()),shells: vec![].into_iter().collect(),}
}
