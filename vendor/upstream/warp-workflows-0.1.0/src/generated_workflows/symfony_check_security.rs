use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Check security issues in Symfony project dependencies"####.into(),
        command: r####"symfony check:security"####.into(),
        tags: vec![r####"symfony"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Check security issues in Symfony project dependencies"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://symfony.com/doc/current/setup.html#checking-security-vulnerabilities"####
                .into(),
        ),
        author: Some(r####"Łukasz Jakutowicz"####.into()),
        author_url: Some(r####"https://github.com/lukaszjakutowicz"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
