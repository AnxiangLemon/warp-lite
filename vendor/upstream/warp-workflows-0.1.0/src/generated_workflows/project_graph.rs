use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"View the project graph"####.into(),command: r####"npx nx graph"####.into(),tags: vec![r####"nx"####.into()].into_iter().collect(),description: Some(r####"Start the project graph for the current Nx workspace. This will graph the dependencies within your workspace."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://nx.dev/cli/dep-graph#graph"####.into()),author: Some(r####"mandarini"####.into()),author_url: Some(r####"https://github.com/mandarini"####.into()),shells: vec![].into_iter().collect(),}
}
