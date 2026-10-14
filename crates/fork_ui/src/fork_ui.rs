use gpui::{
    Action, AnyView, App, Div, ElementId, Pixels, SharedString, Stateful, Window, prelude::*,
};
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

impl ActivityStrip {
    fn collect_items(
        self,
        window: &Window,
        cx: &App,
    ) -> (Vec<ActivityItem>, Vec<ActivityItem>, Vec<AnyView>) {
        let (top_slots, bottom_slots) = panel_slots(self.side);
        let top = top_slots
            .iter()
            .filter_map(|slot| (self.panel_item)(slot, window, cx))
            .chain(self.extra_top)
            .collect();
        let bottom = bottom_slots
            .iter()
            .filter_map(|slot| (self.panel_item)(slot, window, cx))
        (top, bottom, self.footer)
    }

    pub fn into_title_bar_items(self, window: &Window, cx: &App) -> impl IntoElement {
        let (top, bottom, footer) = self.collect_items(window, cx);
        h_flex()
            .gap_1()
            .items_center()
            .children(top.into_iter().chain(bottom).map(render_item))
            .children(footer)
    }
}

impl RenderOnce for ActivityStrip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let top_inset = self.top_inset;
        let (top, bottom, footer) = self.collect_items(window, cx);
            .collect();
        v_flex()
            .h_full()
            .w(strip_width())
            .flex_none()
            .pt(top_inset + px(8.))
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
                    .children(footer),
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

pub fn agent_tab_strip_height() -> Pixels {
    px(35.)
}

pub fn agent_body_line_height() -> Pixels {
    px(22.)
}

pub fn agent_composer_frame(cx: &App) -> Div {
    v_flex()
        .px(px(10.))
        .pt(px(8.))
        .pb(px(6.))
        .rounded(px(12.))
        .bg(cx.theme().colors().editor_background)
        .border_1()
        .border_color(cx.theme().colors().border)
}

pub fn agent_tab_strip() -> Stateful<Div> {
    h_flex()
        .id("agent-thread-tabs")
        .h_full()
        .min_w_0()
        .flex_1()
        .overflow_x_scroll()
}

pub fn agent_thread_tab(id: impl Into<ElementId>, selected: bool, cx: &App) -> Stateful<Div> {
    let colors = cx.theme().colors();
    h_flex()
        .id(id)
        .h_full()
        .min_w_0()
        .flex_shrink_0()
        .px(px(10.))
        .gap_1p5()
        .border_r_1()
        .border_color(colors.border)
        .cursor_pointer()
        .when(selected, |this| {
            this.w(px(200.)).bg(colors.tab_active_background)
        })
        .when(!selected, |this| {
            this.max_w(px(200.))
                .bg(colors.tab_inactive_background)
                .hover(|style| style.bg(colors.tab_active_background))
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
