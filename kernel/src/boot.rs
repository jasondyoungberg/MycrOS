use limine::{
    request::{FramebufferRequest, HhdmRequest, MemoryMapRequest},
    response::{FramebufferResponse, HhdmResponse, MemoryMapResponse},
    BaseRevision,
};
use spin::Lazy;

#[used]
#[link_section = "requests"]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = "requests"]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[link_section = "requests"]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[link_section = "requests"]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub static HHDM_RESPONSE: Lazy<&HhdmResponse> =
    Lazy::new(|| HHDM_REQUEST.get_response().expect("HHDM request failed"));

pub static MEMORY_MAP_RESPONSE: Lazy<&MemoryMapResponse> = Lazy::new(|| {
    MEMORY_MAP_REQUEST
        .get_response()
        .expect("Memory map request failed")
});

pub static FRAMEBUFFER_RESPONSE: Lazy<&FramebufferResponse> = Lazy::new(|| {
    FRAMEBUFFER_REQUEST
        .get_response()
        .expect("Framebuffer request failed")
});

pub fn verify() {
    assert!(BASE_REVISION.is_supported());

    let _ = *HHDM_RESPONSE;
    let _ = *MEMORY_MAP_RESPONSE;
    let _ = *FRAMEBUFFER_RESPONSE;
}
