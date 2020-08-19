extern crate libusb;
use libusb::{Device, Context, DeviceHandle};
use std::time::Duration;
use std::thread;

fn main() {
	let context = libusb::Context::new().unwrap();
	// Get device by bus num and device id
	// obtained with lsusb command
	let selected_device = get_device(&context, 1, 3);
	if let Some(dev) = selected_device {
		let mut dev_hdl = dev.open().unwrap();
		let device_desc = dev.device_descriptor().unwrap();
		let langs = dev_hdl.read_languages(Duration::from_secs(1)).unwrap();
		println!("  iProduct             {:3} {}",
             device_desc.product_string_index().unwrap_or(0),
			 dev_hdl.read_product_string(langs[0], &device_desc, Duration::from_secs(1)).unwrap_or(String::new()));
		detach_kernel_driver(&mut dev_hdl);
		claim_interface(&mut dev_hdl);
		read_interrupt_data(&mut dev_hdl);
	}
}

fn read_interrupt_data(dev_hdl: &mut DeviceHandle) {
	loop {
		let mut buf:[u8;256] = [0;256];
		println!("attempting read from ifc");
		// 130 is endpoint descriptor address of Dell wireless mouse for 0x82
		// Endpoint descriptor details can be found with lsusb -v command 
		match dev_hdl.read_interrupt(130, &mut buf, Duration::from_secs(1)) {
			Ok(res) => println!("res: {:?}, data: {:?}", res, &buf[..res]),
			Err(e) => println!("error reading interrupt: {:?}",e)
		}
		thread::sleep(Duration::from_millis(1000));
	}
}

fn claim_interface(device_hdl: &mut DeviceHandle) {
	match device_hdl.claim_interface(1) {
		Ok(_) => (),
		Err(e) => println!("error claiming interface: {}", e)
	}
}

fn detach_kernel_driver(device_hdl: &mut DeviceHandle) {
	match device_hdl.detach_kernel_driver(1) {
		Ok(_) => (),
		Err(e) => println!("error detaching kernel driver: {}", e)
	}
}

fn get_device(context: &Context, bus_num: u8, addr: u8) -> Option<Device>{
	for device in context.devices().unwrap().iter() {
		if device.bus_number() == bus_num && device.address() == addr {
			return Some(device)
		}
	}
	return None
}
