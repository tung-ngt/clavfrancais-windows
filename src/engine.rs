use clavfrancais_engine::{
    char_buffer::CharBuffer,
    engine::{CombinationTarget, Engine, KeyCombinationMap},
    keys::{Key, CHANGE_FOCUS_KEYS},
};
use std::sync::mpsc::{self, Receiver, Sender};

use crate::input_listener::{InputListener, MouseKeyEvent, WindowsListener as InputListenerImpl};
use crate::input_simulator::{InputSimulator, WindowsInputSimulator as InputSimulatorImpl};

pub struct WindowEngine;

struct WindowEngineState<T>
where
    T: CharBuffer,
{
    input_controller: Engine<T>,
    open_guillmets: bool,
}

impl<T> WindowEngineState<T>
where
    T: CharBuffer,
{
    fn new(combination_map: KeyCombinationMap, char_buffer: T) -> Self {
        Self {
            input_controller: Engine::new(combination_map, char_buffer),
            open_guillmets: true,
        }
    }
    fn handle_event(&mut self, receiver: Receiver<MouseKeyEvent>) {
        loop {
            let Ok(event) = receiver.recv() else {
                return;
            };

            match event {
                MouseKeyEvent::Mouse => {
                    self.input_controller.clear_char_buffer();
                    continue;
                }
                MouseKeyEvent::Key { unicode_char, key } => {
                    if CHANGE_FOCUS_KEYS.contains(&key) {
                        self.input_controller.clear_char_buffer();
                        continue;
                    }

                    let Some(unicode_char) = unicode_char else {
                        continue;
                    };

                    if unicode_char == '"' {
                        let guillements = if self.open_guillmets { '«' } else { '»' };
                        self.open_guillmets = !self.open_guillmets;

                        let _ = self.input_controller.add_char(guillements);

                        InputSimulatorImpl::backspace();
                        InputSimulatorImpl::character(guillements);
                        continue;
                    }

                    if key == Key::Backspace {
                        self.input_controller.backspace();
                        continue;
                    }

                    let target = self.input_controller.add_char(unicode_char);

                    let Some(target) = target else {
                        continue;
                    };
                    InputSimulatorImpl::backspace();
                    InputSimulatorImpl::backspace();

                    if let CombinationTarget::Combine(a) = target {
                        InputSimulatorImpl::character(a);
                        continue;
                    }

                    if let CombinationTarget::Revert(a, b) = target {
                        InputSimulatorImpl::character(a);
                        InputSimulatorImpl::character(b);
                    }
                }
            }
        }
    }
}

impl WindowEngine {
    pub fn start(combination_map: KeyCombinationMap, char_buffer: impl CharBuffer) {
        let (sender, receiver) = mpsc::channel::<MouseKeyEvent>();
        InputListenerImpl::start_mouse_key_listening(sender);
        let mut engine = WindowEngineState::new(combination_map, char_buffer);
        engine.handle_event(receiver);
    }

    pub fn stop() {
        InputListenerImpl::stop_mouse_key_listening();
    }

    pub fn set_toggle_channel(sender: Sender<()>) {
        InputListenerImpl::start_shortcut_listening(sender);
    }
}
