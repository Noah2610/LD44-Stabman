use climer::Timer;

#[derive(Default)]
pub struct ToMainMenu(pub bool);

#[derive(Default)]
pub struct UpdateHealthDisplay(pub bool);

#[derive(Default)]
pub struct Timers {
    pub level:  Timer,
    pub global: Option<Timer>,
}

#[derive(Default)]
pub struct CurrentLevelName(pub Option<String>);
