use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Manually initiate a recon inventory"####.into(),
        command: r####"sudo jamf recon"####.into(),
        tags: vec![r####"jamf"####.into()].into_iter().collect(),
        description: Some(r####"Force a full Jamf inventory from the client"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: None,
        author: Some(r####"Amado Tejada"####.into()),
        author_url: Some(r####"https://github.com/amadotejada"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
