// Паттерн Failover: переключается на резервный сервис при отказе основного.
// Полезен для обеспечения высокой доступности в распределенных системах.
// Пример: переключение на backup сервер при недоступности primary.

/// Структура для управления failover.
/// services: вектор функций, представляющих сервисы (от primary к backup).
pub struct Failover<T, E> {
    services: Vec<Box<dyn Fn() -> Result<T, E>>>,
}

impl<T, E> Failover<T, E> {
    /// Создает новый Failover с вектором сервисов.
    pub fn new(services: Vec<Box<dyn Fn() -> Result<T, E>>>) -> Self {
        Failover { services }
    }

    /// Вызывает сервисы по порядку, возвращает результат первого успешного.
    /// Если все неудачны, возвращает ошибку последнего.
    pub fn call(&self) -> Result<T, E> {
        let mut last_error = None;
        for service in &self.services {
            match service() {
                Ok(result) => return Ok(result),
                Err(e) => last_error = Some(e),
            }
        }
        Err(last_error.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failover_success_first_service() {
        let failover: Failover<i32, &str> = Failover::new(vec![
            Box::new(|| Ok(1)),
            Box::new(|| Ok(2)),
        ]);
        let result = failover.call();
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn failover_success_after_fail() {
        let failover: Failover<i32, &str> = Failover::new(vec![
            Box::new(|| Err("primary failed")),
            Box::new(|| Ok(2)),
        ]);
        let result = failover.call();
        assert_eq!(result, Ok(2));
    }

    #[test]
    fn failover_all_fail() {
        let failover: Failover<i32, &str> = Failover::new(vec![
            Box::new(|| Err("error1")),
            Box::new(|| Err("error2")),
        ]);
        let result = failover.call();
        assert_eq!(result, Err("error2"));
    }
}