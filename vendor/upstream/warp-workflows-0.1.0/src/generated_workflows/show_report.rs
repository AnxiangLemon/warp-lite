use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Show report on Playwright tests"####.into(),
        command: r####"npx playwright show-report"####.into(),
        tags: vec![r####"playwright"####.into()].into_iter().collect(),
        description: Some(
            r####"Opens a webpage showing a report on your Playwright tests."####.into(),
        ),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://playwright.dev/docs/running-tests#test-reports"####.into()),
        author: Some(r####"DominusKelvin"####.into()),
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}
