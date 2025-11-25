// Функциональный паттерн Currying: преобразование функции с несколькими аргументами
// в последовательность функций с одним аргументом.
// Полезен для частичного применения функций и создания специализированных функций.
// Пример: каррированная функция сложения.

/// Каррированная функция сложения: принимает первый аргумент и возвращает функцию для второго.
pub fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// Функция умножения с тремя аргументами, каррированная.
pub fn multiply(x: i32) -> Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>> {
    Box::new(move |y| Box::new(move |z| x * y * z))
}

/// Функция для форматирования строки с каррированием.
pub fn format_greeting(prefix: &str) -> impl Fn(&str) -> String + '_ {
    let prefix = prefix.to_string();
    move |name| format!("{} {}", prefix, name)
}

#[test]
fn test_currying() {
    // Простое каррирование
    let add_five = add(5);
    assert_eq!(add_five(3), 8);
    assert_eq!(add_five(10), 15);

    // Каррирование с тремя аргументами
    let multiply_by_two = multiply(2);
    let multiply_by_four = multiply_by_two(2);
    assert_eq!(multiply_by_four(5), 20);

    // Частичное применение
    let greet_hello = format_greeting("Hello");
    assert_eq!(greet_hello("World"), "Hello World");

    let greet_hi = format_greeting("Hi");
    assert_eq!(greet_hi("Rust"), "Hi Rust");
}