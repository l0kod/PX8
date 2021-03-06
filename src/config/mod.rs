use frontend::PX8Key;
use sdl2::mouse::MouseState;
use std::collections::HashMap;

pub struct Mouse {
    pub x: i32,
    pub y: i32,
    pub state: u32,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse{x: 0, y: 0, state: 0}
    }

}
pub struct PlayerKeys {
    frames: HashMap<PX8Key, f64>,

    enter: bool,
    enter_quick: bool,
    pause: bool,
    pause_quick: bool,

    right: bool,
    right_quick: bool,

    left: bool,
    left_quick: bool,

    up: bool,
    up_quick: bool,

    down: bool,
    down_quick: bool,

    o: bool,
    o_quick: bool,

    x: bool,
    x_quick: bool,
}

impl PlayerKeys {
    pub fn new() -> PlayerKeys {
        PlayerKeys{
            frames: HashMap::new(),
            enter: false,
            enter_quick: false,
            pause: false,
            pause_quick: false,

            right: false,
            right_quick: false,
            left: false,
            left_quick: false,
            up: false,
            up_quick: false,
            down: false,
            down_quick: false,
            o: false,
            o_quick: false,
            x: false,
            x_quick: false,
        }
    }
}

pub struct Players {
    pub keys: HashMap<u8, PlayerKeys>,
    pub mouse: Mouse,

}

impl Players {
    pub fn new() -> Players {
        let mut keys = HashMap::new();
        keys.insert(0, PlayerKeys::new());
        keys.insert(1, PlayerKeys::new());

        Players{ keys: keys, mouse: Mouse::new() }
    }

    pub fn set_mouse_x(&mut self, x: i32) {
        self.mouse.x = x;
    }

    pub fn set_mouse_y(&mut self, y: i32) {
        self.mouse.y = y;
    }

    pub fn set_mouse_state(&mut self, mouse_state: MouseState) {
        if mouse_state.left() {
            self.mouse.state = 1;
        }

        else if mouse_state.right() {
            self.mouse.state = 2;
        }

        else if mouse_state.middle() {
            self.mouse.state = 4;
        }
        else {
            self.mouse.state = 0;
        }
    }

    pub fn update(&mut self, elapsed: f64) {
        for (_, keys) in self.keys.iter_mut() {
            if keys.left == true {
                match keys.frames.get(&PX8Key::Left) {
                    Some(&value) => {
                        if elapsed - value >= 0.01 {
                            keys.left_quick = false;
                        } else {
                            keys.left_quick = true;
                        }
                    },
                    _ => { keys.left_quick = true; }
                }
            }

            if keys.right == true {
                match keys.frames.get(&PX8Key::Right) {
                    Some(&value) => {
                        if elapsed - value >= 0.01 {
                            keys.right_quick = false;
                        } else {
                            keys.right_quick = true;
                        }
                    },
                    _ => { keys.right_quick = true; }
                }
            }

            if keys.up == true {
                match keys.frames.get(&PX8Key::Up) {
                    Some(&value) => {
                        if elapsed - value >= 0.01 {
                            keys.up_quick = false;
                        } else {
                            keys.up_quick = true;
                        }
                    },
                    _ => { keys.up_quick = true; }
                }
            }

            if keys.down == true {
                match keys.frames.get(&PX8Key::Down) {
                    Some(&value) => {
                        if elapsed - value >= 0.01 {
                            keys.down_quick = false;
                        } else {
                            keys.down_quick = true;
                        }
                    },
                    _ => { keys.down_quick = true; }
                }
            }


            if keys.o == true {
                match keys.frames.get(&PX8Key::O) {
                    Some(&value) => {
                        if elapsed - value >= 0.01 {
                            keys.o_quick = false;
                        } else {
                            keys.o_quick = true;
                        }
                    },
                    _ => { keys.o_quick = true; }
                }
            }

            if keys.x == true {
                match keys.frames.get(&PX8Key::X) {
                    Some(&value) => {
                        if elapsed - value >= 0.01 {
                            keys.x_quick = false;
                        } else {
                            keys.x_quick = true;
                        }
                    },
                    _ => { keys.x_quick = true; }
                }
            }

            if keys.pause == true {
                match keys.frames.get(&PX8Key::Pause) {
                    Some(&value) => {
                        if elapsed - value >= 0.01 {
                            keys.pause_quick = false;
                        } else {
                            keys.pause_quick = true;
                        }
                    },
                    _ => { keys.pause_quick = true; }
                }
            }

            if keys.enter == true {
                match keys.frames.get(&PX8Key::Enter) {
                    Some(&value) => {
                        if elapsed - value >= 0.01 {
                            keys.enter_quick = false;
                        } else {
                            keys.enter_quick = true;
                        }
                    },
                    _ => { keys.enter_quick = true; }
                }
            }
        }
    }

