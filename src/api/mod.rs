mod api_key;
mod job;
mod timeslot;
mod tracker;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    timeslot::init(cfg);
    job::init(cfg);
    tracker::init(cfg);
    api_key::init(cfg);
}
