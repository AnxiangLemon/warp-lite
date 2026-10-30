use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Check all the requirements to develop and run Symfony application locally"####.into(),command: r####"symfony check:requirements"####.into(),tags: vec![r####"symfony"####.into(),r####"php"####.into()].into_iter().collect(),description: Some(r####"Check all the tools you need to develop and run your Symfony application locally"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://symfony.com/doc/current/setup.html#technical-requirements"####.into()),author: Some(r####"Łukasz Jakutowicz"####.into()),author_url: Some(r####"https://github.com/lukaszjakutowicz"####.into()),shells: vec![].into_iter().collect(),}
}