    pub fn key_down(&mut self, player: u8, key: PX8Key, repeat: bool, elapsed: f64) {
        debug!("KEY {:?} {:?} {:?} Player {:?} -> DOWN", key, repeat, elapsed, player);

        match self.keys.get_mut(&player) {
            Some(keys) => {
                match key {
                    PX8Key::Right => {
                        if keys.right == false {
                            keys.right_quick = true;
                        }
                        keys.right = true;
                    },
                    PX8Key::Left => {
                        if keys.left == false {
                            keys.left_quick = true;
                        }
                        keys.left = true;
                    },
                    PX8Key::Up => {
                        if keys.up == false {
                            keys.up_quick = true;
                        }
                        keys.up = true
                    },
                    PX8Key::Down => {
                        if keys.down == false {
                            keys.down_quick = true;
                        }
                        keys.down = true;
                    },
                    PX8Key::O => {
                        if keys.o == false {
                            keys.o_quick = true;
                        }
                        keys.o = true;
                    },
                    PX8Key::X => {
                        if keys.x == false {
                            keys.x_quick = true;
                        }

                        keys.x = true;
                    },
                    PX8Key::Enter => {
                        if keys.enter == false {
                            keys.enter_quick = true;
                        }

                        keys.enter = true
                    },
                    PX8Key::Pause => {
                        if keys.pause == false {
                            keys.pause_quick = true;
                        }

                        keys.pause = true;
                    }
                }

                if repeat == false {
                    keys.frames.insert(key, elapsed);
                }
            },
            None => ()
        }
    }

    pub fn key_direc_hor_up(&mut self, player: u8) {
        match self.keys.get_mut(&player) {
            Some(keys) => {
                keys.right = false;
                keys.left = false;
            },
            None => ()
        }
    }

    pub fn key_direc_ver_up(&mut self, player: u8) {
        match self.keys.get_mut(&player) {
            Some(keys) => {
                keys.up = false;
                keys.down = false;
            },
            None => ()
        }
    }

    pub fn key_up(&mut self, player: u8, key: PX8Key) {
        debug!("KEY {:?} Player {:?} -> UP", key, player);

        match self.keys.get_mut(&player) {
            Some(keys) => {
                match key {
                    PX8Key::Right => {
                        keys.right = false;
                        keys.right_quick = false;
                    },
                    PX8Key::Left => {
                        keys.left = false;
                        keys.left_quick = false;
                    },
                    PX8Key::Up => {
                        keys.up = false;
                        keys.up_quick = false;
                    },
                    PX8Key::Down => {
                        keys.down = false;
                        keys.down_quick = false;
                    },
                    PX8Key::O => {
                        keys.o = false;
                        keys.o_quick = false;
                    },
                    PX8Key::X => {
                        keys.x = false;
                        keys.x_quick = false;
                    },
                    PX8Key::Enter => {
                        keys.enter = false;
                        keys.enter_quick = false;
                    },
                    PX8Key::Pause => {
                        keys.pause = false;
                        keys.pause_quick = false;
                    }
                }
            },
            None => ()
        }
    }

    pub fn get_value(&mut self, player: u8, index: u8) -> u8 {
        let mut value = 0;

        match self.keys.get_mut(&player) {
            Some(keys) => {
                if index == 0 {
                    if keys.left {
                        value = 1;
                    }
                } else if index == 1 {
                    if keys.right {
                        value = 1;
                    }
                } else if index == 2 {
                    if keys.up {
                        value = 1;
                    }
                } else if index == 3 {
                    if keys.down {
                        value = 1;
                    }
                } else if index == 4 {
                    if keys.o {
                        value = 1;
                    }
                } else if index == 5 {
                    if keys.x {
                        value = 1;
                    }
                } else if index == 6 {
                    if keys.enter {
                        value = 1;
                    }
                } else if index == 7 {
                    if keys.pause {
                        value = 1;
                    }
                }
            },
            None => ()
        }
        value
    }


    pub fn get_value_quick(&mut self, player: u8, index: u8) -> u8 {
        let mut value = 0;

        match self.keys.get_mut(&player) {
            Some(keys) => {
                if index == 0 {
                    if keys.left_quick {
                        value = 1;
                    }
                } else if index == 1 {
                    if keys.right_quick {
                        value = 1;
                    }
                } else if index == 2 {
                    if keys.up_quick {
                        value = 1;
                    }
                } else if index == 3 {
                    if keys.down_quick {
                        value = 1;
                    }
                } else if index == 4 {
                    if keys.o_quick {
                        value = 1;
                    }
                } else if index == 5 {
                    if keys.x_quick {
                        value = 1;
                    }
                } else if index == 6 {
                    if keys.enter_quick {
                        value = 1;
                    }
                } else if index == 7 {
                    if keys.pause_quick {
                        value = 1;
                    }
                }
            },
            None => ()
        }

        return value;
    }

    pub fn get_mouse(&mut self, index: u8) -> i32 {
        match index {
            0 => self.mouse.x,
            1 => self.mouse.y,
            _ => 0,
        }
    }

    pub fn get_mouse_state(&mut self) -> u32 {
        self.mouse.state
    }

}