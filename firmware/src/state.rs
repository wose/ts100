#[derive(Clone, Copy)]
pub enum ConfigPage {
    Save,
}

pub enum Keys {
    A,
    B,
    AB,
    None,
}

#[derive(Clone, Copy)]
pub enum State {
    Idle,
    Soldering,
    TemperatureControl,
    Config(ConfigPage),
    Sleep,
    Cooling,
    Thermometer,
}

pub struct StateMachine {
    keys: Keys,
    state: State,
}

impl StateMachine {
    pub const fn new() -> Self {
        StateMachine {
            keys: Keys::None,
            state: State::Idle,
        }
    }

    pub fn update_keys(&mut self, keys: Keys) {
        self.keys = keys;
    }

    pub fn current_state(&self) -> State {
        self.state
    }

    pub fn update_state(&mut self) {
        use State::*;
        use Keys::*;

        self.state = match (&self.state, &self.keys) {
            (&Idle, &A) => Soldering,
            (&Idle, &B) => Thermometer,
            (&Soldering, &A) | (&Soldering, &B) => TemperatureControl,
            (&Soldering, &AB) => Idle,
            (_, &None) => self.state,
            _ => Idle,
        };

        self.keys = Keys::None;
    }
}
