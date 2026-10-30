use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Graphite - Run a command on every branch within a stack"####.into(),command: r####"gt downstack test"####.into(),tags: vec![r####"graphite"####.into()].into_iter().collect(),description: Some(r####"Alias `dst`. From trunk to the current branch, run the provided command on each branch and aggregate the results."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://graphite.dev/docs/command-reference"####.into()),author: Some(r####"graphite"####.into()),author_url: Some(r####"https://graphite.dev/"####.into()),shells: vec![].into_iter().collect(),}
}
