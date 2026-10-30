use warpui::{Entity, ModelContext, SingletonEntity};



#[derive(Clone, Debug)]

pub enum PricingPromotionStateEvent {}



#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]

pub enum PricingPromotionSurface {

    #[default]

    Unknown,

}



pub struct PricingPromotionState;



impl PricingPromotionState {

    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {

        Self

    }

}



impl Entity for PricingPromotionState {

    type Event = PricingPromotionStateEvent;

}



impl SingletonEntity for PricingPromotionState {}
