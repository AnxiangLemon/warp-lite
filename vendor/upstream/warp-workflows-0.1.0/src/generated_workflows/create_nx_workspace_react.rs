use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Create a new Nx workspace with a React application"####.into(),command: r####"npx create-nx-workspace@latest {{name}} --preset=react"####.into(),tags: vec![r####"nx"####.into()].into_iter().collect(),description: Some(r####"Create a new Nx workspace with a React application using the latest version of Nx."####.into()),arguments: vec![Argument {name: r####"name"####.into(),description: Some(r####"The name of the workspace."####.into()),default_value: None,}].into_iter().collect(),source_url: Some(r####"https://nx.dev/cli/create-nx-workspace"####.into()),author: Some(r####"mandarini"####.into()),author_url: Some(r####"https://github.com/mandarini"####.into()),shells: vec![].into_iter().collect(),}
}
