use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer};
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::sync::GpuFuture;

fn main() {
    println!("Hello, world!");
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("Failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    // let data = 12;
    // let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), data)
    //     .expect"failed to create buffer");

    let source_content = 0..64;
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), source_content)
        .expect("failed to create buffer");

    let dest_content = (0..64).map(|_| 0);
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), dest_content)
        .expect("failed to create buffer");

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .map_err(|_| "dammit cant create this stuff")
        .and_then(|x| {
            x.copy_buffer(source.clone(), dest.clone())
                .map_err(|_| "cant copy to buffer?")
        })
        .and_then(|x| x.build().map_err(|_| "whuuut"))
        .unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .and_then(|x| x.wait(None))
        .unwrap();

    let source_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();

    assert_eq!(&*source_content, &*dest_content);

    let data_iter = 0..65536;
    let data_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), data_iter)
        .expect("Failed to create buffer");
}
