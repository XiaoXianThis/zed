use gpui::{Action, App, SharedString, Window, prelude::*};
use ui::{Tooltip, prelude::*};

pub const LEFT_TOP_PANELS: [&str; 4] =
    ["Project Panel", "GitPanel", "CollabPanel", "Outline Panel"];
pub const LEFT_BOTTOM_PANELS: [&str; 2] = ["TerminalPanel", "DebugPanel"];
pub const RIGHT_TOP_PANELS: [&str; 1] = ["AgentPanel"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StripSide {
    Left,
    Right,
}

pub fn panel_names(side: StripSide) -> (&'static [&'static str], &'static [&'static str]) {
    match side {
        StripSide::Left => (&LEFT_TOP_PANELS, &LEFT_BOTTOM_PANELS),
        StripSide::Right => (&RIGHT_TOP_PANELS, &[]),
    }
}

pub struct ActivityItem {
    pub id: SharedString,
    pub icon: IconName,
    pub tooltip: SharedString,
    pub selected: bool,
    pub action: Box<dyn Action>,
}

#[derive(IntoElement)]
pub struct ActivityStrip {
    side: StripSide,
    top: Vec<ActivityItem>,
    bottom: Vec<ActivityItem>,
}

impl ActivityStrip {
    pub fn new(side: StripSide, top: Vec<ActivityItem>, bottom: Vec<ActivityItem>) -> Self {
        Self { side, top, bottom }
    }
}

impl RenderOnce for ActivityStrip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .h_full()
            .w(px(40.))
            .flex_none()
            .py_1()
            .justify_between()
            .items_center()
            .bg(cx.theme().colors().panel_background)
            .border_color(cx.theme().colors().border)
            .map(|this| match self.side {
                StripSide::Left => this.border_r_1(),
                StripSide::Right => this.border_l_1(),
            })
            .child(
                v_flex()
                    .gap_1()
                    .items_center()
                    .children(self.top.into_iter().map(render_item)),
            )
            .child(
                v_flex()
                    .gap_1()
                    .items_center()
                    .children(self.bottom.into_iter().map(render_item)),
            )
    }
}

fn render_item(item: ActivityItem) -> impl IntoElement {
    let action = item.action;
    IconButton::new(item.id, item.icon)
        .icon_size(IconSize::Small)
        .toggle_state(item.selected)
        .tooltip(Tooltip::text(item.tooltip))
        .on_click(move |_, window, cx| window.dispatch_action(action.boxed_clone(), cx))
}
