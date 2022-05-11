use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};
use std::cmp;

#[derive(Default)]
struct AppModel {
    greatest_devisor: u32,
}

fn greatest_common_devisor(a_in: u32, b_in: u32) -> u32 {
    let mut a = cmp::max(a_in, b_in);
    let mut b = cmp::min(a_in, b_in);

    while b != 0 {
        let t = b;
        b = a.rem_euclid(b);
        a = t;
    }
    return a;
}

enum AppMsg {
    Calculate,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Calculate => {
                self.greatest_devisor = greatest_common_devisor(32, 48);
            }
        }
        true
    }
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("greatest common devisor"),
            set_default_width: 300,
            set_default_height: 100,
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,
                append = &gtk::Entry {
                                     },
                append = &gtk::Entry {
                                },
                append = &gtk::Button {
                    set_label: "calculate",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Calculate);
                    },
                },
                append = &gtk::Label {
                    set_margin_all: 5,
                    set_label: watch! { &format!("Greatest Common Devisor: {}", model.greatest_devisor) },
                }
            },
        }
    }
}

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();
}
