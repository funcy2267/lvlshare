#![windows_subsystem = "windows"]

//mod backups;
mod levelstring;

extern crate sciter;
use sciter::{
    dispatch_script_call,
    types::{RECT, SCITER_CREATE_WINDOW_FLAGS::*},
    Value, Window,
};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
struct Handler {
    host: Weak<sciter::Host>,
}

const LOCAL_DATA_FOLDER_NAME: &str = "LVLShare";

impl Handler {
    fn export_level(&self, gd_data: String, level_name: String, mut location: String) {
        use std::io::Write;
        location.replace_range(..7, "");
        location = location.replace("%20", " ");
        println!("{}", location);
        let path = PathBuf::from(location);

        let mut file = File::create(path).expect("Error creating file.");
        match levelstring::export_level(gd_data, &level_name) {
            Ok(level) => {
                file.write_all(&level).expect("Error writing to file.");
            }
            Err(err) => message_box(format!("Error when exporting level: {}", err), &self.host),
        };
    }

    fn import_file(&self, gd_data: String, mut level_file: String) {
        level_file.replace_range(..7, "");
        level_file = level_file.replace("%20", " ");
        println!("{}", level_file);
        let path = PathBuf::from(level_file);
        let root = &self.host;
        match levelstring::import_level(gd_data, path) {
            Some(err) => message_box(format!("Error when importing level: {}", err), root),
            None => message_box("Level imported to Geometry Dash!".to_string(), root),
        };
    }

    fn gd_found(&self) -> bool {
        let mut gd_found = false;
        process_list::for_each_process(|_, name: &Path| {
            if name.to_str().unwrap().replace('\0', "") == "GeometryDash.exe" {
                gd_found = true;
            }
        })
        .unwrap();
        gd_found
    }

    fn get_level_names(&self, gd_data: String) -> Value {
        match levelstring::get_level_names(gd_data) {
            Ok(list) => {
                let mut array = Value::array(0);
                for name in list {
                    array.push(name);
                }
                array
            }
            Err(err) => {
                message_box(err, &self.host);
                Value::array(0)
            }
        }
    }

    fn get_user_stats(&self, gd_data: String) -> Value {
        match levelstring::get_user_stats(gd_data) {
            Ok(mut list) => {
                let name = list.remove("name").unwrap();
                let user_id = list.remove("user_id").unwrap();
                let mut stats_array = Value::array(0);
                for stat in list.iter() {
                    let mut stat_object = Value::map();
                    stat_object.set_item("name", (*stat.0).clone());
                    stat_object.set_item("value", (*stat.1).clone());
                    stats_array.push(stat_object);
                }

                let mut ret = Value::map();
                ret.set_item("username", name);
                ret.set_item("user_id", user_id);
                ret.set_item("stats", stats_array);

                ret
            }
            Err(err) => {
                message_box(err, &self.host);
                Value::map()
            }
        }
    }

    fn get_default_gd_path(&self) -> Result<String, String> {
        Ok(std::env::var("localappdata").unwrap()+"/GeometryDash")
    }

    fn format_selected_path(&self, mut path_str: String) -> Result<String, String> {
        path_str.replace_range(..7, "");
        let path_str = path_str.replace("%20", " ");
        Ok(path_str)
    }
}

impl sciter::EventHandler for Handler {
    dispatch_script_call! {
      fn export_level(String, String, String);
      fn import_file(String, String);
      fn gd_found();
      fn get_level_names(String);
      fn get_user_stats(String);
      fn get_default_gd_path();
      fn format_selected_path(String);
    }
}

fn message_box(msg: String, host: &Weak<sciter::Host>) {
    if let Some(host) = host.upgrade() {
        if host
            .eval_script(&format!("view.msgbox(\"{}\");", msg))
            .is_ok()
        {};
    }
}

fn main() {
    // Step 1: Include the 'minimal.html' file as a byte array.
    // Hint: Take a look into 'minimal.html' which contains some tiscript code.

    let html = include_bytes!("gui.htm");

    // Step 2: Enable the features we need in our tiscript code.
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8		// Enables `Sciter.machineName()`
		| sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8, // Enables opening file dialog (`view.selectFile()`)
    ))
    .unwrap();

    // Enable debug mode for all windows, so that we can inspect them via Inspector.
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();

    let mut frame = Window::create(
        RECT {
            left: 0,
            top: 0,
            right: 600,
            bottom: 450,
        },
        SW_MAIN | SW_CONTROLS, // | SW_RESIZEABLE,
        None,
    );

    // create local save folder

    let localdata = PathBuf::from(match std::env::var("localappdata") {
        Ok(path) => path,
        Err(e) => panic!("Error when loading localappdata: {}", e),
    })
    .join(LOCAL_DATA_FOLDER_NAME);

    if !localdata.exists() {
        println!("first time opening app");
        std::fs::create_dir_all(localdata).expect("Problem when creating local data directory.");
    }

    let handler = Handler {
        host: Rc::downgrade(&frame.get_host()),
    };
    frame.event_handler(handler);

    frame.load_html(html, None);

    let host = frame.get_host();
    /*let css = [include_str!("main.css")];
    let mut combined_css = String::new();
    for file in css.iter() {
        combined_css += file;
    }*/

    host.eval_script(include_str!("gui.tis"))
        .expect("Error when evaluating script");

    /*host.set_master_css(&combined_css, false)
    .expect("problem setting default css");*/

    frame.run_app();
}
