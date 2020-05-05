#![warn(rust_2018_idioms)]

use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE
}

fn create_app_info() -> vk::ApplicationInfo {
    let appname = std::ffi::CString::new("The Black Window").unwrap();
    vk::ApplicationInfo {
        p_application_name: appname.as_ptr(),
        application_version: vk::make_version(0, 1, 0),
        ..Default::default()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entry = ash::Entry::new().unwrap();
    
    let layer_names: Vec<std::ffi::CString> = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
    let layer_name_pointers: Vec<*const i8> = layer_names.iter().map(|layer_name| layer_name.as_ptr()).collect();
    let extension_name_pointers: Vec<*const i8> = vec![ash::extensions::ext::DebugUtils::name().as_ptr()];
    let instance_create_info = vk::InstanceCreateInfo {
        p_application_info: &create_app_info(),
        pp_enabled_layer_names: layer_name_pointers.as_ptr(),
        enabled_layer_count: layer_name_pointers.len() as u32,
        pp_enabled_extension_names: extension_name_pointers.as_ptr(),
        enabled_extension_count: extension_name_pointers.len() as u32,
        ..Default::default()
    };

    let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };


    let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);
    let debugcreateinfo = vk::DebugUtilsMessengerCreateInfoEXT {
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        pfn_user_callback: Some(vulkan_debug_utils_callback),
        ..Default::default()
    };

    let utils_messenger = unsafe { debug_utils.create_debug_utils_messenger(&debugcreateinfo, None).unwrap() };

    unsafe { debug_utils.destroy_debug_utils_messenger(utils_messenger, None) };
    unsafe { instance.destroy_instance(None) };

    Ok(())
}
