use editor::Editor;
use file_icons::FileIcons;
use gpui::{App, Empty, Entity, Subscription};
use language::language_settings::{LanguageSettings, all_language_settings};
use project::Project;
use settings::Settings;
use std::path::Path;
use ui::{Label, Tooltip, prelude::*};
use workspace::{HideStatusItem, StatusBarSettings, StatusItemView, Workspace, item::ItemHandle};

#[derive(Default)]
pub struct IndentIndicator {
    indentation: Option<SharedString>,
    _observe_active_editor: Option<Subscription>,
}

impl IndentIndicator {
    fn update(&mut self, editor: Entity<Editor>, _: &mut Window, cx: &mut Context<Self>) {
        self.indentation = editor.read(cx).active_buffer(cx).map(|buffer| {
            indentation_label(&LanguageSettings::for_buffer(buffer.read(cx), cx))
        });
        cx.notify();
    }
}

fn indentation_label(settings: &LanguageSettings) -> SharedString {
    let kind = if settings.hard_tabs { "Tabs" } else { "Spaces" };
    format!("{kind}: {}", settings.tab_size).into()
}

impl Render for IndentIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let indentation = self
            .indentation
            .clone()
            .unwrap_or_else(|| indentation_label(&all_language_settings(None, cx).defaults));
        div().child(
            Label::new(indentation)
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
    }
}

impl StatusItemView for IndentIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe_in(&editor, window, Self::update));
            self.update(editor, window, cx);
        } else {
            self.indentation = None;
            self._observe_active_editor = None;
        }
        cx.notify();
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        None
    }
}

pub struct ActiveFilePath {
    project: Entity<Project>,
    segments: Vec<SharedString>,
    file_icon: Option<SharedString>,
    full_path: Option<SharedString>,
}

impl ActiveFilePath {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            project: workspace.project().clone(),
            segments: Vec::new(),
            file_icon: None,
            full_path: None,
        }
    }
}

impl Render for ActiveFilePath {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !StatusBarSettings::get_global(cx).show_active_file {
            return Empty.into_any_element();
        }
        if self.segments.is_empty() {
            return Label::new("\u{2014}")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .into_any_element();
        }
        let last_index = self.segments.len() - 1;
        let mut row = h_flex().id("active-file-path").flex_none().gap_1p5();
        for (index, segment) in self.segments.iter().enumerate() {
            let is_last = index == last_index;
            if index > 0 {
                row = row.child(
                    Icon::new(IconName::ChevronRight)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                );
            }
            if is_last && let Some(file_icon) = self.file_icon.clone() {
                row = row.child(Icon::from_path(file_icon).size(IconSize::Small));
            }
            row = row.child(
                Label::new(segment.clone())
                    .size(LabelSize::Small)
                    .color(if is_last {
                        Color::Default
                    } else {
                        Color::Muted
                    }),
            );
        }
        row.when_some(self.full_path.clone(), |el, full_path| {
            el.tooltip(Tooltip::text(full_path))
        })
        .into_any_element()
    }
}

impl StatusItemView for ActiveFilePath {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let project_path = active_pane_item.and_then(|item| item.project_path(cx));
        match project_path {
            Some(project_path) => {
                let relative_path = project_path.path.as_unix_str().to_string();
                let root_name = self
                    .project
                    .read(cx)
                    .worktree_for_id(project_path.worktree_id, cx)
                    .map(|worktree| worktree.read(cx).root_name_str().to_string());
                self.segments = root_name
                    .into_iter()
                    .chain(relative_path.split('/').map(str::to_string))
                    .map(SharedString::from)
                    .collect();
                self.file_icon = FileIcons::get_icon(Path::new(&relative_path), cx);
                self.full_path = active_pane_item.and_then(|item| item.tab_tooltip_text(cx));
            }
            None => {
                self.segments.clear();
                self.file_icon = None;
                self.full_path = None;
            }
        }
        cx.notify();
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings.status_bar.get_or_insert_default().show_active_file = Some(false);
        }))
    }
}
