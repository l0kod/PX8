#[cfg(feature = "cpython")]
pub mod plugin {
    use cpython::*;

    use std::sync::{Arc, Mutex};
    use std::fs::File;
    use std::io::Read;

    use gfx;
    use gfx::Sprite;
    use config::Players;
    use px8::info::Info;
    use px8;
    use gfx::Screen;

    py_class!(class PX8Instance |py| {
    data member: i32;
    data screen: Arc<Mutex<Screen>>;
    data players: Arc<Mutex<Players>>;
    data sprites: Vec<Sprite>;
    data info: Arc<Mutex<Info>>;

    // Audio

    // Cart Data

    // Graphics

    def camera(&self, x: i32, y: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().camera(x, y);
        Ok(0)
    }

    def circ(&self, x: i32, y: i32, r: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().circ(x as u32, y as u32, r as u32, px8::Color::from_u8(color as u8));
        Ok(0)
    }

    def circfill(&self, x: i32, y: i32, r: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().circfill(x as u32, y as u32, r as u32, px8::Color::from_u8(color as u8));
        Ok(0)
    }

    def cls(&self) -> PyResult<i32> {
        self.screen(py).lock().unwrap().cls();
        Ok(0)
    }

    def color(&self, color:u8) -> PyResult<i32> {
        self.screen(py).lock().unwrap().color(px8::Color::from_u8(color as u8));
        Ok(0)
    }

    def flip(&self) -> PyResult<i32> {
        Ok(0)
    }

    def line(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().line(x1 as u32, y1 as u32, x2 as u32, y2 as u32, px8::Color::from_u8(color as u8));
        Ok(0)
    }

    def palt(&self, c: i32, t: bool) -> PyResult<i32> {
        self.screen(py).lock().unwrap().palt(c as u32, t);
        Ok(0)
    }

    def pal(&self, c0: i32, c1: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().pal(c0, c1);
        Ok(0)
    }

    def pset(&self, x: i32, y: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().pset(x as u32, y as u32, px8::Color::from_u8(color as u8));
        Ok(0)
    }

    def print(&self, str: String, x: i32, y: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().print(str, x as i32, y as i32, px8::Color::from_u8(color as u8));
        Ok(0)
    }

    def pget(&self, x: i32, y: i32) -> PyResult<u8> {
        let value = self.screen(py).lock().unwrap().pget(x as u32, y as u32);
        Ok(value)
    }

    def rect(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().rect(x1 as u32, y1 as u32, x2 as u32, y2 as u32, px8::Color::from_u8(color as u8));
        Ok(0)
    }

    def rectfill(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().rectfill(x1 as u32, y1 as u32, x2 as u32, y2 as u32, px8::Color::from_u8(color as u8));
        Ok(0)
    }

    def sget(&self, x: i32, y: i32) -> PyResult<u8> {
        Ok(self.screen(py).lock().unwrap().sget(x as u32, y as u32))
    }

    def spr(&self, n: i32, x: i32, y: i32, w: i32, h: i32, flip_x: bool, flip_y: bool) -> PyResult<i32> {
        self.screen(py).lock().unwrap().spr(n as u32,
                                                     x as u32,
                                                     y as u32,
                                                     w as u32,
                                                     h as u32,
                                                     flip_x,
                                                     flip_y);

        Ok(0)
    }

    def sset(&self, x: i32, y: i32, color: i32) -> PyResult<u8> {
        self.screen(py).lock().unwrap().sset(x as u32, y as u32, px8::Color::from_u8(color as u8));
        Ok(0)
    }

    def sspr(&self, sx: i32, sy: i32, sw: i32, sh: i32, dx: i32, dy: i32, dw: i32, dh: i32, flip_x: bool, flip_y: bool) -> PyResult<i32> {
        self.screen(py).lock().unwrap().sspr(sx as u32,
                                             sy as u32,
                                             sw as u32,
                                             sh as u32,
                                             dx as u32,
                                             dy as u32,
                                             dw as u32,
                                             dh as u32,
                                             flip_x,
                                             flip_y);
        Ok(0)
    }


    def trigon(&self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().trigon(x1 as u32, y1 as u32, x2 as u32, y2 as u32, x3 as u32, y3 as u32, px8::Color::from_u8(color as u8));
        Ok(0)
    }

    // Input

    def btn(&self, x: i32, p: i32) -> PyResult<u8> {
        let value = self.players(py).lock().unwrap().get_value(p as u8, x as u8);
        Ok(value)
    }

    def btnp(&self, x: i32, p: i32) -> PyResult<u8> {
        let value = self.players(py).lock().unwrap().get_value_quick(p as u8, x as u8);
        Ok(value)
    }

    def btn_mouse(&self, x: i32) -> PyResult<i32> {
        let value = self.players(py).lock().unwrap().get_mouse(x as u8);
        Ok(value)
    }

    def btn_mouse_state(&self) -> PyResult<u32> {
        let value = self.players(py).lock().unwrap().get_mouse_state();
        Ok(value)
    }

    // Map

    // Math

    // Memory

    def spr_map(&self, cel_x: i32, cel_y: i32, sx: i32, sy: i32, cel_w: i32, cel_h: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().map(cel_x as u32, cel_y as u32,
                                            sx as u32, sy as u32,
                                            cel_w as u32, cel_h as u32);

        Ok(0)
    }

    // Peek/Poke

    // Others
    def time(&self) -> PyResult<f64> {
        Ok(self.info(py).lock().unwrap().elapsed_time)
    }

    });

