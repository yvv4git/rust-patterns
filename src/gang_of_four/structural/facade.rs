// Паттерн Facade: предоставляет унифицированный интерфейс к набору интерфейсов в подсистеме.
// Полезен для упрощения сложных систем.
// Пример: домашний кинотеатр.

/// Подсистема: Усилитель.
pub struct Amplifier;

impl Amplifier {
    pub fn on(&self) {
        println!("Усилитель включен");
    }

    pub fn off(&self) {
        println!("Усилитель выключен");
    }

    pub fn set_volume(&self, level: i32) {
        println!("Громкость усилителя установлена на {}", level);
    }
}

/// Подсистема: Проигрыватель DVD.
pub struct DvdPlayer;

impl DvdPlayer {
    pub fn on(&self) {
        println!("DVD-проигрыватель включен");
    }

    pub fn off(&self) {
        println!("DVD-проигрыватель выключен");
    }

    pub fn play(&self, movie: &str) {
        println!("Воспроизведение фильма: {}", movie);
    }

    pub fn stop(&self) {
        println!("Остановка воспроизведения");
    }
}

/// Подсистема: Проектор.
pub struct Projector;

impl Projector {
    pub fn on(&self) {
        println!("Проектор включен");
    }

    pub fn off(&self) {
        println!("Проектор выключен");
    }

    pub fn wide_screen_mode(&self) {
        println!("Проектор переключен в режим широкоэкранного изображения");
    }
}

/// Подсистема: Экран.
pub struct Screen;

impl Screen {
    pub fn up(&self) {
        println!("Экран поднят");
    }

    pub fn down(&self) {
        println!("Экран опущен");
    }
}

/// Подсистема: Подсветка.
pub struct TheaterLights;

impl TheaterLights {
    pub fn on(&self) {
        println!("Подсветка включена");
    }

    #[allow(dead_code)]
    pub fn off(&self) {
        println!("Подсветка выключена");
    }

    pub fn dim(&self, level: i32) {
        println!("Подсветка приглушена до {}", level);
    }
}

/// Фасад: Домашний кинотеатр.
pub struct HomeTheaterFacade {
    amp: Amplifier,
    dvd: DvdPlayer,
    projector: Projector,
    screen: Screen,
    lights: TheaterLights,
}

impl HomeTheaterFacade {
    pub fn new() -> Self {
        HomeTheaterFacade {
            amp: Amplifier,
            dvd: DvdPlayer,
            projector: Projector,
            screen: Screen,
            lights: TheaterLights,
        }
    }

    pub fn watch_movie(&self, movie: &str) {
        println!("Подготовка к просмотру фильма...");
        self.lights.dim(10);
        self.screen.down();
        self.projector.on();
        self.projector.wide_screen_mode();
        self.amp.on();
        self.amp.set_volume(5);
        self.dvd.on();
        self.dvd.play(movie);
        println!("Наслаждайтесь фильмом!");
    }

    pub fn end_movie(&self) {
        println!("Завершение просмотра фильма...");
        self.dvd.stop();
        self.dvd.off();
        self.amp.off();
        self.projector.off();
        self.screen.up();
        self.lights.on();
        println!("Фильм окончен.");
    }
}

/// Тест для паттерна Facade.
#[test]
fn test_facade() {
    let home_theater = HomeTheaterFacade::new();
    home_theater.watch_movie("Интерстеллар");
    home_theater.end_movie();
    // Тест проходит, если не паникует и логи выводятся корректно.
}