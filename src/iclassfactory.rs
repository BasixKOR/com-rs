use super::*;
use com_interface_attribute::com_interface;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::{GUID, REFIID};
use winapi::shared::minwindef::BOOL;
use winapi::shared::ntdef::HRESULT;

#[com_interface(00000001-0000-0000-c000-000000000046)]
pub trait IClassFactory {
    fn create_instance(
        &mut self,
        aggr: *mut IUnknownVPtr,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    fn lock_server(&mut self, increment: BOOL) -> HRESULT;
}

impl ComPtr<dyn IClassFactory> {
    pub fn get_instance<T: ComInterface + ?Sized>(&mut self) -> Option<ComPtr<T>> {
        let mut ppv = std::ptr::null_mut::<c_void>();
        let aggr = std::ptr::null_mut();
        let hr = self.create_instance(aggr, &T::IID as *const IID, &mut ppv);
        if failed(hr) {
            // TODO: decide what failures are possible
            return None;
        }
        unsafe { Some(ComPtr::new(ppv)) }
    }
}
