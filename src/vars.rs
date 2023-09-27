
use std::env::{var,set_var,remove_var};
use std::ffi::OsStr;

pub fn get<K: AsRef<OsStr>>(ref key: K) -> Option<String> {
	match var(key) {
        Ok(value) => Some(value),
        Err(_) => None
    }
}

pub fn set<K: AsRef<OsStr>, V: AsRef<OsStr>>(ref key: K, ref value: V) {
	if
		key.as_ref().is_empty() || key.as_ref().to_string_lossy().contains('=') || key.as_ref().to_string_lossy().contains('\0')
		|| value.as_ref().to_string_lossy().contains('\0')
	{
        return;
    }
	set_var(key, value);
}

pub fn del<K: AsRef<OsStr>>(ref key: K) {
	remove_var(key);
}