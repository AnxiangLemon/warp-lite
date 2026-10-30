use std::sync::Arc;

use warpui::{Entity, ModelContext, SingletonEntity};

use crate::server::server_api::referral::ReferralsClient;

pub enum ReferralThemeEvent {
    SentReferralThemeActivated,
    ReceivedReferralThemeActivated,
}

pub struct ReferralThemeStatus;

impl Entity for ReferralThemeStatus {
    type Event = ReferralThemeEvent;
}

impl ReferralThemeStatus {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn sent_referral_theme_active(&self) -> bool {
        false
    }

    pub fn received_referral_theme_active(&self) -> bool {
        false
    }

    pub fn query_referral_status(
        &self,
        _referrals_client: Arc<dyn ReferralsClient>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }
}

impl SingletonEntity for ReferralThemeStatus {}
