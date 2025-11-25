// Паттерн State: позволяет объекту изменять свое поведение при изменении внутреннего состояния.
// Объект будет выглядеть так, будто он изменил свой класс.
// Полезен для реализации конечных автоматов.
// Пример: автомат по продаже напитков.

use std::rc::Rc;
use std::cell::RefCell;

/// Трейт для состояния.
pub trait State {
    fn insert_coin(&self, machine: &VendingMachine) -> Option<Rc<dyn State>>;
    fn eject_coin(&self, machine: &VendingMachine) -> Option<Rc<dyn State>>;
    fn dispense(&self, machine: &VendingMachine) -> Option<Rc<dyn State>>;
}

/// Контекст - автомат по продаже напитков.
pub struct VendingMachine {
    state: RefCell<Rc<dyn State>>,
    count: RefCell<i32>,
}

impl VendingMachine {
    pub fn new(count: i32) -> Self {
        VendingMachine {
            state: RefCell::new(Rc::new(NoCoinState)),
            count: RefCell::new(count),
        }
    }

    #[allow(dead_code)]
    pub fn set_state(&self, state: Rc<dyn State>) {
        *self.state.borrow_mut() = state;
    }

    pub fn insert_coin(&self) {
        let new_state = self.state.borrow().insert_coin(self);
        if let Some(state) = new_state {
            *self.state.borrow_mut() = state;
        }
    }

    pub fn eject_coin(&self) {
        let new_state = self.state.borrow().eject_coin(self);
        if let Some(state) = new_state {
            *self.state.borrow_mut() = state;
        }
    }

    pub fn dispense(&self) {
        let new_state = self.state.borrow().dispense(self);
        if let Some(state) = new_state {
            *self.state.borrow_mut() = state;
        }
        if *self.count.borrow() > 0 {
            *self.count.borrow_mut() -= 1;
        }
    }

    pub fn get_count(&self) -> i32 {
        *self.count.borrow()
    }
}

/// Состояние: нет монеты.
pub struct NoCoinState;

impl State for NoCoinState {
    fn insert_coin(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Монета вставлена");
        Some(Rc::new(HasCoinState))
    }

    fn eject_coin(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Нет монеты для возврата");
        None
    }

    fn dispense(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Сначала вставьте монету");
        None
    }
}

/// Состояние: есть монета.
pub struct HasCoinState;

impl State for HasCoinState {
    fn insert_coin(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Монета уже вставлена");
        None
    }

    fn eject_coin(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Монета возвращена");
        Some(Rc::new(NoCoinState))
    }

    fn dispense(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Напиток выдан");
        Some(Rc::new(SoldState))
    }
}

/// Состояние: напиток продан.
pub struct SoldState;

impl State for SoldState {
    fn insert_coin(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Подождите, напиток выдается");
        None
    }

    fn eject_coin(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Слишком поздно, напиток уже выдается");
        None
    }

    fn dispense(&self, machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Напиток выдан");
        if machine.get_count() > 0 {
            Some(Rc::new(NoCoinState))
        } else {
            Some(Rc::new(SoldOutState))
        }
    }
}

/// Состояние: напитки закончились.
pub struct SoldOutState;

impl State for SoldOutState {
    fn insert_coin(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Напитки закончились");
        None
    }

    fn eject_coin(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Напитки закончились, монета возвращена");
        None
    }

    fn dispense(&self, _machine: &VendingMachine) -> Option<Rc<dyn State>> {
        println!("Напитки закончились");
        None
    }
}

/// Тест для паттерна State.
#[test]
fn test_state() {
    let machine = VendingMachine::new(2);

    machine.insert_coin();
    machine.dispense();

    machine.insert_coin();
    machine.eject_coin();

    machine.insert_coin();
    machine.dispense();

    machine.insert_coin(); // Попытка вставить, когда напитки закончились
}