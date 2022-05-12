use gtk::prelude::{
    BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, GtkWindowExt, OrientableExt, WidgetExt,
};
use relm4::{gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};
use std::cmp;

#[derive(Default)]
struct AppModel {
    greatest_devisor: u32,
    entry_a: gtk::EntryBuffer,
    entry_b: gtk::EntryBuffer,
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
                let text_a = self.entry_a.text();
                let text_b = self.entry_b.text();
                if let Ok(v_a) = text_a.parse::<u32>() {
                    if let Ok(v_b) = text_b.parse::<u32>() {
                        self.greatest_devisor = greatest_common_devisor(v_a, v_b);
                    }
                }
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
                    set_buffer: &model.entry_a,
                    set_tooltip_text: Some("Input a")
                                     },
                append = &gtk::Entry {
                    set_buffer: &model.entry_b,
                    set_tooltip_text: Some("Input a")
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
