// Паттерн Object Pool: управляет пулом переиспользуемых объектов.
// Полезен для дорогих в создании объектов, таких как соединения.
// Пример: пул строковых объектов.

use std::collections::VecDeque;

/// Трейт для объектов пула.
pub trait PoolObject {
    fn reset(&mut self);
}

/// Простой объект пула (строка).
#[derive(Debug, PartialEq)]
pub struct PooledString {
    pub value: String,
}

impl PoolObject for PooledString {
    fn reset(&mut self) {
        self.value.clear();
    }
}

/// Пул объектов.
pub struct ObjectPool<T: PoolObject> {
    available: VecDeque<T>,
    created_count: usize,
}

impl<T: PoolObject> ObjectPool<T> {
    pub fn new() -> Self {
        ObjectPool {
            available: VecDeque::new(),
            created_count: 0,
        }
    }

    /// Получить объект из пула.
    pub fn acquire<F>(&mut self, factory: F) -> T
    where
        F: FnOnce() -> T,
    {
        if let Some(mut obj) = self.available.pop_front() {
            obj.reset();
            obj
        } else {
            self.created_count += 1;
            factory()
        }
    }

    /// Вернуть объект в пул.
    pub fn release(&mut self, mut obj: T) {
        obj.reset();
        self.available.push_back(obj);
    }

    pub fn available_count(&self) -> usize {
        self.available.len()
    }

    pub fn created_count(&self) -> usize {
        self.created_count
    }
}

/// Тест для паттерна Object Pool.
#[test]
fn test_object_pool() {
    let mut pool = ObjectPool::new();

    // Создать объект через фабрику
    let obj1 = pool.acquire(|| PooledString { value: "new".to_string() });
    assert_eq!(obj1.value, "new");
    assert_eq!(pool.created_count(), 1);

    // Вернуть в пул
    pool.release(obj1);
    assert_eq!(pool.available_count(), 1);

    // Получить из пула
    let obj2 = pool.acquire(|| PooledString { value: "unused".to_string() });
    assert_eq!(obj2.value, ""); // сброшен
    assert_eq!(pool.created_count(), 1); // не создали новый
}