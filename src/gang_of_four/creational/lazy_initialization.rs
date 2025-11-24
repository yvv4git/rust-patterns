// Паттерн Lazy Initialization: отложенная инициализация объектов до первого использования.
// Полезен для дорогих операций или ресурсов.
// В Rust реализуется через OnceLock для потокобезопасности.

use std::sync::OnceLock;

/// Структура с ленивой инициализацией.
pub struct LazyExample {
    expensive_data: OnceLock<String>,
}

impl LazyExample {
    pub fn new() -> Self {
        LazyExample {
            expensive_data: OnceLock::new(),
        }
    }

    /// Получить данные с ленивой инициализацией.
    pub fn get_expensive_data(&self) -> &String {
        self.expensive_data.get_or_init(|| {
            // Имитация дорогой операции
            println!("Инициализация дорогих данных...");
            "Дорогие данные".to_string()
        })
    }
}

/// Глобальная ленивая инициализация.
static GLOBAL_DATA: OnceLock<String> = OnceLock::new();

pub fn get_global_data() -> &'static String {
    GLOBAL_DATA.get_or_init(|| {
        println!("Инициализация глобальных данных...");
        "Глобальные данные".to_string()
    })
}

/// Тест для паттерна Lazy Initialization.
#[test]
fn test_lazy_initialization() {
    let example = LazyExample::new();

    // Первый вызов инициализирует
    let data1 = example.get_expensive_data();
    assert_eq!(data1, "Дорогие данные");

    // Второй вызов использует кэшированное значение
    let data2 = example.get_expensive_data();
    assert_eq!(data2, "Дорогие данные");

    // Глобальная
    let global1 = get_global_data();
    assert_eq!(global1, "Глобальные данные");

    let global2 = get_global_data();
    assert_eq!(global2, "Глобальные данные");
}