// Функциональный паттерн Memoization: кеширование результатов функций.
// Позволяет избежать повторных вычислений для одних и тех же аргументов.
// Полезен для оптимизации рекурсивных функций или дорогих вычислений.
// Пример: мемоизация функции Фибоначчи.

use std::collections::HashMap;
use std::hash::Hash;

/// Структура для мемоизации функции.
pub struct Memoized<T, U, F>
where
    T: Eq + Hash + Clone + std::fmt::Debug,
    U: Clone,
    F: Fn(T) -> U,
{
    func: F,
    cache: HashMap<T, U>,
}

impl<T, U, F> Memoized<T, U, F>
where
    T: Eq + Hash + Clone + std::fmt::Debug,
    U: Clone,
    F: Fn(T) -> U,
{
    /// Создает новую мемоизацию для функции.
    pub fn new(func: F) -> Self {
        Memoized {
            func,
            cache: HashMap::new(),
        }
    }

    /// Вызывает функцию с мемоизацией.
    pub fn call(&mut self, arg: T) -> U {
        if let Some(result) = self.cache.get(&arg) {
            println!("Возвращаем кешированный результат для {:?}", arg);
            result.clone()
        } else {
            println!("Вычисляем результат для {:?}", arg);
            let result = (self.func)(arg.clone());
            self.cache.insert(arg, result.clone());
            result
        }
    }

    /// Возвращает размер кеша.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

/// Функция для создания мемоизации.
pub fn memoize<T, U, F>(func: F) -> Memoized<T, U, F>
where
    T: Eq + Hash + Clone + std::fmt::Debug,
    U: Clone,
    F: Fn(T) -> U,
{
    Memoized::new(func)
}

#[test]
fn test_memoization() {
    // Функция Фибоначчи без мемоизации (для сравнения)
    fn fib(n: u64) -> u64 {
        if n <= 1 {
            n
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    // Мемоизация Фибоначчи
    let mut memo_fib = memoize(|n: u64| {
        if n <= 1 {
            n
        } else {
            // В реальном коде здесь нужно рекурсивно вызывать мемоизацию,
            // но для простоты используем обычную функцию
            fib(n - 1) + fib(n - 2)
        }
    });

    // Первый вызов - вычисление
    let result1 = memo_fib.call(5);
    assert_eq!(result1, 5);

    // Второй вызов того же аргумента - из кеша
    let result2 = memo_fib.call(5);
    assert_eq!(result2, 5);

    // Размер кеша
    assert_eq!(memo_fib.cache_size(), 1);

    // Другой аргумент
    let result3 = memo_fib.call(3);
    assert_eq!(result3, 2);
    assert_eq!(memo_fib.cache_size(), 2);
}