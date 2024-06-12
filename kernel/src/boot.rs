use limine::{request::FramebufferRequest, BaseRevision};

#[used]
#[link_section = "requests"]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = "requests"]
pub static MEMORY_MAP_REQUEST: limine::request::MemoryMapRequest =
    limine::request::MemoryMapRequest::new();

#[used]
#[link_section = "requests"]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub fn verify() {
    assert!(BASE_REVISION.is_supported());
    assert!(MEMORY_MAP_REQUEST.get_response().is_some());
    assert!(FRAMEBUFFER_REQUEST.get_response().is_some());
}
