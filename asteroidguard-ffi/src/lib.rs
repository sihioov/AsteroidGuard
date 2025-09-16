#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jboolean, JNI_TRUE, JNI_FALSE};

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_asteroidguard_Native_isRooted(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    if asteroidguard_core::is_rooted() {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}
