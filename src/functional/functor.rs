// Функциональный паттерн Functor: абстракция для применения функций к значениям в контексте.
// Функтор позволяет "поднимать" функцию в контекст, применяя её к обернутым значениям.
// Полезен для работы с контейнерами, опциями и другими контекстами.
// Пример: применение функций к Option и Vec.

/// Трейт для функтора.
pub trait Functor<T> {
    type Output<U>;
    fn map<U, F>(self, f: F) -> Self::Output<U>
    where
        F: Fn(T) -> U;
}

/// Реализация функтора для Option.
impl<T> Functor<T> for Option<T> {
    type Output<U> = Option<U>;

    fn map<U, F>(self, f: F) -> Option<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}

/// Реализация функтора для Vec.
impl<T> Functor<T> for Vec<T> {
    type Output<U> = Vec<U>;

    fn map<U, F>(self, f: F) -> Vec<U>
    where
        F: Fn(T) -> U,
    {
        self.into_iter().map(f).collect()
    }
}

/// Пользовательский функтор - Maybe (простая обертка над Option).
#[derive(Debug, PartialEq)]
pub struct Maybe<T>(Option<T>);

impl<T> Maybe<T> {
    pub fn some(value: T) -> Self {
        Maybe(Some(value))
    }

    pub fn none() -> Self {
        Maybe(None)
    }
}

impl<T> Functor<T> for Maybe<T> {
    type Output<U> = Maybe<U>;

    fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: Fn(T) -> U,
    {
        match self.0 {
            Some(value) => Maybe::some(f(value)),
            None => Maybe::none(),
        }
    }
}

#[test]
fn test_functor() {
    // Functor для Option
    let some_val: Option<i32> = Some(5);
    let mapped_some = some_val.map(|x| x * 2);
    assert_eq!(mapped_some, Some(10));

    let none_val: Option<i32> = None;
    let mapped_none = none_val.map(|x| x * 2);
    assert_eq!(mapped_none, None);

    // Functor для Vec
    let vec_val = vec![1, 2, 3];
    let mapped_vec = vec_val.map(|x| x * 3);
    assert_eq!(mapped_vec, vec![3, 6, 9]);

    // Functor для Maybe
    let maybe_some = Maybe::some(42);
    let mapped_maybe = maybe_some.map(|x| x + 8);
    assert_eq!(mapped_maybe, Maybe::some(50));

    let maybe_none = Maybe::<i32>::none();
    let mapped_maybe_none = maybe_none.map(|x| x + 8);
    assert_eq!(mapped_maybe_none, Maybe::none());
}