    pub struct PythonPlugin {
        pub mydict: PyDict,
        pub loaded_code: bool,
    }

    impl PythonPlugin {
        pub fn new() -> PythonPlugin {
            let gil = Python::acquire_gil();
            let py = gil.python();

            let d = PyDict::new(py);

            PythonPlugin{ mydict: d, loaded_code: false }
        }


        pub fn load(&mut self,
                    players: Arc<Mutex<Players>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    sprites: Vec<Sprite>,
                    map: [[u32; 32]; gfx::SCREEN_WIDTH]) {
            info!("INIT PYTHON plugin");

            let gil = Python::acquire_gil();
            let py = gil.python();

            let obj = PX8Instance::create_instance(py,
                                                   7,
                                                   screen.clone(),
                                                   players.clone(),
                                                   sprites.clone(),
                                                   info.clone()).unwrap();
            self.mydict.set_item(py, "obj", obj).unwrap();

            py.run(r###"globals()["global_obj"] = obj;"###, None, Some(&self.mydict));

            let mut f = File::open("./sys/config/api.py").unwrap();
            let mut data = String::new();
            f.read_to_string(&mut data).unwrap();

            let result = py.run(&data, None, None);
            match result {
                Err(v) => {
                    panic!("FAILED TO LOAD PYTHON API = {:?}", v);
                }
                Ok(v) => {
                    info!("SUCCESSFULLY LOAD PYTHON API = {:?}", v);
                }
            }
        }


        pub fn init(&mut self) {
            info!("CALL INIT");

            if self.loaded_code == false {
                return;
            }

            let gil = Python::acquire_gil();
            let py = gil.python();

            let result = py.run(r###"_init()"###, None, Some(&self.mydict));
            info!("RES INIT = {:?}", result);
        }

        pub fn draw(&mut self) -> bool {
            let mut return_draw_value = true;
            debug!("CALL DRAW");

            if self.loaded_code == false {
                return false;
            }

            let gil = Python::acquire_gil();
            let py = gil.python();

            let result = py.eval(r###"_draw()"###, None, Some(&self.mydict));

            match result {
                Err(v) => {
                    return_draw_value = false;
                    warn!("DRAW = {:?}", v);
                },
                Ok(v) => {
                    match v.extract(py) {
                        Ok(draw_value) => {
                            return_draw_value = draw_value;
                        }
                        _ => (),
                    }
                },
            }

            return return_draw_value;
        }

        pub fn update(&mut self) -> bool {
            let mut return_update_value = true;
            debug!("CALL UPDATE");

            if self.loaded_code == false {
                return false;
            }

            let gil = Python::acquire_gil();
            let py = gil.python();

            let result = py.eval(r###"_update()"###, None, Some(&self.mydict));

            match result {
                Err(v) => {
                    return_update_value = false;
                    warn!("UPDATE = {:?}", v);
                },
                Ok(v) => {
                    match v.extract(py) {
                        Ok(update_value) => {
                            return_update_value = update_value;
                        }
                        _ => (),
                    }
                },
            }

            return return_update_value;
        }


        pub fn load_code(&mut self, data: String) {
            info!("LOAD CODE");
            let gil = Python::acquire_gil();
            let py = gil.python();


            let result = py.run(&data, None, None);
            debug!("RES CODE = {:?}", result);

            match result {
                Ok(_) => self.loaded_code = true,
                _ => self.loaded_code = false,
            }
        }
    }
}

#[cfg(not(feature = "cpython"))]
pub mod plugin {
    use std::sync::{Arc, Mutex};

    use gfx::Sprite;
    use config::Players;

    use px8;
    use px8::info::Info;

    use gfx;
    use gfx::{SCREEN_WIDTH, SCREEN_HEIGHT};
    use gfx::Screen;

    pub struct PythonPlugin {}

    impl PythonPlugin {
        pub fn new() -> PythonPlugin {
            PythonPlugin {}
        }


        pub fn load(&mut self,
                    players: Arc<Mutex<Players>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    sprites: Vec<Sprite>,
                    map: [[u32; 32]; gfx::SCREEN_WIDTH]) {
            panic!("PYTHON plugin disabled");
        }
        pub fn init(&mut self) {}
        pub fn draw(&mut self) -> bool { return false; }
        pub fn update(&mut self) -> bool { return false; }
        pub fn load_code(&mut self, data: String) {}
    }
}