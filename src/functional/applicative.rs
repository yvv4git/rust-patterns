// Функциональный паттерн Applicative(Аппликативный функтор): расширение функтора для применения функций в контексте.
// Аппликативный функтор позволяет применять функции, обернутые в контекст, к значениям в контексте.
// Полезен для работы с несколькими контекстами одновременно.
// Пример: применение функций к Option значениям.

/// Трейт для аппликативного функтора.
pub trait Applicative<T> {
    type Output<U>;
    fn pure(value: T) -> Self::Output<T>;
    fn apply<U, F>(self, f: Self::Output<F>) -> Self::Output<U>
    where
        F: Fn(T) -> U;
}

// Импортируем Functor для использования в Applicative
use super::functor::Functor;

/// Реализация Applicative для Option.
impl<T> Applicative<T> for Option<T> {
    type Output<U> = Option<U>;

    fn pure(value: T) -> Option<T> {
        Some(value)
    }

    fn apply<U, F>(self, f: Option<F>) -> Option<U>
    where
        F: Fn(T) -> U,
    {
        match (self, f) {
            (Some(value), Some(func)) => Some(func(value)),
            _ => None,
        }
    }
}

/// Пользовательский аппликативный функтор - Result с ошибкой.
#[derive(Debug, PartialEq)]
pub struct AppResult<T, E>(Result<T, E>);

impl<T, E> AppResult<T, E> {
    pub fn ok(value: T) -> Self {
        AppResult(Ok(value))
    }

    pub fn err(error: E) -> Self {
        AppResult(Err(error))
    }
}

impl<T, E> Functor<T> for AppResult<T, E> {
    type Output<U> = AppResult<U, E>;

    fn map<U, F>(self, f: F) -> AppResult<U, E>
    where
        F: Fn(T) -> U,
    {
        match self.0 {
            Ok(value) => AppResult::ok(f(value)),
            Err(e) => AppResult::err(e),
        }
    }
}

impl<T, E> Applicative<T> for AppResult<T, E> {
    type Output<U> = AppResult<U, E>;

    fn pure(value: T) -> AppResult<T, E> {
        AppResult::ok(value)
    }

    fn apply<U, F>(self, f: AppResult<F, E>) -> AppResult<U, E>
    where
        F: Fn(T) -> U,
    {
        match (self.0, f.0) {
            (Ok(value), Ok(func)) => AppResult::ok(func(value)),
            (Err(e), _) => AppResult::err(e),
            (_, Err(e)) => AppResult::err(e),
        }
    }
}

#[test]
fn test_applicative() {
    // Applicative для Option
    let value = Some(5);
    let func = Some(|x: i32| x * 2);
    let result = value.apply(func);
    assert_eq!(result, Some(10));

    let none_value: Option<i32> = None;
    let result_none = none_value.apply(Some(|x: i32| x * 2));
    assert_eq!(result_none, None);

    // Pure для Option
    let pure_val = Option::pure(42);
    assert_eq!(pure_val, Some(42));

    // Applicative для AppResult
    let ok_value: AppResult<i32, &str> = AppResult::ok(10);
    let ok_func: AppResult<fn(i32) -> i32, &str> = AppResult::ok(|x: i32| x + 5);
    let app_result = ok_value.apply(ok_func);
    assert_eq!(app_result, AppResult::ok(15));

    let err_value: AppResult<i32, &str> = AppResult::err("Error");
    let app_err = err_value.apply(AppResult::ok(|x: i32| x + 5));
    assert_eq!(app_err, AppResult::err("Error"));
}