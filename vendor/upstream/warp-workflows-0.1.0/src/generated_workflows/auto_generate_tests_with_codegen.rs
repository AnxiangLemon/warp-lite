use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Auto-generate Playwright tests with Codegen"####.into(),
        command: r####"npx playwright codegen {{url}}"####.into(),
        tags: vec![r####"playwright"####.into()].into_iter().collect(),
        description: Some(r####"Auto-generates Playwright tests with Codegen."####.into()),
        arguments: vec![Argument {
            name: r####"url"####.into(),
            description: Some(r####"URL of the webpage."####.into()),
            default_value: Some(r####"playwright.dev"####.into()),
        }]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://playwright.dev/docs/codegen-intro#running-codegen"####.into(),
        ),
        author: Some(r####"DominusKelvin"####.into()),
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}
