use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Test only the affected code in your Nx workspace"####.into(),command: r####"npx nx affected:test"####.into(),tags: vec![r####"nx"####.into()].into_iter().collect(),description: Some(r####"Test only those projects that are affected by the changes in the current Nx workspace."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://nx.dev/using-nx/affected#affected"####.into()),author: Some(r####"mandarini"####.into()),author_url: Some(r####"https://github.com/mandarini"####.into()),shells: vec![].into_iter().collect(),}
}
