// Паттерн Command: инкапсулирует запрос как объект, позволяя параметризовать клиентов
// с различными запросами, ставить запросы в очередь и логировать операции.
// Полезен для реализации отмены операций, очередей команд.
// Пример: пульт управления светом.

/// Трейт для команды.
pub trait Command {
    fn execute(&self);
}

/// Получатель - свет.
pub struct Light;

impl Light {
    pub fn on(&self) {
        println!("Свет включен");
    }

    pub fn off(&self) {
        println!("Свет выключен");
    }
}

/// Конкретная команда - включить свет.
pub struct LightOnCommand {
    light: Light,
}

impl LightOnCommand {
    pub fn new(light: Light) -> Self {
        LightOnCommand { light }
    }
}

impl Command for LightOnCommand {
    fn execute(&self) {
        self.light.on();
    }
}

/// Конкретная команда - выключить свет.
pub struct LightOffCommand {
    light: Light,
}

impl LightOffCommand {
    pub fn new(light: Light) -> Self {
        LightOffCommand { light }
    }
}

impl Command for LightOffCommand {
    fn execute(&self) {
        self.light.off();
    }
}

/// Вызывающий - пульт управления.
pub struct RemoteControl {
    command: Option<Box<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl { command: None }
    }

    pub fn set_command(&mut self, command: Box<dyn Command>) {
        self.command = Some(command);
    }

    pub fn press_button(&self) {
        if let Some(ref cmd) = self.command {
            cmd.execute();
        } else {
            println!("Команда не установлена");
        }
    }
}

/// Тест для паттерна Command.
#[test]
fn test_command() {
    let light = Light;
    let light_on = Box::new(LightOnCommand::new(light));
    let mut remote = RemoteControl::new();

    remote.set_command(light_on);
    remote.press_button();

    let light2 = Light;
    let light_off = Box::new(LightOffCommand::new(light2));
    remote.set_command(light_off);
    remote.press_button();
}