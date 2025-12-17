// Паттерн Retry: автоматически повторяет неудачные операции с задержкой.
// Полезен для обработки временных сбоев, таких как сетевые ошибки.
// Пример: повтор запроса к API при таймауте.

use std::thread;
use std::time::Duration;

/// Выполняет функцию с повтором при неудаче.
/// max_attempts: максимальное количество попыток (включая первую).
/// delay: задержка между попытками.
pub fn retry<F, T, E>(mut f: F, max_attempts: usize, delay: Duration) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    for attempt in 1..=max_attempts {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt < max_attempts {
                    thread::sleep(delay);
                } else {
                    return Err(e);
                }
            }
        }
    }
    unreachable!()
}

#[test]
fn test_retry_success_first_attempt() {
    let mut call_count = 0;
    let result: Result<i32, &str> = retry(
        || {
            call_count += 1;
            Ok(42)
        },
        3,
        Duration::from_millis(10),
    );
    assert_eq!(result, Ok(42));
    assert_eq!(call_count, 1);
}

#[test]
fn test_retry_success_after_retries() {
    let mut call_count = 0;
    let result: Result<i32, &str> = retry(
        || {
            call_count += 1;
            if call_count < 3 {
                Err("temporary error")
            } else {
                Ok(42)
            }
        },
        5,
        Duration::from_millis(1), // Короткая задержка для теста
    );
    assert_eq!(result, Ok(42));
    assert_eq!(call_count, 3);
}

#[test]
fn test_retry_failure_all_attempts() {
    let mut call_count = 0;
    let result: Result<i32, &str> = retry(
        || {
            call_count += 1;
            Err("persistent error")
        },
        3,
        Duration::from_millis(1),
    );
    assert_eq!(result, Err("persistent error"));
    assert_eq!(call_count, 3);
}