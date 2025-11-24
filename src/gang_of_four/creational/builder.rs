// Паттерн Builder: позволяет создавать сложные объекты шаг за шагом.
// Полезен для объектов с множеством параметров или сложной инициализацией.
// Пример: сборка автомобиля.

/// Структура автомобиля.
#[derive(Debug, Clone, PartialEq)]
pub struct Car {
    pub seats: u32,
    pub engine: String,
    pub trip_computer: bool,
    pub gps: bool,
}

/// Трейт для строителя автомобиля.
pub trait CarBuilder {
    fn reset(&mut self);
    fn set_seats(&mut self, seats: u32);
    fn set_engine(&mut self, engine: String);
    fn set_trip_computer(&mut self);
    fn set_gps(&mut self);
    fn get_result(&self) -> Car;
}

/// Конкретный строитель автомобиля.
pub struct ConcreteCarBuilder {
    car: Car,
}

impl ConcreteCarBuilder {
    pub fn new() -> Self {
        ConcreteCarBuilder {
            car: Car {
                seats: 0,
                engine: String::new(),
                trip_computer: false,
                gps: false,
            },
        }
    }
}

impl CarBuilder for ConcreteCarBuilder {
    fn reset(&mut self) {
        self.car = Car {
            seats: 0,
            engine: String::new(),
            trip_computer: false,
            gps: false,
        };
    }

    fn set_seats(&mut self, seats: u32) {
        self.car.seats = seats;
    }

    fn set_engine(&mut self, engine: String) {
        self.car.engine = engine;
    }

    fn set_trip_computer(&mut self) {
        self.car.trip_computer = true;
    }

    fn set_gps(&mut self) {
        self.car.gps = true;
    }

    fn get_result(&self) -> Car {
        self.car.clone()
    }
}

/// Директор, управляющий строительством.
pub struct Director {
    builder: Box<dyn CarBuilder>,
}

impl Director {
    pub fn new(builder: Box<dyn CarBuilder>) -> Self {
        Director { builder }
    }

    pub fn construct_sports_car(&mut self) -> Car {
        self.builder.reset();
        self.builder.set_seats(2);
        self.builder.set_engine("SportEngine".to_string());
        self.builder.set_trip_computer();
        self.builder.set_gps();
        self.builder.get_result()
    }

    pub fn construct_suv(&mut self) -> Car {
        self.builder.reset();
        self.builder.set_seats(4);
        self.builder.set_engine("SUVEngine".to_string());
        self.builder.set_gps();
        self.builder.get_result()
    }
}

/// Тест для паттерна Builder.
#[test]
fn test_builder() {
    let builder = Box::new(ConcreteCarBuilder::new());
    let mut director = Director::new(builder);

    let sports_car = director.construct_sports_car();
    assert_eq!(sports_car.seats, 2);
    assert_eq!(sports_car.engine, "SportEngine");
    assert!(sports_car.trip_computer);
    assert!(sports_car.gps);

    let suv = director.construct_suv();
    assert_eq!(suv.seats, 4);
    assert_eq!(suv.engine, "SUVEngine");
    assert!(!suv.trip_computer);
    assert!(suv.gps);
}