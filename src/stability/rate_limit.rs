// Паттерн Rate Limit: ограничивает количество запросов в единицу времени для предотвращения перегрузки.
// Полезен для защиты сервисов от DDoS-атак или чрезмерного использования.
// Пример: ограничение количества API-запросов в минуту.

use std::time::{Duration, Instant};

/// Структура для ограничения скорости запросов.
pub struct RateLimiter {
    max_requests: usize,
    window: Duration,
    requests: Vec<Instant>,
}

impl RateLimiter {
    /// Создает новый RateLimiter с максимальным количеством запросов и окном времени.
    pub fn new(max_requests: usize, window: Duration) -> Self {
        RateLimiter {
            max_requests,
            window,
            requests: Vec::new(),
        }
    }

    /// Проверяет, разрешен ли запрос, и обновляет счетчик если да.
    pub fn allow(&mut self) -> bool {
        let now = Instant::now();
        // Удалить запросы старше окна
        self.requests.retain(|&time| now.duration_since(time) <= self.window);
        if self.requests.len() < self.max_requests {
            self.requests.push(now);
            true
        } else {
            false
        }
    }
}

#[test]
fn rate_limit_test() {
    let mut limiter = RateLimiter::new(3, Duration::from_secs(1));

    // Должно разрешить первые 3 запроса
    assert!(limiter.allow());
    assert!(limiter.allow());
    assert!(limiter.allow());

    // Четвертый должен быть отклонен
    assert!(!limiter.allow());

    // Подождать больше секунды, чтобы окно обновилось
    std::thread::sleep(Duration::from_secs(2));

    // Теперь снова разрешено
    assert!(limiter.allow());
}