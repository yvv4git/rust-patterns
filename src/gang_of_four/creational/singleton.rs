// Паттерн Singleton: обеспечивает, что у класса есть только один экземпляр,
// и предоставляет глобальную точку доступа к этому экземпляру.
// В Rust используется OnceLock для потокобезопасной инициализации.

use std::sync::OnceLock;

/// Структура Singleton, представляющая единственный экземпляр.
pub struct Singleton {
    data: String,
}

/// Реализация методов для Singleton.
impl Singleton {
    /// Приватный конструктор для создания нового экземпляра.
    fn new() -> Self {
        Singleton {
            data: "Hello, Singleton!".to_string(),
        }
    }

    /// Публичный метод для получения единственного экземпляра.
    /// Использует OnceLock для ленивой инициализации.
    pub fn instance() -> &'static Singleton {
        static INSTANCE: OnceLock<Singleton> = OnceLock::new();
        INSTANCE.get_or_init(|| Singleton::new())
    }

    /// Метод для получения данных из экземпляра.
    pub fn get_data(&self) -> &str {
        &self.data
    }
}

/// Тест для проверки паттерна Singleton.
/// Проверяет, что instance() возвращает один и тот же экземпляр.
#[test]
fn test_singleton() {
    let s1 = Singleton::instance();
    let s2 = Singleton::instance();
    assert_eq!(s1 as *const _, s2 as *const _); // Проверяем, что это один объект
    assert_eq!(s1.get_data(), "Hello, Singleton!"); // Проверяем данные
}