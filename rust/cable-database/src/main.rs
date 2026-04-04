use std::error::Error;
use text_io::read;
use csv::Writer;


enum Port {
    UsbC,
    UsbA,
    Hdmi,
    DisplayPort,
}

impl ToString for Port {
    fn to_string(&self) -> String {
        match self {
            Port::UsbC => "USB-C",
            Port::UsbA => "USB-A",
            Port::Hdmi => "HDMI",
            Port::DisplayPort => "DisplayPort",
        }
        .to_string()
    }
}

struct Cable {
    port1: Port,
    port2: Port,
    length_in_m: f32
}



impl Cable {
    fn to_csv(self: &Cable) -> Vec<String>{
        let string = [self.port1.to_string(), self.port2.to_string(), self.length_in_m.to_string()]; 
        (&string).to_vec()
    }
}

fn main() {
    loop {
        println!("What do you want to do? (1: add cable)");
        let user_option: i32 = read!();
        if user_option == 1 {
            println!("Please enter the cable Port1:");
            let port1: String = read!();
            println!("Please enter the cable Port2:");
            let port2: String = read!();
            println!("Please enter the cable length in m:");
            let length: f32 = read!();

            let cable = Cable { port1: Port::UsbC , port2: Port::UsbA, length_in_m: 1.5 };

        }
        let test_cable = Cable { port1: Port::UsbC , port2: Port::UsbA, length_in_m: 1.5 };

        write_cable(test_cable);
    }

}


fn write_cable(cable: Cable) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("data.csv")?;
    wtr.write_record(&cable.to_csv())?;
    wtr.flush()?;
    Ok(())
}
