#![allow(non_snake_case)]

use std::fmt::{Debug, Display, LowerHex, UpperHex};

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::{gdi32, kernel32};
use crate::funcs::GetLastError;
use crate::privs::bool_to_winresult;

/// Any Windows
/// [handle](https://docs.microsoft.com/en-us/windows/win32/sysinfo/handles-and-objects).
pub trait Handle: Debug + Display + LowerHex + UpperHex + Copy + Clone + PartialEq + Eq + Send {
	/// The null, invalid handle.
	const NULL: Self;

	/// Creates a new handle instance by wrapping a pointer.
	unsafe fn from_ptr<T>(p: *mut T) -> Self;

	/// Returning the underlying raw pointer.
	unsafe fn as_ptr(self) -> *mut std::ffi::c_void;

	/// Tells if the handle is invalid (null).
	fn is_null(self) -> bool {
		unsafe { self.as_ptr().is_null() }
	}

	/// Returns `None` if the handle is null, otherwise returns `Some(&Self)`.
	fn as_opt(self) -> Option<Self> {
		if self.is_null() {
			None
		} else {
			Some(self)
		}
	}
}

/// Any [`Handle`](crate::prelude::Handle) which can be closed.
pub trait HandleClose: Handle {
	/// [`CloseHandle`](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// method.
	fn CloseHandle(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::CloseHandle(self.as_ptr()) })
	}
}

/// Any
/// [`HGDIOBJ`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hgdiobj)
/// handle, which is a specific handle for
/// [GDI objects](https://docs.microsoft.com/en-us/windows/win32/sysinfo/gdi-objects).
pub trait HandleGdi: Handle {
	/// [`DeleteObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
	/// method.
	fn DeleteObject(self) -> WinResult<()> {
		match unsafe { gdi32::DeleteObject(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(()), // not really an error
				err => Err(err),
			},
			_ => Ok(()),
		}
	}
}
