use std::error::Error;
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
    fn to_csv(self: Cable) -> &[String] {
        let string = [self.port1.to_string(), self.port2.to_string(), self.length_in_m.to_string()]; 
        &string
    }
}

fn main() {
    let test_cable = Cable { port1: Port::UsbC , port2: Port::UsbA, length_in_m: 1.5 };
    write_cable(test_cable);
}


fn write_cable(cable: Cable) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("data.csv")?;
    wtr.write_record(&cable.to_csv())?;
    wtr.flush()?;
    Ok(())
}