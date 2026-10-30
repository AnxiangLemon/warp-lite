use std::sync::Arc;

use input_classifier::{HeuristicClassifier, InputClassifier};
use warpui::{Entity, SingletonEntity};

pub struct InputClassifierModel {
    pub classifier: Arc<dyn InputClassifier>,
}

impl Default for InputClassifierModel {
    fn default() -> Self {
        Self {
            classifier: Arc::new(HeuristicClassifier),
        }
    }
}

impl InputClassifierModel {
    pub fn classifier(&self) -> Arc<dyn InputClassifier> {
        self.classifier.clone()
    }
}

impl Entity for InputClassifierModel {
    type Event = ();
}

impl SingletonEntity for InputClassifierModel {}
