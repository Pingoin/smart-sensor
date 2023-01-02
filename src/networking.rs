use esp32c3_hal as hal;
use hal::{prelude::*, Delay};
use embedded_svc::wifi::{Wifi};
use esp_println::{print, println};
use esp_wifi::wifi_interface::{Wifi as Wifinterface};
use embedded_svc::wifi::{Configuration,ClientConfiguration};
use heapless::String;

pub fn connect_wifi(wifi_interface: &mut Wifinterface, delay: &mut Delay,ssid:String<32>,pass:String<64>){

    println!("is wifi started: {:?}", wifi_interface.is_started());

    println!("Call wifi_connect");
    let client_config = Configuration::Client(ClientConfiguration {
        ssid: ssid,
        password: pass,
        ..Default::default()
    });
    let res = wifi_interface.set_configuration(&client_config);
    println!("wifi_set_configuration returned {:?}", res);
    println!("{:?}", wifi_interface.get_capabilities());
    println!("wifi_connect {:?}", wifi_interface.connect());
    // wait to get connected
    println!("Wait to get connected");
    loop {
        let res = wifi_interface.is_connected();
        match res {
            Ok(connected) => {
                if connected {
                    break;
                } else {
                    print!(".");
                    delay.delay_ms(100u32);
                }
            }
            Err(err) => {
                println!("{:?}", err);
                loop {}
            }
        }
    }
    println!("{:?}", wifi_interface.is_connected());
}
