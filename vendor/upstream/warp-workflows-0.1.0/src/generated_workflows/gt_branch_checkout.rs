use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Graphite - Checkout a branch"####.into(),command: r####"gt bco {{branch_name}}"####.into(),tags: vec![r####"graphite"####.into()].into_iter().collect(),description: Some(r####"Alias `bco`. Switch to or checkout a branch. Similar to `git checkout <branch_name>`"####.into()),arguments: vec![Argument {name: r####"branch_name"####.into(),description: Some(r####"The name of the branch that you want to checkout"####.into()),default_value: Some(r####"main"####.into()),}].into_iter().collect(),source_url: Some(r####"https://graphite.dev/docs/command-reference"####.into()),author: Some(r####"graphite"####.into()),author_url: Some(r####"https://graphite.dev/"####.into()),shells: vec![].into_iter().collect(),}
}
