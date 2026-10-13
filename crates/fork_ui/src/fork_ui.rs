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

pub type PanelItemLookup = Box<dyn Fn(&str, &Window, &App) -> Option<ActivityItem>>;

#[derive(IntoElement)]
pub struct ActivityStrip {
    side: StripSide,
    panel_item: PanelItemLookup,
    extra_top: Vec<ActivityItem>,
}

impl ActivityStrip {
    pub fn new(side: StripSide, panel_item: PanelItemLookup, extra_top: Vec<ActivityItem>) -> Self {
        Self {
            side,
            panel_item,
            extra_top,
        }
    }
}

impl RenderOnce for ActivityStrip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (top_names, bottom_names) = panel_names(self.side);
        let top: Vec<_> = top_names
            .iter()
            .filter_map(|name| (self.panel_item)(name, window, cx))
            .chain(self.extra_top)
            .collect();
        let bottom: Vec<_> = bottom_names
            .iter()
            .filter_map(|name| (self.panel_item)(name, window, cx))
            .collect();
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
                    .children(top.into_iter().map(render_item)),
            )
            .child(
                v_flex()
                    .gap_1()
                    .items_center()
                    .children(bottom.into_iter().map(render_item)),
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
