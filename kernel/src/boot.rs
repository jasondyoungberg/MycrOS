use limine::{
    request::{FramebufferRequest, HhdmRequest, MemoryMapRequest, SmpRequest},
    response::{FramebufferResponse, HhdmResponse, MemoryMapResponse, SmpResponse},
    BaseRevision,
};
use spin::Lazy;

#[used]
#[link_section = "requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = "requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
pub static HHDM_RESPONSE: Lazy<&HhdmResponse> =
    Lazy::new(|| HHDM_REQUEST.get_response().expect("HHDM request failed"));

#[used]
#[link_section = "requests"]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();
pub static MEMORY_MAP_RESPONSE: Lazy<&MemoryMapResponse> = Lazy::new(|| {
    MEMORY_MAP_REQUEST
        .get_response()
        .expect("Memory map request failed")
});

#[used]
#[link_section = "requests"]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();
pub static FRAMEBUFFER_RESPONSE: Lazy<&FramebufferResponse> = Lazy::new(|| {
    FRAMEBUFFER_REQUEST
        .get_response()
        .expect("Framebuffer request failed")
});

#[used]
#[link_section = "requests"]
pub static SMP_REQUEST: SmpRequest = SmpRequest::new();
pub static SMP_RESPONSE: Lazy<&SmpResponse> =
    Lazy::new(|| SMP_REQUEST.get_response().expect("SMP request failed"));

pub fn verify() {
    assert!(BASE_REVISION.is_supported());

    let _ = *HHDM_RESPONSE;
    let _ = *MEMORY_MAP_RESPONSE;
    let _ = *FRAMEBUFFER_RESPONSE;
    let _ = *SMP_RESPONSE;
}
