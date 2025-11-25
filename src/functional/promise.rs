// Функциональный паттерн Promise: представляет асинхронную операцию, которая может завершиться успехом или неудачей.
// Полезен для обработки асинхронных вычислений в функциональном стиле.
// Пример: цепочка операций с обработкой ошибок.

use std::fmt;

/// Перечисление для состояния Promise.
#[derive(Debug, Clone)]
pub enum Promise<T, E> {
    Pending,
    Fulfilled(T),
    Rejected(E),
}

impl<T, E> Promise<T, E> {
    /// Создает новый Promise в состоянии Pending.
    pub fn new() -> Self {
        Promise::Pending
    }

    /// Разрешает Promise с значением.
    pub fn resolve(self, value: T) -> Self {
        Promise::Fulfilled(value)
    }

    /// Отклоняет Promise с ошибкой.
    pub fn reject(self, error: E) -> Self {
        Promise::Rejected(error)
    }

    /// Применяет функцию к успешному значению, возвращая новый Promise.
    pub fn then<U, F>(self, f: F) -> Promise<U, E>
    where
        F: Fn(T) -> U,
    {
        match self {
            Promise::Fulfilled(value) => Promise::Fulfilled(f(value)),
            Promise::Rejected(error) => Promise::Rejected(error),
            Promise::Pending => Promise::Pending,
        }
    }

    /// Обрабатывает ошибку, возвращая новый Promise.
    pub fn catch<F>(self, f: F) -> Promise<T, E>
    where
        F: Fn(E) -> T,
    {
        match self {
            Promise::Rejected(error) => Promise::Fulfilled(f(error)),
            other => other,
        }
    }

    /// Проверяет, разрешен ли Promise.
    pub fn is_fulfilled(&self) -> bool {
        matches!(self, Promise::Fulfilled(_))
    }

    /// Проверяет, отклонен ли Promise.
    #[allow(dead_code)]
    pub fn is_rejected(&self) -> bool {
        matches!(self, Promise::Rejected(_))
    }
}

impl<T: fmt::Display, E: fmt::Display> fmt::Display for Promise<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Promise::Pending => write!(f, "Promise: Pending"),
            Promise::Fulfilled(value) => write!(f, "Promise: Fulfilled({})", value),
            Promise::Rejected(error) => write!(f, "Promise: Rejected({})", error),
        }
    }
}

#[test]
fn test_promise() {
    // Создание и разрешение Promise
    let promise = Promise::<i32, &str>::new()
        .resolve(42)
        .then(|x| x * 2);

    assert!(promise.is_fulfilled());
    println!("{}", promise);

    // Цепочка then
    let chained = Promise::<i32, &str>::new()
        .resolve(10)
        .then(|x| x + 5)
        .then(|x| x * 3);

    match chained {
        Promise::Fulfilled(value) => assert_eq!(value, 45),
        _ => panic!("Expected fulfilled"),
    }

    // Обработка ошибки
    let rejected = Promise::<i32, &str>::new()
        .reject("Error occurred")
        .catch(|_| 0);

    assert!(rejected.is_fulfilled());
    match rejected {
        Promise::Fulfilled(value) => assert_eq!(value, 0),
        _ => panic!("Expected fulfilled after catch"),
    }
}