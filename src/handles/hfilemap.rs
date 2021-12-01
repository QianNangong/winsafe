#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::{GetLastError, HIDWORD, LODWORD};
use crate::handles::prelude::HandleClose;
use crate::privs::bool_to_winresult;

/// Handle to a
/// [file mapping](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw).
/// Originally just a `HANDLE`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HFILEMAP(pub(crate) *mut std::ffi::c_void);

impl_handle!(HFILEMAP);
impl HandleClose for HFILEMAP {}

impl HFILEMAP {
	/// [`MapViewOfFile`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HFILEMAPVIEW::UnmapViewOfFile`](crate::HFILEMAPVIEW::UnmapViewOfFile)
	/// call.
	pub fn MapViewOfFile(self,
		desired_access: co::FILE_MAP,
		offset: u64,
		number_of_bytes_to_map: Option<i64>) -> WinResult<HFILEMAPVIEW>
	{
		unsafe {
			kernel32::MapViewOfFile(
				self.0,
				desired_access.0,
				HIDWORD(offset),
				LODWORD(offset),
				number_of_bytes_to_map.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| HFILEMAPVIEW(ptr))
			.ok_or_else(|| GetLastError())
	}
}

//------------------------------------------------------------------------------

/// Address of a
/// [mapped view](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile).
/// Originally just an `LPVOID`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HFILEMAPVIEW(pub(crate) *mut std::ffi::c_void);

impl_handle!(HFILEMAPVIEW);

impl HFILEMAPVIEW {
	/// [`UnmapViewOfFile`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-unmapviewoffile)
	/// method.
	pub fn UnmapViewOfFile(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::UnmapViewOfFile(self.0) })
	}

	/// Returns a slice representing the mapped memory. You can modify the
	/// contents. You should call this method only if the file has write access.
	///
	/// **Note**: If the file is resized to a smaller size, the slice will still
	/// map the bytes beyond the file. This may cause serious errors. So, if the
	/// file is resized, re-generate the slice by calling `as_slice` again.
	pub fn as_mut_slice<'a>(self, len: usize) -> &'a mut [u8] {
		unsafe { std::slice::from_raw_parts_mut(self.0 as _, len) }
	}

	/// Returns a slice representing the mapped memory.
	///
	/// **Note**: If the file is resized to a smaller size, the slice will still
	/// map the bytes beyond the file. This may cause serious errors. So, if the
	/// file is resized, re-generate the slice by calling `as_slice` again.
	///
	/// # Examples
	///
	/// Reading the contents of a file into a string:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HFILE};
	///
	/// let hfile = HFILE::CreateFile(
	///     "C:\\Temp\\test.txt",
	///     co::GENERIC::READ,
	///     co::FILE_SHARE::READ,
	///     None,
	///     co::DISPOSITION::OPEN_EXISTING,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	/// )?;
	///
	/// let hmap = hfile.CreateFileMapping(
	///     None,
	///     co::PAGE::READONLY,
	///     None,
	///     None,
	/// )?;
	///
	/// let view = hmap.MapViewOfFile(co::FILE_MAP::READ, 0, None)?;
	///
	/// let slice = view.as_slice(hfile.GetFileSizeEx()?);
	/// let text = std::str::from_utf8(slice)?;
	///
	/// view.UnmapViewOfFile()?;
	/// hmap.CloseHandle()?;
	/// hfile.CloseHandle()?;
	///
	/// println!("{}", text);
	/// ```
	pub fn as_slice<'a>(self, len: usize) -> &'a [u8] {
		unsafe { std::slice::from_raw_parts(self.0 as _, len) }
	}
}
