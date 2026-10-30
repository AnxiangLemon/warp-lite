use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"MacOS 13+ - Disable/Enabled mouse acceleration"####.into(),command: r####"echo "old value:"
defaults read -g com.apple.mouse.scaling 
defaults write -g com.apple.mouse.scaling -integer {{acceleration_amount}}"####.into(),tags: vec![r####"MacOS"####.into()].into_iter().collect(),description: Some(r####"Update mouse acceleration built into MacOS. Returns -1 when disabled. For MacOS 13+"####.into()),arguments: vec![Argument {name: r####"acceleration_amount"####.into(),description: Some(r####"(-1) = disabled, (0-3) = enabled, amount of acceleration"####.into()),default_value: Some(r####"-1"####.into()),}].into_iter().collect(),source_url: Some(r####"https://productivityspot.com/how-to-turn-off-mac-mouse-acceleration/"####.into()),author: Some(r####"mikikiv"####.into()),author_url: None,shells: vec![].into_iter().collect(),}
}
