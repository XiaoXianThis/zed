use editor::Editor;
use gpui::{App, Entity, Subscription};
use language::language_settings::LanguageSettings;
use ui::{Label, prelude::*};
use workspace::{HideStatusItem, StatusItemView, item::ItemHandle};

#[derive(Default)]
pub struct IndentIndicator {
    indentation: Option<SharedString>,
    _observe_active_editor: Option<Subscription>,
}

impl IndentIndicator {
    fn update(&mut self, editor: Entity<Editor>, _: &mut Window, cx: &mut Context<Self>) {
        self.indentation = editor.read(cx).active_buffer(cx).map(|buffer| {
            let settings = LanguageSettings::for_buffer(buffer.read(cx), cx);
            let kind = if settings.hard_tabs { "Tabs" } else { "Spaces" };
            format!("{kind}: {}", settings.tab_size).into()
        });
        cx.notify();
    }
}

impl Render for IndentIndicator {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().when_some(self.indentation.clone(), |el, indentation| {
            el.child(
                Label::new(indentation)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
        })
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
