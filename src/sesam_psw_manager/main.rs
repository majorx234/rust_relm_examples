use gtk::prelude::{
    BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, GtkWindowExt, OrientableExt, WidgetExt,
};
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use relm4::{gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};
use std::str;

#[derive(Default)]
struct AppModel {
    psw: String,
    master_buffer: gtk::EntryBuffer,
    domain_buffer: gtk::EntryBuffer,
    user_buffer: gtk::EntryBuffer,
}

enum AppMsg {
    Show,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Show => {
                let master = self.master_buffer.text();
                let domain = self.domain_buffer.text();
                let user = self.user_buffer.text();

                let salt: &str = &user[1..8];
                /*{
                        let usr_str_result = str::from_utf8(user);
                        match usr_str_result {
                            Ok(usr_str) => usr_str,
                            Err(_) => "",
                        }
                };*/
                self.psw = String::from("error234");
                let password: String = format!("{}{}", master, domain);
                let password_hash_result = Pbkdf2.hash_password(password.as_bytes(), &salt);
                match password_hash_result {
                    Ok(password_hash) => {
                        let password_hash_string = password_hash.to_string();
                        self.psw = password_hash_string;
                    }
                    Err(_) => {
                        self.psw = String::from("error");
                    }
                }
            }
        }
        true
    }
}

struct AppWidgets {
    window: gtk::ApplicationWindow,
    vbox: gtk::Box,
    master_label: gtk::Label,
    master_input: gtk::Entry,
    domain_label: gtk::Label,
    domain_input: gtk::Entry,
    user_label: gtk::Label,
    user_input: gtk::Entry,
    config_label: gtk::Label,
    config_input: gtk::Entry,
    confirm_button: gtk::Button,
    result_label: gtk::Label,
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    // init GUI
    fn init_view(model: &AppModel, _parent_widgets: &(), sender: Sender<AppMsg>) -> Self {
        let window = gtk::ApplicationWindow::builder()
            .title("SESAM Psw Manager")
            .default_width(600)
            .default_height(300)
            .build();
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();
        let master_label = gtk::Label::new(Some("MasterPsw:"));
        let master_input = gtk::Entry::new();
        master_input.set_buffer(&model.master_buffer);
        let domain_label = gtk::Label::new(Some("Domain:"));
        let domain_input = gtk::Entry::new();
        domain_input.set_buffer(&model.domain_buffer);
        let user_label = gtk::Label::new(Some("User:"));
        let user_input = gtk::Entry::new();
        user_input.set_buffer(&model.user_buffer);
        let config_label = gtk::Label::new(Some("configfile:"));
        let config_input = gtk::Entry::new();
        let confirm_button = gtk::Button::with_label("confirm");
        let result_label = gtk::Label::new(Some("enter username and domain"));
        result_label.set_margin_all(5);

        // add widgets
        window.set_child(Some(&vbox));
        vbox.append(&master_label);
        vbox.append(&master_input);
        vbox.append(&domain_label);
        vbox.append(&domain_input);
        vbox.append(&user_label);
        vbox.append(&user_input);
        vbox.append(&config_label);
        vbox.append(&config_input);
        vbox.append(&confirm_button);
        vbox.append(&result_label);

        let btn_confirm_sender = sender.clone();
        confirm_button.connect_clicked(move |_| {
            send!(btn_confirm_sender, AppMsg::Show);
        });
        Self {
            window,
            vbox,
            master_label,
            master_input,
            domain_label,
            domain_input,
            user_label,
            user_input,
            config_label,
            config_input,
            confirm_button,
            result_label,
        }
    }

    // return the root widget
    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    // update the view to represent the update model
    fn view(&mut self, model: &AppModel, _sender: Sender<AppMsg>) {
        self.result_label
            .set_label(&format!("Password: {}", model.psw));
    }
}

fn main() {
    let model = AppModel::default();
    /*AppModel {
        psw: String::from("tester"),
    };*/
    let app = RelmApp::new(model);
    app.run();
}
