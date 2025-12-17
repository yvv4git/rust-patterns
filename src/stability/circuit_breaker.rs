// Паттерн Circuit Breaker: предотвращает вызовы к неисправному сервису, автоматически открывая "цепь" при обнаружении сбоев.
// Полезен для защиты от каскадных сбоев в распределенных системах.
// Пример: защита от повторных вызовов к упавшему API.

use std::time::{Duration, Instant};

/// Состояния Circuit Breaker.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused)]
enum State {
    Closed,      // Нормальная работа, вызовы разрешены.
    Open,        // Цепь открыта, вызовы заблокированы.
    HalfOpen,    // Проверка восстановления, один вызов разрешен.
}

/// Структура Circuit Breaker.
pub struct CircuitBreaker {
    failure_threshold: usize,     // Порог неудач для открытия цепи.
    recovery_timeout: Duration,   // Время ожидания перед переходом в Half-Open.
    failure_count: usize,         // Текущий счетчик неудач.
    last_failure_time: Option<Instant>, // Время последней неудачи.
    state: State,
}

impl CircuitBreaker {
    /// Создает новый Circuit Breaker с порогом неудач и таймаутом восстановления.
    pub fn new(failure_threshold: usize, recovery_timeout: Duration) -> Self {
        CircuitBreaker {
            failure_threshold,
            recovery_timeout,
            failure_count: 0,
            last_failure_time: None,
            state: State::Closed,
        }
    }

    /// Вызывает функцию, если цепь позволяет.
    /// Возвращает Result: Ok(результат) или Err(ошибка).
    /// Если цепь открыта, возвращает Err без вызова функции.
    pub fn call<F, T, E>(&mut self, f: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: FnOnce() -> Result<T, E>,
    {
        match self.state {
            State::Closed => {
                match f() {
                    Ok(result) => {
                        self.failure_count = 0; // Сброс счетчика при успехе.
                        Ok(result)
                    }
                    Err(e) => {
                        self.failure_count += 1;
                        self.last_failure_time = Some(Instant::now());
                        if self.failure_count >= self.failure_threshold {
                            self.state = State::Open;
                        }
                        Err(CircuitBreakerError::ServiceError(e))
                    }
                }
            }
            State::Open => {
                if let Some(last_time) = self.last_failure_time {
                    if last_time.elapsed() >= self.recovery_timeout {
                        self.state = State::HalfOpen;
                        // Попытка вызова в Half-Open.
                        match f() {
                            Ok(result) => {
                                self.failure_count = 0;
                                self.state = State::Closed;
                                Ok(result)
                            }
                            Err(e) => {
                                self.state = State::Open;
                                self.last_failure_time = Some(Instant::now());
                                Err(CircuitBreakerError::ServiceError(e))
                            }
                        }
                    } else {
                        Err(CircuitBreakerError::CircuitOpen)
                    }
                } else {
                    Err(CircuitBreakerError::CircuitOpen)
                }
            }
            State::HalfOpen => {
                // В Half-Open уже обработано выше, но на случай.
                Err(CircuitBreakerError::CircuitOpen)
            }
        }
    }
}

/// Ошибки Circuit Breaker.
// #[derive(Debug, PartialEq)] - derive макросы для автоматической реализации трейтов:
// Debug - позволяет форматировать и печатать значение для отладки (println!("{:?}", error))
// PartialEq - позволяет сравнивать значения на равенство (assert_eq! в тестах)
#[derive(Debug, PartialEq)]
pub enum CircuitBreakerError<E> {
    CircuitOpen,       // Цепь открыта, вызов заблокирован.
    ServiceError(E),   // Ошибка от сервиса.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn circuit_breaker_closed_success() {
        let mut cb = CircuitBreaker::new(2, Duration::from_secs(1));
        let result: Result<i32, CircuitBreakerError<&str>> = cb.call(|| Ok(42));
        assert_eq!(result, Ok(42));
        assert_eq!(cb.state, State::Closed);
    }

    #[test]
    fn circuit_breaker_closed_failure() {
        let mut cb = CircuitBreaker::new(2, Duration::from_secs(1));
        let result: Result<i32, CircuitBreakerError<&str>> = cb.call(|| Err("error"));
        assert_eq!(result, Err(CircuitBreakerError::ServiceError("error")));
        assert_eq!(cb.failure_count, 1);
        assert_eq!(cb.state, State::Closed);
    }

    #[test]
    fn circuit_breaker_open_after_threshold() {
        let mut cb = CircuitBreaker::new(2, Duration::from_secs(1));
        let _: Result<(), CircuitBreakerError<&str>> = cb.call(|| Err("error1"));
        let _: Result<(), CircuitBreakerError<&str>> = cb.call(|| Err("error2"));
        assert_eq!(cb.state, State::Open);

        let result: Result<i32, CircuitBreakerError<&str>> = cb.call(|| Ok(42));
        assert_eq!(result, Err(CircuitBreakerError::CircuitOpen));
    }

    #[test]
    fn circuit_breaker_recovery() {
        let mut cb = CircuitBreaker::new(2, Duration::from_millis(100));
        let _: Result<(), CircuitBreakerError<&str>> = cb.call(|| Err("error1"));
        let _: Result<(), CircuitBreakerError<&str>> = cb.call(|| Err("error2"));
        assert_eq!(cb.state, State::Open);

        // Ждем таймаут.
        std::thread::sleep(Duration::from_millis(150));

        // Теперь Half-Open, и вызов успешен.
        let result: Result<i32, CircuitBreakerError<&str>> = cb.call(|| Ok(42));
        assert_eq!(result, Ok(42));
        assert_eq!(cb.state, State::Closed);
    }

    #[test]
    fn circuit_breaker_half_open_failure() {
        let mut cb = CircuitBreaker::new(2, Duration::from_millis(100));
        let _: Result<(), CircuitBreakerError<&str>> = cb.call(|| Err("error1"));
        let _: Result<(), CircuitBreakerError<&str>> = cb.call(|| Err("error2"));
        assert_eq!(cb.state, State::Open);

        std::thread::sleep(Duration::from_millis(150));

        // Вызов в Half-Open неудачен.
        let result: Result<i32, CircuitBreakerError<&str>> = cb.call(|| Err("error3"));
        assert_eq!(result, Err(CircuitBreakerError::ServiceError("error3")));
        assert_eq!(cb.state, State::Open);
    }
}