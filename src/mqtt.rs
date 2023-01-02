use embedded_io::blocking::{Write,Read};
use esp_println::println;
use esp_wifi::{wifi_interface::Socket, current_millis};
use gmqtt::{
    control_packet::{connect::ConnectProperties, Connect, Packet, Publish},
    write_packet,
};


const MAX_MQTT_PACKET_SIZE: u32 = 1024;

pub fn connect(tcp:&mut Socket){
    let keep_alive: u16 = 60; // 60 seconds
    let properties = ConnectProperties {
        topic_alias_max: Some(0),
        max_packet_size: Some(MAX_MQTT_PACKET_SIZE),
        ..ConnectProperties::default()
    };
    let connect_packet = Packet::Connect(Connect {
        protocol: gmqtt::Protocol::V5,
        clean_start: false,
        keep_alive,
        client_id: "our client id",
        last_will: None,
        login: None,
        properties: Some(properties),
    });
    let mut buffer = [0x00u8; MAX_MQTT_PACKET_SIZE as usize];

    let len = write_packet(&connect_packet, &mut buffer).unwrap();

    tcp.write(&buffer[..len]).unwrap();
    tcp.flush().unwrap();

    let wait_end = current_millis() + 2 * 1000;
    loop {
        let mut buffer = [0u8; 512];
        if let Ok(len) = tcp.read(&mut buffer) {
            if len > 0 {
                //let read_packet = read_packet(&buffer[..len]).unwrap();
                //print!("{:?}", read_packet);
                break;
            }
        } else {
            break;
        }

        if current_millis() > wait_end {
            println!("Timeout");
            break;
        }
    }
}

pub fn publish(tcp:&mut Socket,topic:&str, payload:&[u8] ){
    let pub_pack = Packet::Publish(Publish {
        dup: false,
        qospid: gmqtt::QosPid::AtMostOnce,
        retain: false,
        topic: topic,
        properties: None,
        payload:payload,
    });
    let mut buffer = [0x00u8; MAX_MQTT_PACKET_SIZE as usize];

    let len = write_packet(&pub_pack, &mut buffer).unwrap();

    tcp.write(&buffer[..len]).unwrap();
    tcp.flush().unwrap();
}