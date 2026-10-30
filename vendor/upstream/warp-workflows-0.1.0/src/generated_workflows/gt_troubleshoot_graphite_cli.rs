use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Graphite - Troubleshoot the Graphite CLI"####.into(),command: r####"gt repo init --reset && gt dev cache --clear"####.into(),tags: vec![r####"graphite"####.into()].into_iter().collect(),description: Some(r####"This combination of commands resets Graphite repo metadata and clears the cache."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://graphite.dev/docs/command-reference"####.into()),author: Some(r####"graphite"####.into()),author_url: Some(r####"https://graphite.dev/"####.into()),shells: vec![].into_iter().collect(),}
}
