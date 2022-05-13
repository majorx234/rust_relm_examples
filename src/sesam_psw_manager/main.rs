use gtk::prelude::{
    BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, GtkWindowExt, OrientableExt, WidgetExt,
};
use relm4::{gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

struct AppModel {
    psw: String,
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
                self.psw = String::from("test");
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
        let domain_label = gtk::Label::new(Some("Domain:"));
        let domain_input = gtk::Entry::new();
        let user_label = gtk::Label::new(Some("User:"));
        let user_input = gtk::Entry::new();
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
    let model = AppModel {
        psw: String::from("tester"),
    };
    let app = RelmApp::new(model);
    app.run();
}
