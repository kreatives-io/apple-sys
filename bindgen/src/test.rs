use crate::Builder;

#[test]
fn generate_core_foundation() {
    Builder::with_builtin_config("CoreFoundation", "macosx")
        .expect("macosx sdk not found")
        .generate()
        .expect("generate failed");
}
#[test]
fn generate_core_media() {
    Builder::with_builtin_config("CoreMedia", "macosx")
        .expect("macosx sdk not found")
        .generate()
        .expect("generate failed");
}
#[test]
fn generate_core_media_with_wrong_target() {
    Builder::with_builtin_config("CoreMedia", "ios")
        .expect("macosx sdk not found")
        .generate()
        .expect("generate failed");
}
