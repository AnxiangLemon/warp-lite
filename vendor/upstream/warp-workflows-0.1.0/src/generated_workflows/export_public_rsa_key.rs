use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Export public key from private RSA key"####.into(),
        command: r####"openssl rsa -in {{in_key}} -pubout -out {{out_pub_key}}"####.into(),
        tags: vec![r####"ssl"####.into(), r####"openssl"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Export public part of the key from private RSA key."####.into()),
        arguments: vec![
            Argument {
                name: r####"in_key"####.into(),
                description: Some(r####"Input private RSA key."####.into()),
                default_value: Some(r####"in.key"####.into()),
            },
            Argument {
                name: r####"out_pub_key"####.into(),
                description: Some(r####"Output key which contains only public part."####.into()),
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
