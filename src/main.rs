#[macro_use]
extern crate vulkano;

use vulkano::device::Device;
use vulkano::instance::DeviceExtensions;
use vulkano::instance::Features;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;

fn main() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("Your computer doesn't seem to support Vulkan.");

    for device in PhysicalDevice::enumerate(&instance) {
        println!("{}", device.name());
    }
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No physical device available");

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Couldn't find a graphical queue family.");

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        ).expect("Failed to create device.")
    };
    let queue = queues.next().unwrap();

    let data = 12;
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), data)
        .expect("Failed to create buffer");

    {
        let valr = buffer.read().unwrap();
        println!("val: {}", *valr);
    }

    {
        let mut content = buffer.write().unwrap();
        *content *= 2;
        println!("val: {}", *content);
    }

    println!("Hello, world!");
}
