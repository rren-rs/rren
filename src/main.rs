use std::borrow::Cow;
use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
};

fn main() {
    let app_info = ApplicationInfo {
        application_name: Some(Cow::from("rren")),
        application_version: Some(Version{ major: 1, minor: 0, patch: 0 }),
        engine_name: Some(Cow::from("rren-core")),
        engine_version: Some(Version { major: 1, minor: 0, patch: 0 })
    };
    let instance = Instance::new(Some(&app_info), &InstanceExtensions::none(), None).expect("failed to initialize instance");
}
