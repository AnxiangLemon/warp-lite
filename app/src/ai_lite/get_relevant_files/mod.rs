pub mod controller {
    use warpui::{Entity, ModelContext};

    pub struct GetRelevantFilesController;

    impl GetRelevantFilesController {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for GetRelevantFilesController {
        type Event = ();
    }
}

pub mod api;
