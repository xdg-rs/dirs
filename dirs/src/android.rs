use anyhow::Result;
use std::path::{Path, PathBuf};

unsafe fn android_dir(dir: &str) -> Result<PathBuf> {
    let ctx = ndk_context::android_context();
    let vm = jni::JavaVM::from_raw(ctx.vm() as _)?;
    let activity = jni::objects::JObject::from_raw(ctx.context() as _);
    let env = vm.attach_current_thread()?;
    let file = env.call_method(activity, dir, "()Ljava/io/File;", &[])?.l()?;
    let path = env.call_method(file, "getAbsolutePath", "()Ljava/lang/String;", &[])?.l()?;
    let path = env.get_string(path.into())?;
    Ok(Path::new(path.to_str()?).to_path_buf())
}

pub fn home_dir() -> Option<PathBuf> {
    None
}
pub fn cache_dir() -> Option<PathBuf> {
    unsafe { android_dir("getCacheDir") }.ok()
}
pub fn config_dir() -> Option<PathBuf> {
    data_dir()
}
pub fn data_dir() -> Option<PathBuf> {
    unsafe { android_dir("getFilesDir") }.ok()
}
pub fn data_local_dir() -> Option<PathBuf> {
    data_dir()
}
pub fn executable_dir() -> Option<PathBuf> {
    None
}
pub fn runtime_dir() -> Option<PathBuf> {
    None
}
pub fn audio_dir() -> Option<PathBuf> {
    None
}
pub fn desktop_dir() -> Option<PathBuf> {
    None
}
pub fn document_dir() -> Option<PathBuf> {
    None
}
pub fn download_dir() -> Option<PathBuf> {
    None
}
pub fn font_dir() -> Option<PathBuf> {
    Some(Path::new("system").join("fonts"))
}
pub fn picture_dir() -> Option<PathBuf> {
    None
}
pub fn public_dir() -> Option<PathBuf> {
    None
}
pub fn template_dir() -> Option<PathBuf> {
    None
}
pub fn video_dir() -> Option<PathBuf> {
    None
}
