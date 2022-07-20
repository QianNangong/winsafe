use crate::ffi_types::{BOOL, HRES, PCSTR, PCVOID, PSTR, PVOID};

// This block should be in the "ole" feature, but there is a circular dependency
// in the Windows headers.
extern_sys! { "ole32";
	PropVariantClear(PVOID) -> HRES
}

extern_sys! { "oleaut32";
	OleLoadPicture(PVOID, i32, BOOL, PCVOID, PVOID) -> HRES
	OleLoadPicturePath(PCSTR, *mut PVOID, u32, u32, PCVOID, *mut PVOID) -> HRES
	SysAllocString(PCSTR) -> PSTR
	SysFreeString(PSTR)
	SysReAllocString(PSTR, PCSTR) -> PSTR
	SysStringLen(PSTR) -> u32
	SystemTimeToVariantTime(PVOID, *mut f64) -> i32
	VariantClear(PVOID) -> HRES
	VariantInit(PVOID)
	VariantTimeToSystemTime(f64, PVOID)  -> i32
}

extern_sys! { "propsys";
	PSGetNameFromPropertyKey(PCVOID, *mut PSTR) -> HRES
}
