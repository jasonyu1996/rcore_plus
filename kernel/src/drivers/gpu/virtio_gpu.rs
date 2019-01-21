use device_tree::Node;
use device_tree::util::SliceRead;
use super::super::bus::virtio_mmio::*;
use rcore_memory::PAGE_SIZE;
use crate::HEAP_ALLOCATOR;
use alloc::alloc::{Layout, GlobalAlloc};
use volatile::{Volatile, ReadOnly, WriteOnly};
use core::mem::size_of;
use super::super::{DRIVERS, Driver, NetDriver, DeviceType};
use log::*;
use alloc::prelude::*;
use crate::arch::cpu;
use crate::memory::active_table;
use rcore_memory::paging::PageTable;
use core::slice;

const VIRTIO_GPU_EVENT_DISPLAY : u32 = 1 << 0;

struct VirtIOGpu {
    interrupt_parent: u32,
    interrupt: u32,
    header: usize,
    // 0 for transmit, 1 for cursor
    queue_num: u32,
    queue_address: usize,
    queue_page: [usize; 2],
    last_used_idx: u16,
}

#[repr(packed)]
#[derive(Debug)]
struct VirtIOGpuConfig {
    events_read: ReadOnly<u32>,
    events_clear: WriteOnly<u32>,
    num_scanouts: Volatile<u32>
}

bitflags! {
    struct VirtIOGpuFeature : u64 {
        const VIRGL = 1 << 0;
        const EDID = 1 << 1;
        // device independent
        const NOFIFY_ON_EMPTY = 1 << 24; // legacy
        const ANY_LAYOUT = 1 << 27; // legacy
        const RING_INDIRECT_DESC = 1 << 28;
        const RING_EVENT_IDX = 1 << 29;
        const UNUSED = 1 << 30; // legacy
        const VERSION_1 = 1 << 32; // detect legacy
        const ACCESS_PLATFORM = 1 << 33; // since virtio v1.1
        const RING_PACKED = 1 << 34; // since virtio v1.1
        const IN_ORDER = 1 << 35; // since virtio v1.1
        const ORDER_PLATFORM = 1 << 36; // since virtio v1.1
        const SR_IOV = 1 << 37; // since virtio v1.1
        const NOTIFICATION_DATA = 1 << 38; // since virtio v1.1
    }
}

const VIRTIO_GPU_CMD_GET_DISPLAY_INFO : u32 = 0x100;
const VIRTIO_GPU_CMD_RESOURCE_CREATE_2D : u32 = 0x101;
const VIRTIO_GPU_CMD_RESOURCE_UNREF : u32 = 0x102;
const VIRTIO_GPU_CMD_SET_SCANOUT : u32 = 0x103;
const VIRTIO_GPU_CMD_RESOURCE_FLUSH : u32 = 0x104;
const VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D : u32 = 0x105;
const VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING : u32 = 0x106;
const VIRTIO_GPU_CMD_RESOURCE_DETACH_BACKING : u32 = 0x107;
const VIRTIO_GPU_CMD_GET_CAPSET_INFO : u32 = 0x108;
const VIRTIO_GPU_CMD_GET_CAPSET : u32 = 0x109;
const VIRTIO_GPU_CMD_GET_EDID : u32 = 0x10a;

const VIRTIO_GPU_CMD_UPDATE_CURSOR : u32 = 0x300;
const VIRTIO_GPU_CMD_MOVE_CURSOR : u32 = 0x301;

const VIRTIO_GPU_RESP_OK_NODATA : u32 = 0x1100;
const VIRTIO_GPU_RESP_OK_DISPLAY_INFO : u32 = 0x1101;
const VIRTIO_GPU_RESP_OK_CAPSET_INFO : u32 = 0x1102;
const VIRTIO_GPU_RESP_OK_CAPSET : u32 = 0x1103;
const VIRTIO_GPU_RESP_OK_EDID : u32 = 0x1104;

const VIRTIO_GPU_RESP_ERR_UNSPEC : u32 = 0x1200;
const VIRTIO_GPU_RESP_ERR_OUT_OF_MEMORY : u32 = 0x1201;
const VIRTIO_GPU_RESP_ERR_INVALID_SCANOUT_ID : u32 = 0x1202;

const VIRTIO_GPU_FLAG_FENCE : u32 = 1 << 0;

#[repr(packed)]
#[derive(Debug)]
struct VirtIOGpuCtrlHdr {
    hdr_type: u32,
    flags: u32,
    fence_id: u64,
    ctx_id: u32,
    padding: u32
}

#[repr(packed)]
#[derive(Debug)]
struct VirtIOGpuRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

#[repr(packed)]
#[derive(Debug)]
struct VirtIOGpuRespDisplayInfo {
    header: VirtIOGpuCtrlHdr,
    rect: VirtIOGpuRect,
    enabled: u32,
    flags: u32
}

const VIRTIO_QUEUE_TRANSMIT: usize = 0;
const VIRTIO_QUEUE_RECEIVE: usize = 1;

impl Driver for VirtIOGpu {
    fn try_handle_interrupt(&mut self) -> bool {
        // for simplicity
        if cpu::id() > 0 {
            return false
        }

        // ensure header page is mapped
        {
            let mut table = active_table();
            if let None = table.get_entry(self.header as usize) {
                table.map(self.header as usize, self.header as usize);
            }
        }
        let mut header = unsafe { &mut *(self.header as *mut VirtIOHeader) };
        let interrupt = header.interrupt_status.read();
        if interrupt != 0 {
            header.interrupt_ack.write(interrupt);
            println!("Got interrupt {:?}", interrupt);
            let response = unsafe { &mut *(self.queue_page[VIRTIO_QUEUE_RECEIVE] as *mut VirtIOGpuRespDisplayInfo) };
            println!("response: {:?}", response);
            return true;
        }
        return false;
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Gpu
    }
}

