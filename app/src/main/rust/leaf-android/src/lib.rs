use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_club_lemos_android_logic_LeafVpnService_runLeaf(
    env: JNIEnv,
    _: JClass,
    config_path: JString,
) {
    let config_path = env
        .get_string(config_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let opts = leaf::StartOptions {
        config: leaf::Config::File(config_path),
        runtime_opt: leaf::RuntimeOption::MultiThreadAuto(2),
    };
    leaf::start(0, opts).unwrap();
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_club_lemos_android_logic_LeafVpnService_stopLeaf(
    _: JNIEnv,
    _: JClass,
) {
    leaf::shutdown(0);
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_club_lemos_android_logic_LeafVpnService_reloadLeaf(
    _: JNIEnv,
    _: JClass,
) {
    leaf::reload(0).unwrap();
}
