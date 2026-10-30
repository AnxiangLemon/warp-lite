use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Graphite - Recursively track parent branches"####.into(),command: r####"gt downstack track"####.into(),tags: vec![r####"graphite"####.into()].into_iter().collect(),description: Some(r####"Alias `dstr`. Track a series of untracked branches, by specifying each branch's parent, stopping when you reach a tracked branch."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://graphite.dev/docs/command-reference"####.into()),author: Some(r####"graphite"####.into()),author_url: Some(r####"https://graphite.dev/"####.into()),shells: vec![].into_iter().collect(),}
}