fn setup_rings(driver: &mut VirtIOGpu) {
    let mut header = unsafe { &mut *(driver.header as *mut VirtIOHeader) };
    let mut ring = unsafe { 
        &mut *((driver.queue_address + size_of::<VirtIOVirtqueueDesc>() * driver.queue_num as usize) as *mut VirtIOVirtqueueAvailableRing) 
    };

    // re-add two buffers to desc
    // chaining read buffer and write buffer into one desc
    for buffer in 0..2 {
        let mut desc = unsafe { &mut *(driver.queue_address as *mut VirtIOVirtqueueDesc).add(buffer) };
        desc.addr.write(driver.queue_page[buffer] as u64);
        desc.len.write(PAGE_SIZE as u32);
        if buffer == VIRTIO_QUEUE_TRANSMIT {
            // device readable
            desc.flags.write(VirtIOVirtqueueFlag::NEXT.bits());
            desc.next.write(1);
        } else {
            // device writable
            desc.flags.write(VirtIOVirtqueueFlag::WRITE.bits());
        }
        ring.ring[buffer].write(buffer as u16);
    }
}

fn notify_device(driver: &mut VirtIOGpu) {
    let mut header = unsafe { &mut *(driver.header as *mut VirtIOHeader) };
    let mut ring = unsafe { 
        &mut *((driver.queue_address + size_of::<VirtIOVirtqueueDesc>() * driver.queue_num as usize) as *mut VirtIOVirtqueueAvailableRing) 
    };
    ring.idx.write(ring.idx.read() + 1);
    header.queue_notify.write(0);
}

fn setup_framebuffer(driver: &mut VirtIOGpu) {
    setup_rings(driver);
    let mut request = unsafe { &mut *(driver.queue_page[0] as *mut VirtIOGpuCtrlHdr) };
    *request = VirtIOGpuCtrlHdr {
        hdr_type: VIRTIO_GPU_CMD_GET_DISPLAY_INFO,
        flags: 0,
        fence_id: 0,
        ctx_id: 0,
        padding: 0
    };
    notify_device(driver);
}

pub fn virtio_gpu_init(node: &Node) {
    let reg = node.prop_raw("reg").unwrap();
    let from = reg.as_slice().read_be_u64(0).unwrap();
    let mut header = unsafe { &mut *(from as *mut VirtIOHeader) };

    header.status.write(VirtIODeviceStatus::DRIVER.bits());

    let mut device_features_bits: u64 = 0;
    header.device_features_sel.write(0); // device features [0, 32)
    device_features_bits = header.device_features.read().into();
    header.device_features_sel.write(1); // device features [32, 64)
    device_features_bits = device_features_bits + ((header.device_features.read() as u64) << 32);
    let device_features = VirtIOGpuFeature::from_bits_truncate(device_features_bits);
    println!("Device features {:?}", device_features);

    // negotiate these flags only
    let supported_features = VirtIOGpuFeature::empty();
    let driver_features = (device_features & supported_features).bits();
    header.driver_features_sel.write(0); // driver features [0, 32)
    header.driver_features.write((driver_features & 0xFFFFFFFF) as u32);
    header.driver_features_sel.write(1); // driver features [32, 64)
    header.driver_features.write(((driver_features & 0xFFFFFFFF00000000) >> 32) as u32);

    // read configuration space
    let mut config = unsafe { &mut *((from + 0x100) as *mut VirtIOGpuConfig) };
    println!("Config: {:?}", config);

    // virtio 4.2.4 Legacy interface
    // configure two virtqueues: ingress and egress
    header.guest_page_size.write(PAGE_SIZE as u32); // one page

    let queue_num = 2;
    let mut driver = VirtIOGpu {
        interrupt: node.prop_u32("interrupts").unwrap(),
        interrupt_parent: node.prop_u32("interrupt-parent").unwrap(),
        header: from as usize,
        queue_num,
        queue_address: 0,
        queue_page: [0, 0],
        last_used_idx: 0
    };

    // 0 for control, 1 for cursor, we use controlq only
    for queue in 0..2 {
        header.queue_sel.write(queue);
        assert_eq!(header.queue_pfn.read(), 0); // not in use
        // 0 for transmit, 1 for receive
        let queue_num_max = header.queue_num_max.read();
        assert!(queue_num_max >= queue_num); // queue available
        let size = virtqueue_size(queue_num as usize, PAGE_SIZE);
        assert!(size % PAGE_SIZE == 0);
        // alloc continuous pages
        let address = unsafe {
            HEAP_ALLOCATOR.alloc_zeroed(Layout::from_size_align(size, PAGE_SIZE).unwrap())
        } as usize;

        println!("queue {} using page address {:#X} with size {}", queue, address as usize, size);

        header.queue_num.write(queue_num);
        header.queue_align.write(PAGE_SIZE as u32);
        header.queue_pfn.write((address as u32) >> 12);

        if queue == 0 {
            driver.queue_address = address;
            // 0 for transmit, 1 for receive
            for buffer in 0..2 {
                // allocate a page for each buffer
                let page = unsafe {
                    HEAP_ALLOCATOR.alloc_zeroed(Layout::from_size_align(PAGE_SIZE, PAGE_SIZE).unwrap())
                } as usize;
                driver.queue_page[buffer as usize] = page;
                println!("buffer {} using page address {:#X}", buffer, page as usize);
            }
        }
        header.queue_notify.write(queue);
    }
    header.status.write(VirtIODeviceStatus::DRIVER_OK.bits());

    setup_framebuffer(&mut driver);

    DRIVERS.lock().push(Box::new(driver));
}