use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Manually initiate a policy via policy id"####.into(),
        command: r####"sudo jamf policy -event "{{policy_id}}" -verbose"####.into(),
        tags: vec![r####"jamf"####.into()].into_iter().collect(),
        description: Some(r####"Use to run a Jamf policy manually"####.into()),
        arguments: vec![Argument {
            name: r####"policy_id"####.into(),
            description: Some(r####"The id number of the Jamf policy"####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: None,
        author: Some(r####"Amado Tejada"####.into()),
        author_url: Some(r####"https://github.com/amadotejada"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
