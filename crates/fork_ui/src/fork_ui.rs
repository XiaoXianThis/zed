use gpui::{Action, AnyView, App, Pixels, SharedString, Window, prelude::*};
use ui::{Tooltip, prelude::*};

pub struct PanelSlot {
    pub persistent_name: &'static str,
    pub icon: IconName,
    pub tooltip: &'static str,
}

const LEFT_TOP_PANELS: [PanelSlot; 4] = [
    PanelSlot {
        persistent_name: "Project Panel",
        icon: IconName::FileTree,
        tooltip: "Project Panel",
    },
    PanelSlot {
        persistent_name: "GitPanel",
        icon: IconName::GitBranch,
        tooltip: "Git Panel",
    },
    PanelSlot {
        persistent_name: "CollabPanel",
        icon: IconName::UserGroup,
        tooltip: "Collab Panel",
    },
    PanelSlot {
        persistent_name: "Outline Panel",
        icon: IconName::ListTree,
        tooltip: "Outline Panel",
    },
];
const LEFT_BOTTOM_PANELS: [PanelSlot; 2] = [
    PanelSlot {
        persistent_name: "TerminalPanel",
        icon: IconName::TerminalAlt,
        tooltip: "Terminal Panel",
    },
    PanelSlot {
        persistent_name: "DebugPanel",
        icon: IconName::Debug,
        tooltip: "Debug Panel",
    },
];
const RIGHT_TOP_PANELS: [PanelSlot; 1] = [PanelSlot {
    persistent_name: "AgentPanel",
    icon: IconName::ZedAssistant,
    tooltip: "Agent Panel",
}];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StripSide {
    Left,
    Right,
}

pub fn panel_slots(side: StripSide) -> (&'static [PanelSlot], &'static [PanelSlot]) {
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

pub type PanelItemLookup = Box<dyn Fn(&PanelSlot, &Window, &App) -> Option<ActivityItem>>;

pub fn strip_width() -> Pixels {
    px(36.)
}

#[derive(IntoElement)]
pub struct ActivityStrip {
    side: StripSide,
    panel_item: PanelItemLookup,
    extra_top: Vec<ActivityItem>,
    footer: Vec<AnyView>,
    top_inset: Pixels,
}

impl ActivityStrip {
    pub fn new(side: StripSide, panel_item: PanelItemLookup, extra_top: Vec<ActivityItem>) -> Self {
        Self {
            side,
            panel_item,
            extra_top,
            footer: Vec::new(),
            top_inset: px(0.),
        }
    }

    pub fn with_footer(mut self, footer: Vec<AnyView>) -> Self {
        self.footer = footer;
        self
    }

    pub fn with_top_inset(mut self, top_inset: Pixels) -> Self {
        self.top_inset = top_inset;
        self
    }
}

impl RenderOnce for ActivityStrip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (top_slots, bottom_slots) = panel_slots(self.side);
        let top: Vec<_> = top_slots
            .iter()
            .filter_map(|slot| (self.panel_item)(slot, window, cx))
            .chain(self.extra_top)
            .collect();
        let bottom: Vec<_> = bottom_slots
            .iter()
            .filter_map(|slot| (self.panel_item)(slot, window, cx))
            .collect();
        v_flex()
            .h_full()
            .w(strip_width())
            .flex_none()
            .pt(self.top_inset + px(8.))
            .pb_2()
            .justify_between()
            .items_center()
            .bg(cx.theme().colors().title_bar_background)
            .child(
                v_flex()
                    .gap_3()
                    .items_center()
                    .children(top.into_iter().map(render_item)),
            )
            .child(
                v_flex()
                    .gap_3()
                    .items_center()
                    .children(bottom.into_iter().map(render_item))
                    .children(self.footer),
            )
    }
}

pub fn settings_button() -> impl IntoElement {
    IconButton::new("fork-settings", IconName::Settings)
        .icon_size(IconSize::Small)
        .icon_color(Color::Muted)
        .tooltip(Tooltip::text("Settings"))
        .on_click(|_, window, cx| {
            window.dispatch_action(zed_actions::OpenSettings.boxed_clone(), cx)
        })
}

fn render_item(item: ActivityItem) -> impl IntoElement {
    let action = item.action;
    let color = if item.selected {
        Color::Accent
    } else {
        Color::Muted
    };
    IconButton::new(item.id, item.icon)
        .icon_size(IconSize::Small)
        .icon_color(color)
        .tooltip(Tooltip::text(item.tooltip))
        .on_click(move |_, window, cx| window.dispatch_action(action.boxed_clone(), cx))
}
