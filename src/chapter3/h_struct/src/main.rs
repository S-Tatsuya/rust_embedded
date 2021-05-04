struct Sensor {
    active: bool,
    latest: u32,
}

impl Sensor {
    fn read(&self) -> u32 {
        self.latest
    }

    fn init(&mut self) {
        self.active = true;
        self.latest = 42;
    }
}

impl Sensor {
    fn new() -> Sensor {
        Sensor {
            active:false,
            latest: 0,
        }
    }
}

fn main() {
    println!("構造体");
    let sensor = Sensor {
        active: false,
        latest: 0,
    };

    println!("active = {}, latest = {}", sensor.active, sensor.latest);

    println!("");
    println!("構造体:mut");
    let mut sensor = Sensor {
        active:false,
        latest: 0,
    };
    sensor.latest = 42;
    println!("acrive = {}, latest = {}", sensor.active, sensor.latest);

    println!("");
    println!("impl");
    let mut sensor = Sensor {
        active: false,
        latest: 0,
    };
    sensor.init();
    let latest = sensor.read();
    println!("latest = {}", latest);

    println!("");
    println!("impl:コンストラクタ");
    let sensor = Sensor::new();
    let latest = sensor.read();
    println!("latest = {}", latest);
}