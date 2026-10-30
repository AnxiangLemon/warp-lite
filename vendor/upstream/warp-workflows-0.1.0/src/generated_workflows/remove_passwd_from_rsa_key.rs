use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Remove password from private RSA key"####.into(),
        command: r####"openssl rsa -in {{in_pass_key}} -out {{out_key}}"####.into(),
        tags: vec![r####"ssl"####.into(), r####"openssl"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Remove password from private RSA key."####.into()),
        arguments: vec![
            Argument {
                name: r####"in_pass_key"####.into(),
                description: Some(r####"Input RSA key protected with password."####.into()),
                default_value: Some(r####"in.key"####.into()),
            },
            Argument {
                name: r####"out_key"####.into(),
                description: Some(r####"Output RSA key without password."####.into()),
                default_value: Some(r####"out.key"####.into()),
            },
        ]
        .into_iter()
        .collect(),
        source_url: None,
        author: Some(r####"Petr Stepan (Deworn)"####.into()),
        author_url: Some(r####"https://github.com/deworn"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
