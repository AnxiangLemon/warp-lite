use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Check website certificate"####.into(),
        command: r####"openssl s_client -connect {{url}}:443 -showcerts"####.into(),
        tags: vec![r####"ssl"####.into(), r####"openssl"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Check certificate of specific web site or URL."####.into()),
        arguments: vec![Argument {
            name: r####"url"####.into(),
            description: Some(r####"URL to check."####.into()),
            default_value: Some(r####""####.into()),
        }]
        .into_iter()
        .collect(),
        source_url: None,
        author: Some(r####"Petr Stepan (Deworn)"####.into()),
        author_url: Some(r####"https://github.com/deworn"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
