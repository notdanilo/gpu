pub use ash::vk;
use std::ffi::CString;
use ash::extensions::khr;
use ash::LoadingError;

pub type Error = Box<dyn std::error::Error>;

pub struct Instance {
    entry: ash::Entry,
    instance: ash::Instance
}

impl Instance {
    pub fn new() -> Result<Self, Error> {
        let entry = unsafe { ash::Entry::new() }?;
        let application_info = vk::ApplicationInfo::builder()
            .application_name(CString::new("Vulkan Application")?.as_c_str())
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(CString::new("No Engine").unwrap().as_c_str())
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::make_api_version(1, 0, 0, 0))
            .build();
        let extension_names = vec![khr::Surface::name().as_ptr(), khr::XlibSurface::name().as_ptr()];
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extension_names)
            .build();
        let instance = unsafe { entry.create_instance(&create_info, None) }?;
        Ok(Self { entry, instance })
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None); }
    }
}
