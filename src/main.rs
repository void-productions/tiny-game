#![warn(rust_2018_idioms)]

use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use winit::platform::unix::WindowExtUnix;

use std::thread;
use std::time;

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

    vk::ApplicationInfo::builder()
        .application_name(&appname)
        .application_version(vk::make_version(0, 1, 0))
        .build()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let eventloop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&eventloop)?;

    let entry = ash::Entry::new().unwrap();
    
    let layer_names: Vec<std::ffi::CString> = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
    let layer_name_pointers: Vec<*const i8> = layer_names.iter().map(|layer_name| layer_name.as_ptr()).collect();
    let extension_name_pointers: Vec<*const i8> = vec![
        ash::extensions::ext::DebugUtils::name().as_ptr(),
        ash::extensions::khr::Surface::name().as_ptr(),
        ash::extensions::khr::XlibSurface::name().as_ptr(),
    ];
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

    // winit + vulkan
    let x11_display = window.xlib_display().unwrap();
    let x11_window = window.xlib_window().unwrap();
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR::builder()
        .window(x11_window)
        .dpy(x11_display as *mut vk::Display);
    let xlib_surface_loader = ash::extensions::khr::XlibSurface::new(&entry, &instance);
    let surface = unsafe { xlib_surface_loader.create_xlib_surface(&x11_create_info, None) }?;
    let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

    // choose device
    let phys_devs = unsafe { instance.enumerate_physical_devices()? };
    if let Some(physical_device) = phys_devs.first() {
        // queues
        let queuefamilyproperties = unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };

        let qfamindices = {
            let mut found_graphics_q_index = None;
            let mut found_transfer_q_index = None;
            for (index, qfam) in queuefamilyproperties.iter().enumerate() {
                if qfam.queue_count > 0 && qfam.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    found_graphics_q_index = Some(index as u32);
                }
                if qfam.queue_count > 0 && qfam.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                    if found_transfer_q_index.is_none() || !qfam.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        found_transfer_q_index = Some(index as u32);
                    }
                }
            }
            (found_graphics_q_index.unwrap(), found_transfer_q_index.unwrap())
        };

        let priorities = [1.0f32];
        let queue_infos = [
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(qfamindices.0)
                .queue_priorities(&priorities)
                .build(),
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(qfamindices.1)
                .queue_priorities(&priorities)
                .build(),
        ];
        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_layer_names(&layer_name_pointers);
        let logical_device = unsafe { instance.create_device(*physical_device, &device_create_info, None)? };
        let graphics_queue = unsafe { logical_device.get_device_queue(qfamindices.0, 0) };
        let transfer_queue = unsafe { logical_device.get_device_queue(qfamindices.1, 0) };

        thread::sleep(time::Duration::from_millis(1000));

        unsafe { logical_device.destroy_device(None) };
    }

    unsafe {
        surface_loader.destroy_surface(surface, None);
        debug_utils.destroy_debug_utils_messenger(utils_messenger, None);
        instance.destroy_instance(None);
    };

    Ok(())
}
