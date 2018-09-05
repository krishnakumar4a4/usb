extern crate libusb;
use std::time::Duration;
use std::slice;

fn main() {
    let context = libusb::Context::new().unwrap();

    for mut device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device {:03} ID {}:{} usb version {:?} manufacturer_string_index {:?}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
        	device_desc.usb_version(),
        	device_desc.manufacturer_string_index().unwrap());

        let dh = device.open().unwrap();
        let mut l = dh.read_languages(Duration::new(100,0)).unwrap();
        println!("device manufacturer string is {:?}", dh.read_manufacturer_string(l.pop().unwrap(),&device_desc,Duration::new(100,0)));
        // let mut vec: Vec<u8> = Vec::<u8>::with_capacity(64);
        //     let mut bulk_buf = unsafe {
        //         slice::from_raw_parts_mut(vec.as_mut_ptr(), vec.capacity())
        //     };
        // println!("created bulk_buf");
        // dh.read_bulk(1,&mut bulk_buf,Duration::new(100,0)).unwrap();
        // println!("bulk_buf {:?}", bulk_buf);

        // let num_configurations = device_desc.num_configurations();
        // for i in 0..num_configurations {
	       //  let config_descriptor = device.config_descriptor(i).unwrap();
	       //  println!("~config_descriptor: {}",i);
	       //  println!("~~number : {}", config_descriptor.number());
	       //  println!("~~max_power : {}", config_descriptor.max_power());
	       //  println!("~~self_powered : {}", config_descriptor.self_powered());
	       //  println!("~~remote_wakeup : {}", config_descriptor.remote_wakeup());
	       //  println!("~~description_string_index : {:?}", config_descriptor.description_string_index());
	       //  let num_interfaces = config_descriptor.num_interfaces();
	       //  println!("~~num_interfaces : {}", num_interfaces);

	       //  for interface in config_descriptor.interfaces() {
	       //  	println!("~~Interface: {}", interface.number());
	       //  	for interface_descriptor in interface.descriptors() {
	       //  		println!("~~~interface_number {}", interface_descriptor.interface_number());
	       //  		println!("~~~setting_number {}", interface_descriptor.setting_number());
	       //  		println!("~~~class_code {}", interface_descriptor.class_code());
	       //  		println!("~~~sub_class_code {}", interface_descriptor.sub_class_code());
	       //  		println!("~~~protocol_code {}", interface_descriptor.protocol_code());
	       //  		println!("~~~description_string_index {:?}", interface_descriptor.description_string_index());
	       //  		println!("~~~num_endpoints {}", interface_descriptor.num_endpoints());

	       //  		for endpoint_descriptor in interface_descriptor.endpoint_descriptors() {
	       //  			println!("~~~endpoint: {}", endpoint_descriptor.number());
	       //  			println!("~~~~address {}", endpoint_descriptor.address());
	       //  			println!("~~~~Direction {:?}", endpoint_descriptor.direction());
	       //  			println!("~~~~transfer_type {:?}", endpoint_descriptor.transfer_type());
	       //  			println!("~~~~sync_type {:?}", endpoint_descriptor.sync_type());
	       //  			println!("~~~~usage_type {:?}", endpoint_descriptor.usage_type());
	       //  			println!("~~~~max_packet_size {:?}", endpoint_descriptor.max_packet_size());
	       //  			println!("~~~~interval {:?}", endpoint_descriptor.interval());

	       //  		}
	       //  	}
	       //  }
        // }
    }

    // println!("{:?}", context.has_hotplug());

    // let d_h = match context.open_device_with_vid_pid(1008, 15879) {
    //     Some(device_handle) => device_handle,
    //     None => panic!("cannot open device")
    // };
    // println!("Found device deviceHandle {:?}",d_h.active_configuration().unwrap());
    // let mut l = d_h.read_languages(Duration::new(100,0)).unwrap();
    // println!("language is {:?}", d_h.read_string_descriptor(l.pop().unwrap(),0,Duration::new(100,0)));
}
