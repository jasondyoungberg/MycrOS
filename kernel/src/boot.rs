use limine::{
    request::{FramebufferRequest, HhdmRequest},
    BaseRevision,
};
use spin::Lazy;

pub static HHDM_OFFSET: Lazy<usize> =
    Lazy::new(|| HHDM_REQUEST.get_response().unwrap().offset() as usize);

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: [u64; 4] = [
    0xf6b8f4b39de7d1ae,
    0xfab91a6940fcb9cf,
    0x785c6ed015d3e316,
    0x181e920a7852b9d9,
];

#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: [u64; 2] = [0xadc0e0531bb10d03, 0x9572709f31764c62];

pub fn verify() {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
    assert!(BASE_REVISION.is_supported());
}
