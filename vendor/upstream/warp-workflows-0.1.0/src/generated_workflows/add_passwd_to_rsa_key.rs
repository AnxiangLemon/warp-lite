use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Add password to private RSA key using AES256"####.into(),
        command: r####"openssl rsa -aes256 -in {{in_key}} -out {{out_pass_key}}"####.into(),
        tags: vec![r####"ssl"####.into(), r####"openssl"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Add password to private RSA key using AES256."####.into()),
        arguments: vec![
            Argument {
                name: r####"in_key"####.into(),
                description: Some(r####"Input RSA key without password."####.into()),
                default_value: Some(r####"in.key"####.into()),
            },
            Argument {
                name: r####"out_pass_key"####.into(),
                description: Some(r####"Output RSA key protected with password."####.into()),
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
