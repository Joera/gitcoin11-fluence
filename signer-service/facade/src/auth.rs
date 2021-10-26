use marine_rs_sdk::marine;
use marine_rs_sdk::CallParameters;

#[marine]
#[allow(dead_code)]
pub fn debug_meta() -> CallParameters {

    marine_rs_sdk::get_call_parameters()
}

pub fn is_owner() -> bool {
    let meta = marine_rs_sdk::get_call_parameters();
    let caller = meta.init_peer_id;
    let owner = meta.service_creator_peer_id;

    caller == owner
}

#[marine]
#[allow(dead_code)]
pub fn am_i_owner() -> bool {
    is_owner()
}
