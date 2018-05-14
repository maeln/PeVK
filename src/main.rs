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
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;
use vulkano::sync::GpuFuture;

fn init_vulkan() -> (
    std::sync::Arc<vulkano::instance::Instance>,
    std::sync::Arc<vulkano::device::Device>,
    std::sync::Arc<vulkano::device::Queue>,
) {
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
    return (instance.clone(), device.clone(), queue.clone());
}

fn main() {
    let (instance, device, queue) = init_vulkan();
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

    let source_content = 0..64;
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), source_content)
        .expect("Failed to create source buffer.");

    let dest_content = (0..64).map(|_| 0);
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), dest_content)
        .expect("Failed to create destination buffer.");

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .unwrap()
        .copy_buffer(source.clone(), dest.clone())
        .unwrap()
        .build()
        .unwrap();
    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();
    let src_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();
    assert_eq!(&*src_content, &*dest_content);

    println!("Hello, world!");
}
