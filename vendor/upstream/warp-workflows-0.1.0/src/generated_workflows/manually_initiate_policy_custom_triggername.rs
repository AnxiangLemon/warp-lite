use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Manually initiate a policy via policy trigger name"####.into(),
        command: r####"sudo jamf policy -event "{{triggerName}}" -verbose"####.into(),
        tags: vec![r####"jamf"####.into()].into_iter().collect(),
        description: Some(r####"Use to run a Jamf policy manually"####.into()),
        arguments: vec![Argument {
            name: r####"triggerName"####.into(),
            description: Some(r####"Custom trigger name of the Jamf policy"####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://docs.jamf.com/technical-articles/Manually_Initiating_a_Policy.html"####
                .into(),
        ),
        author: Some(r####"Amado Tejada"####.into()),
        author_url: Some(r####"https://github.com/amadotejada"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
