use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Graphite - Push the parent branches to GitHub"####.into(),command: r####"gt downstack submit"####.into(),tags: vec![r####"graphite"####.into()].into_iter().collect(),description: Some(r####"Alias `dss`.  Idempotently force push all branches from trunk to the current branch to GitHub, creating or updating distinct pull requests for each."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://graphite.dev/docs/command-reference"####.into()),author: Some(r####"graphite"####.into()),author_url: Some(r####"https://graphite.dev/"####.into()),shells: vec![].into_iter().collect(),}
}
