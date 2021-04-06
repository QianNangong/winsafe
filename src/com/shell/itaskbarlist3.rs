#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::funcs::hr_to_winresult;
use crate::com::PPComVT;
use crate::com::shell::{co, ITaskbarList2};
use crate::com::shell::vt::{ITaskbarList2VT, ITaskbarList3VT};
use crate::handles::HWND;

/// [`ITaskbarList3`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
/// COM interface.
///
/// Inherits from:
/// * [`ITaskbarList2`](crate::shell::ITaskbarList2);
/// * [`ITaskbarList`](crate::shell::ITaskbarList);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls [`IUnknown::Release`](crate::IUnknown::Release) when the
/// object goes out of scope.
///
/// # Examples
///
/// Usually instantiated with [`CoCreateInstance`](crate::CoCreateInstance):
/// ```rust,ignore
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj: shell::ITaskbarList3 = CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
#[derive(Clone)]
pub struct ITaskbarList3 {
	/// Methods of base interface
	/// [`ITaskbarList2`](crate::shell::ITaskbarList2).
	pub ITaskbarList2: ITaskbarList2,
}

impl From<PPComVT<ITaskbarList3VT>> for ITaskbarList3 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPComVT<ITaskbarList3VT>) -> Self {
		Self {
			ITaskbarList2: ITaskbarList2::from(ppv as PPComVT<ITaskbarList2VT>),
		}
	}
}

impl ITaskbarList3 {
	unsafe fn ppv(&self) -> PPComVT<ITaskbarList3VT> {
		self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3VT>()
	}

	/// [`ITaskbarList3::RegisterTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
	/// method.
	pub fn RegisterTab(&self,
		hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
	{
		unsafe {
			hr_to_winresult(
				((**self.ppv()).RegisterTab)(self.ppv(), hwndTab.ptr, hwndMDI.ptr),
			)
		}
	}

	/// [`ITaskbarList3::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
	/// method.
	pub fn SetProgressState(&self,
		hwnd: HWND, tbpfFlags: co::TBPF) -> WinResult<()>
	{
		unsafe {
			hr_to_winresult(
				((**self.ppv()).SetProgressState)(
					self.ppv(), hwnd.ptr, tbpfFlags.0,
				),
			)
		}
	}

	/// [`ITaskbarList3::SetProgressValue`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
	/// method.
	pub fn SetProgressValue(&self,
		hwnd: HWND, ullCompleted: u64, ullTotal: u64) -> WinResult<()>
	{
		unsafe {
			hr_to_winresult(
				((**self.ppv()).SetProgressValue)(
					self.ppv(), hwnd.ptr, ullCompleted, ullTotal,
				),
			)
		}
	}

	/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
	/// method.
	pub fn SetTabActive(&self,
		hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
	{
		unsafe {
			hr_to_winresult(
				((**self.ppv()).SetTabActive)(
					self.ppv(), hwndTab.ptr, hwndMDI.ptr, 0,
				),
			)
		}
	}

	/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
	/// method.
	pub fn SetTabOrder(&self,
		hwndTab: HWND, hwndInsertBefore: HWND) -> WinResult<()>
	{
		unsafe {
			hr_to_winresult(
				((**self.ppv()).SetTabOrder)(
					self.ppv(), hwndTab.ptr, hwndInsertBefore.ptr,
				),
			)
		}
	}
}
