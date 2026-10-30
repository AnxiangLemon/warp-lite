use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Connect to openfortivpn with one time password"####.into(),
        command: r####"sudo openfortivpn -o {{otp}}"####.into(),
        tags: vec![r####"openfortivpn"####.into()].into_iter().collect(),
        description: Some(r####"Uses one time password to connect to openfortivpn."####.into()),
        arguments: vec![Argument {
            name: r####"otp"####.into(),
            description: Some(r####"The one time password"####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://manpages.debian.org/unstable/openfortivpn/openfortivpn.1.en.html"####
                .into(),
        ),
        author: Some(r####"Rafal Petryka"####.into()),
        author_url: Some(r####"https://github.com/RafalPetryka"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
