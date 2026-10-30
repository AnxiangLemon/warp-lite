use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Start Symfony local web server"####.into(),
        command: r####"symfony server:start"####.into(),
        tags: vec![r####"symfony"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Start Symfony local web server"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://symfony.com/doc/current/setup/symfony_server.html"####.into(),
        ),
        author: Some(r####"Łukasz Jakutowicz"####.into()),
        author_url: Some(r####"https://github.com/lukaszjakutowicz"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
