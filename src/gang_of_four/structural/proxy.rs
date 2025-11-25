// Паттерн Proxy: предоставляет заместителя для другого объекта, контролируя доступ к нему.
// Полезен для ленивой загрузки, кеширования, контроля доступа.
// Пример: прокси для изображения с ленивой загрузкой.
//
// Что делает: Контролирует доступ к объекту, добавляя дополнительную логику (например, кэширование, логирование, контроль доступа).
// Интерфейс: Предоставляет точно такой же интерфейс, как у оригинального объекта.
// Цель:
// - Скрыть детали реализации оригинального объекта.
// - Управлять доступом к объекту (например, ленивая инициализация).
use std::cell::RefCell;

/// Трейт для изображения.
pub trait Image {
    fn display(&self);
}

/// Реальное изображение, которое загружается из файла.
pub struct RealImage {
    filename: String,
}

impl RealImage {
    pub fn new(filename: &str) -> Self {
        let image = RealImage {
            filename: filename.to_string(),
        };
        image.load_from_disk();
        image
    }

    fn load_from_disk(&self) {
        println!("Загрузка изображения '{}' из диска", self.filename);
    }
}

impl Image for RealImage {
    fn display(&self) {
        println!("Отображение изображения '{}'", self.filename);
    }
}

/// Прокси-изображение, которое загружает реальное изображение лениво.
pub struct ProxyImage {
    filename: String,
    real_image: RefCell<Option<RealImage>>,
}

impl ProxyImage {
    pub fn new(filename: &str) -> Self {
        ProxyImage {
            filename: filename.to_string(),
            real_image: RefCell::new(None),
        }
    }
}

impl Image for ProxyImage {
    fn display(&self) {
        if self.real_image.borrow().is_none() {
            *self.real_image.borrow_mut() = Some(RealImage::new(&self.filename));
        }
        self.real_image.borrow().as_ref().unwrap().display();
    }
}

/// Тест для паттерна Proxy.
#[test]
fn test_proxy() {
    let image = ProxyImage::new("photo.jpg");

    // Первое отображение - загрузка и отображение
    image.display();
    // Второе отображение - только отображение, без загрузки
    image.display();
}