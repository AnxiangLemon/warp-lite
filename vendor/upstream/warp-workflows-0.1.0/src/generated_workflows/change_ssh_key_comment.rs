use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Change an SSH key comment"####.into(),
        command: r####"ssh-keygen -c -C "{{new_comment}}" -f {{ssh_key_path}}"####.into(),
        tags: vec![r####"ssh"####.into(), r####"ssl"####.into()]
            .into_iter()
            .collect(),
        description: Some(
            r####"Change the comment stored in the public key of a pub/priv key pair."####.into(),
        ),
        arguments: vec![
            Argument {
                name: r####"new_comment"####.into(),
                description: Some(
                    r####"The new comment that you want to replace the existing comment."####
                        .into(),
                ),
                default_value: None,
            },
            Argument {
                name: r####"ssh_key_path"####.into(),
                description: Some(r####"The path to the private key"####.into()),
                default_value: None,
            },
        ]
        .into_iter()
        .collect(),
        source_url: None,
        author: Some(r####"Kia Matthews"####.into()),
        author_url: Some(r####"https://www.github.com/kiamatthews"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
