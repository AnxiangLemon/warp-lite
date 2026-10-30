use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Open Cypress"####.into(),command: r####"npx cypress open"####.into(),tags: vec![r####"cypress"####.into()].into_iter().collect(),description: Some(r####"Open the Cypress Test Runner."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://docs.cypress.io/guides/getting-started/installing-cypress#Opening-Cypress"####.into()),author: Some(r####"cypress-dx"####.into()),author_url: None,shells: vec![].into_iter().collect(),}
}
