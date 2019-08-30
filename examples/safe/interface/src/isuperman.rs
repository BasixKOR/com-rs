use com::{ComInterface, ComOutPtr, ComPtr, IUnknown, IUnknownVTable};
use std::mem::MaybeUninit;
use winapi::{shared::guiddef::IID, um::winnt::HRESULT};

pub const IID_ISUPERMAN: IID = IID {
    Data1: 0xa56b76e4,
    Data2: 0x4bd7,
    Data3: 0x48b9,
    Data4: [0x8a, 0xf6, 0xb9, 0x3f, 0x43, 0xe8, 0x69, 0xc8],
};

pub trait ISuperman {
    // [in]
    fn take_input(&mut self, in_var: u32) -> HRESULT;

    // [out]
    fn populate_output(&mut self, out_var: &mut ComOutPtr<u32>) -> HRESULT;

    // [in, out]
    fn mutate_and_return(&mut self, in_out_var: &mut Option<Box<u32>>) -> HRESULT;

    // [in] pointer
    fn take_input_ptr(&mut self, in_ptr_var: &Option<u32>) -> HRESULT;

    // // [in, out] Interface
    // fn take_interface();

    // // [out] Interface
    // fn populate_interface(ComOutPtr<ComItf>);
}

unsafe impl ComInterface for dyn ISuperman {
    type VTable = ISupermanVTable;
    const IID: IID = IID_ISUPERMAN;
}

pub type ISupermanVPtr = *const ISupermanVTable;

impl<T: ISuperman + ComInterface + ?Sized> ISuperman for ComPtr<T> {
    fn take_input(&mut self, in_var: u32) -> HRESULT {
        let itf_ptr = self.into_raw() as *mut ISupermanVPtr;
        unsafe { ((**itf_ptr).TakeInput)(itf_ptr, in_var) }
    }

    fn populate_output(&mut self, out_var: &mut ComOutPtr<u32>) -> HRESULT {
        let itf_ptr = self.into_raw() as *mut ISupermanVPtr;

        // Let called-procedure write to possibly uninit memory.
        let mut proxy = MaybeUninit::<u32>::uninit();

        unsafe {
            let hr = ((**itf_ptr).PopulateOutput)(itf_ptr, proxy.as_mut_ptr());
            println!("Returned from populate output!");

            // Consumes the MaybeUninit. Not exactly sure what happens to the
            // allocated memory here. Working verison for now.
            let value = proxy.assume_init();
            out_var.set(value);

            // Attempt 2:
            // out_var.wrap(proxy.as_mut_ptr());
            // let mut value = proxy.assume_init();
            // out_var.set(value);
            //
            // Remarks: This should be the ideal way to do it (with the old "set" that
            // just writes to the underlying pointer), as we are
            // pointing to the same location that the server wrote to.
            // However, failed later during client code when doing
            // &*com_out_ptr.as_mut_ptr(). Might be triggering UB somewhere.

            hr
        }
    }

    fn mutate_and_return(&mut self, in_out_var: &mut Option<Box<u32>>) -> HRESULT {
        let itf_ptr = self.into_raw() as *mut ISupermanVPtr;
        let in_out_raw = match in_out_var {
            Some(ref mut n) => n.as_mut() as *mut u32,
            None => std::ptr::null_mut::<u32>(),
        };

        unsafe { ((**itf_ptr).MutateAndReturn)(itf_ptr, in_out_raw) }
    }

    fn take_input_ptr(&mut self, in_ptr_var: &Option<u32>) -> HRESULT {
        let itf_ptr = self.into_raw() as *mut ISupermanVPtr;
        let in_out_raw = match in_ptr_var {
            Some(n) => n as *const u32,
            None => std::ptr::null_mut::<u32>(),
        };

        unsafe { ((**itf_ptr).TakeInputPtr)(itf_ptr, in_out_raw) }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct ISupermanVTable {
    pub base: IUnknownVTable,
    pub TakeInput: unsafe extern "stdcall" fn(*mut ISupermanVPtr, in_var: u32) -> HRESULT,
    pub PopulateOutput:
        unsafe extern "stdcall" fn(*mut ISupermanVPtr, out_var: *mut u32) -> HRESULT,
    pub MutateAndReturn:
        unsafe extern "stdcall" fn(*mut ISupermanVPtr, in_out_var: *mut u32) -> HRESULT,
    pub TakeInputPtr:
        unsafe extern "stdcall" fn(*mut ISupermanVPtr, in_ptr_var: *const u32) -> HRESULT,
}
