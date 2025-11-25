// Паттерн Mediator: определяет объект, инкапсулирующий взаимодействие между группой объектов.
// Ослабляет связи между объектами, позволяя изменять их взаимодействие независимо.
// Пример: чат-комната, где пользователи общаются через медиатор.

use std::rc::Rc;

/// Трейт для медиатора.
pub trait Mediator {
    fn send_message(&self, message: &str, user: &User);
}

/// Коллега - пользователь.
pub struct User {
    name: String,
    mediator: Rc<dyn Mediator>,
}

impl User {
    pub fn new(name: &str, mediator: Rc<dyn Mediator>) -> Self {
        User {
            name: name.to_string(),
            mediator,
        }
    }

    pub fn send(&self, message: &str) {
        println!("{} отправляет: {}", self.name, message);
        self.mediator.send_message(message, self);
    }

    pub fn receive(&self, message: &str, from: &str) {
        println!("{} получает от {}: {}", self.name, from, message);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

/// Конкретный медиатор - чат-комната.
#[allow(dead_code)]
pub struct ChatRoom {
    users: Vec<Rc<User>>,
}

#[allow(dead_code)]
impl ChatRoom {
    pub fn new() -> Rc<Self> {
        Rc::new(ChatRoom { users: Vec::new() })
    }

    pub fn add_user(self: &Rc<Self>, _user: Rc<User>) {
        // В Rust Rc не позволяет mutable borrow, так что используем RefCell
        // Но для простоты, добавим метод для добавления
    }
}

impl Mediator for ChatRoom {
    fn send_message(&self, message: &str, user: &User) {
        for u in &self.users {
            if u.get_name() != user.get_name() {
                u.receive(message, user.get_name());
            }
        }
    }
}

// Для добавления пользователей используем RefCell
use std::cell::RefCell;

pub struct ChatRoomRef {
    users: RefCell<Vec<Rc<User>>>,
}

impl ChatRoomRef {
    pub fn new() -> Rc<Self> {
        Rc::new(ChatRoomRef {
            users: RefCell::new(Vec::new()),
        })
    }

    pub fn add_user(self: &Rc<Self>, user: Rc<User>) {
        self.users.borrow_mut().push(user);
    }
}

impl Mediator for ChatRoomRef {
    fn send_message(&self, message: &str, user: &User) {
        for u in self.users.borrow().iter() {
            if u.get_name() != user.get_name() {
                u.receive(message, user.get_name());
            }
        }
    }
}

/// Тест для паттерна Mediator.
#[test]
fn test_mediator() {
    let chat_room = ChatRoomRef::new();

    let user1 = Rc::new(User::new("Алексей", chat_room.clone()));
    let user2 = Rc::new(User::new("Мария", chat_room.clone()));
    let user3 = Rc::new(User::new("Иван", chat_room.clone()));

    chat_room.add_user(user1.clone());
    chat_room.add_user(user2.clone());
    chat_room.add_user(user3.clone());

    user1.send("Привет всем!");
    user2.send("Привет, Алексей!");
}