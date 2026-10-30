use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Clear the Android logcat buffer"####.into(),command: r####"adb logcat -c"####.into(),tags: vec![r####"android"####.into(),r####"adb"####.into()].into_iter().collect(),description: None,arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/3315389/how-to-empty-clear-the-logcat-buffer-in-android"####.into()),author: Some(r####"Michael Burr"####.into()),author_url: Some(r####"https://stackoverflow.com/users/12711/michael-burr"####.into()),shells: vec![].into_iter().collect(),}
}
