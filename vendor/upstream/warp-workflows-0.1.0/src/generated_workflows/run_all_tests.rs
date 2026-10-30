use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Run all Playwright tests"####.into(),
        command: r####"npx playwright test"####.into(),
        tags: vec![r####"playwright"####.into()].into_iter().collect(),
        description: Some(r####"Runs all Playwright tests"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://playwright.dev/docs/intro#running-the-example-test"####.into(),
        ),
        author: Some(r####"DominusKelvin"####.into()),
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}
