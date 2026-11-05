//! Private selector for the fixed two-change experiment.
static ENABLED:std::sync::OnceLock<bool>=std::sync::OnceLock::new();
#[inline]
pub fn enabled()->bool{*ENABLED.get_or_init(||std::env::var("ZOPRO_CPU_FINALIZE_DRAIN_BUNDLE").as_deref()==Ok("1"))}
