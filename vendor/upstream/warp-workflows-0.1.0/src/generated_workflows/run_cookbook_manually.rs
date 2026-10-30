use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Manually run a cookbook by name"####.into(),
        command: r####"sudo chef client -o recipe['{{cookbook}}']"####.into(),
        tags: vec![r####"chef"####.into()].into_iter().collect(),
        description: Some(r####"Use to run a cookbook manually"####.into()),
        arguments: vec![Argument {
            name: r####"cookbook"####.into(),
            description: Some(r####"Name of the cookbook"####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://docs.chef.io/ctl_chef_client"####.into()),
        author: Some(r####"Amado Tejada"####.into()),
        author_url: Some(r####"https://github.com/amadotejada"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
