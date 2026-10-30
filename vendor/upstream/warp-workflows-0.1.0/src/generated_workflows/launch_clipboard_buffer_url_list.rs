use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Launch tabs with clipboard URLs"####.into(),command: r####"pbpaste | xargs -n1 -I{} open {}"####.into(),tags: vec![r####"pbpaste"####.into()].into_iter().collect(),description: Some(r####"Launch all URLs (arranged in a list in the clipboard buffer). Will be launched using default browser and tab creation preference. Useful for copying a column of URLs from a spreadsheet and visiting them all at once."####.into()),arguments: vec![].into_iter().collect(),source_url: None,author: None,author_url: None,shells: vec![].into_iter().collect(),}
}
