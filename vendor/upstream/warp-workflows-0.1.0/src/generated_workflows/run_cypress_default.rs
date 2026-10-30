use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Run Cypress Headless"####.into(),
        command: r####"npx cypress run"####.into(),
        tags: vec![r####"cypress"####.into()].into_iter().collect(),
        description: Some(r####"Run Cypress in Headless Mode"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://docs.cypress.io/guides/guides/command-line#cypress-run"####.into(),
        ),
        author: Some(r####"cypress-dx"####.into()),
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}
