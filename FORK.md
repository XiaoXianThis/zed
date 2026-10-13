# Fork 维护规则

本仓库是 `zed-industries/zed` 的个人 fork，目标是改造为类似 Cursor 的外观（面板位置、默认显隐、按钮位置、圆角、图标）以及部分功能。本文件定义改动放在哪里、如何同步上游、冲突听谁的。

## 一、分支与远程

- `upstream`：官方仓库 `https://github.com/zed-industries/zed.git`（需要手动添加，见下方"初始化"）。
- `origin`：个人 fork `https://github.com/XiaoXianThis/zed.git`。
- `main`：只跟随 `upstream`，不放任何 fork 提交。
- `fork/main`：fork 的集成分支。由 `main` 分叉，所有 fork 提交都在它上面。日常开发和发布都基于它。
- `fork/<功能名>`：较大的独立功能，从 `fork/main` 分出，完成后合回 `fork/main`。

初始化（只需做一次）：

```
git remote add upstream https://github.com/zed-industries/zed.git
git fetch upstream --tags
git config rerere.enabled true
```

## 二、改动放置规则

按"冲突风险从低到高"排列，能用上面的方式就不用下面的。

| 层级 | 放什么 | 位置 | 冲突风险 |
| --- | --- | --- | --- |
| 1. 新 crate | fork 自有功能逻辑、组件、常量 | `crates/fork_*/`（如 `fork_ui`） | 极低 |
| 2. 主题文件 | 颜色、字体、Cursor 风格配色 | `assets/themes/fork-cursor/`（新目录，不改内置主题） | 极低 |
| 3. 默认设置 | 面板位置、按钮显隐、默认宽度 | `assets/settings/default.json` 中对应面板的键 | 低（仅限改动的行） |
| 4. 图标资源 | 替换/新增图标 | `assets/icons/*.svg`；新增名称放在 `crates/icons` 枚举末尾 | 中 |
| 5. 核心 UI 代码 | 面板显隐初始化、标题栏/状态栏布局、圆角 token | `crates/workspace`、`crates/title_bar`、`crates/ui`、`crates/theme` | 高 |
| 6. 上游核心逻辑 | 编辑器、语言、Agent 内核、终端、gpui | 不改，除非绝对必要 | 最高 |

### 具体规则

1. **优先新 crate 挂接**：新功能在 `fork_*` crate 里实现，只在 `crates/zed` 的初始化处注册一次（一两行）。
2. **默认值只改值，不改结构**：`default.json` 中只改 `dock`、`button`、`default_width` 等具体值，不挪动字段、不重排段落、不改注释。
3. **不替换内置主题**：fork 主题单独成目录，`default.json` 的 `theme` 指向它。
4. **核心代码改动收敛**：改动核心文件时，只改必要的几行，不重写整段渲染函数，不改无关的格式和 import 顺序。
5. **登记台账**：每次修改上面第 3–6 层的文件，都在本文件"台账"一节登记文件、函数/键名、改动目的。合并时靠台账定位冲突。
6. **不做格式化和重命名**：不运行全仓 `cargo fmt` 或重命名上游符号，只格式化自己改动的行。
7. **注释规则**：遵循 `AGENTS.md`，不随意加注释。

## 三、提交规则

- 每个 fork 功能或改动独立提交，提交信息前缀统一为 `fork(<领域>): <说明>`，例如 `fork(theme): add cursor-like dark theme`。
- 不同领域的改动不混在同一个提交里，便于合并时单独跳过或重做。
- 台账中的每一项对应至少一个提交，方便追溯。
- `AGENTS.md` 要求修改源文件时在 `README.md` 顶部加入 `> [!IMPORTANT]` 提示行。这些行只能由人工删除，不要自行删除。建议把这部分单独放在一个提交中，合并时如遇冲突，手动保留。

## 四、同步上游

### 节奏
- 以上游 **release tag**（如 `v1.23.x`）为同步基线，而不是每天的 `main`，减少中间态冲突。
- 频率：每两周一次，或上游有重要安全/稳定性修复时立即同步。
- 不要积压超过一个 minor 版本。

### 步骤
1. `git checkout main && git fetch upstream --tags && git merge --ff-only upstream/main`（或快进到目标 tag）。
2. `git checkout fork/main && git merge <目标 tag or main>`。使用 merge 而不是 rebase：冲突只解决一次，历史清晰，rerere 也更有效。
3. 解决冲突，按第五节"冲突裁决"处理。
4. 验证：`cargo build`、`./script/clippy`、运行 Zed 检查面板位置、默认显隐、主题、图标是否正常。
5. 更新台账，推送 `fork/main`。

### 同步前的检查
- `git diff main...fork/main --stat`：看自己改动范围。如果核心文件（第 5、6 层）数量在增长，说明需要重新评估是否可以下沉到更低层。

## 五、冲突裁决（谁说了算）

### 1. 以 fork 为准（保留我们的改动）
- 台账中列出的 `default.json` 键值（面板位置、按钮、默认显隐、默认宽度）。
- fork 主题文件与 fork 图标文件。
- `fork_*` crate 中的所有代码。
- 圆角、间距等 fork 设计 token。
- 台账中登记的核心 UI 改动（标题栏、状态栏、面板布局），但结构以上游为准，我们的改动重新套上去。

### 2. 以上游为准（接受上游改动）
- 编辑器、语言服务、LSP、DAP、终端内核、Agent 内核、项目/Git 逻辑、gpui 框架。
- 上游 bug 修复、性能优化、安全修复。
- 新增的设置项。如果新键的默认值与我们的 fork 风格冲突，只在 `default.json` 中改那一行值。
- 上游新增的 UI 元素，默认接受；如果影响 Cursor 风格，再按规则调整。

### 3. 需要人工判断（合并时停下来问）
- 上游修改了我们台账中登记的核心函数（如 `workspace` 的 dock 初始化、标题栏渲染）。
- 上游删除或重命名了我们依赖的符号、图标名、设置键。
- 上游改动了我们替换过的图标文件内容。
- 上游修改了 `default.json` 中我们改过的键，且语义变化（而不是只改数值）。

### 4. 冲突处理原则
- 逻辑冲突：采用上游结构，把我们的改动以最小形式重新应用。
- 数值冲突（默认值）：以 fork 为准。
- 文件被删除：确认上游是否有替代，没有则在 `fork_*` 中补回。
- 任何裁决如果不确定，记录到台账的"待确认"栏，不要悄悄选一边。

## 六、降低冲突的设计原则

1. **集中修改**：所有 fork 的样式 token（圆角、间距、颜色）尽量集中到一处（`fork_ui` 或主题文件），而不是分散在各组件里。
2. **配置优先于代码**：能用设置或主题实现的，不改代码。
3. **不删上游代码**：要隐藏某个功能，优先用设置关闭，而不是删除代码。
4. **避免在长函数中间插入**：如果必须改核心函数，在函数末尾或独立的辅助函数中添加。
5. **避免改动高频文件**：`crates/workspace/src/workspace.rs` 等文件上游改动极频繁，修改尽量下沉到更小的模块或 `fork_*` 中。
6. **rerere**：开启后，第一次解决的冲突会被记住，下次同步时自动套用。

## 七、台账

每项改动登记：文件、位置（函数名或设置键）、目的、对应提交、冲突裁决（fork 或上游）。

| 层级 | 文件 / 位置 | 目的 | 提交 | 裁决 | 状态 |
| --- | --- | --- | --- | --- | --- |
| 3 | `assets/settings/default.json` → `project_panel`、`git_panel`、`collaboration_panel`、`outline_panel` 的 `dock`（改为 left） | 左侧竖排图标栏所属面板 | fork(settings): 左右竖排图标栏布局 | fork | 已实施 |
| 3 | `assets/settings/default.json` → `agent.dock`（改为 right）、`agent.button`/各面板 `button`（改为 false） | Agent 面板右侧；面板按钮改由竖排图标栏承载 | fork(settings): 左右竖排图标栏布局 | fork | 已实施 |
| 3 | `assets/settings/default.json` → `agent.threads_sidebar.position`（right）、`auto_open`（false） | Threads 侧栏放到最右侧且默认隐藏 | fork(settings): 左右竖排图标栏布局 | fork | 已实施 |
| 3 | `assets/settings/default.json` → `terminal.button`、`debugger.button`、`auto_update`、`title_bar.show_sign_in`/`show_user_menu`/`show_user_picture`、`edit_predictions.provider`、`show_edit_predictions` | 隐藏登录入口、自动更新、Edit Prediction（仅设置，不删代码） | fork(settings): 左右竖排图标栏布局 | fork | 已实施 |
| 4/5 | `crates/fork_ui/`（新 crate：`ActivityStrip` 竖排图标栏组件） | 左右竖排固定图标栏 | fork(ui): 新增竖排图标栏 crate | fork | 已实施 |
| 5 | `crates/workspace/src/dock.rs` → `Dock::activity_item`（新增方法） | 由面板生成图标栏条目（图标、选中态、切换动作） | fork(ui): 新增竖排图标栏 crate | 人工判断 | 已实施 |
| 5 | `crates/workspace/src/workspace.rs` → `Workspace::render_activity_strip`（新增）、`Workspace::render` 中 `bottom_dock_layout` 外层包裹左右图标栏 | 挂载左右竖排图标栏 | fork(ui): 新增竖排图标栏 crate | 人工判断 | 已实施 |
| 5 | `Cargo.toml`（workspace members/依赖）、`crates/workspace/Cargo.toml` | 注册 `fork_ui` | fork(ui): 新增竖排图标栏 crate | fork | 已实施 |
| 5 | `crates/workspace/src/dock.rs` → `Dock` 面板渲染中 `border_color`（`colors().border.opacity(0.3)`） | 面板之间的分割线淡化 | fork(ui): fade dock panel dividers | 人工判断 | 已实施 |
| 3 | `assets/settings/default.json` → `project_panel.folder_indicator`（改为 `chevron`）、`project_panel.indent_guides.show`（改为 `never`） | 文件夹左侧显示展开箭头（展开后为 v），去掉文件夹展开竖线 | fork(settings): project panel chevron without indent guides | fork | 已实施 |
| 6 | `crates/project_panel/src/project_panel.rs` → `render_entry` 的 `on_click` | 单击只选中，双击展开/收起文件夹并双击打开文件（上游核心逻辑，改动仅限点击分支） | fork(project_panel): double click to expand folders and open files | 人工判断 | 已实施 |
| 5 | `crates/workspace/src/multi_workspace.rs` → `MultiWorkspace::render` 中左右图标栏挂到最外层（Threads 侧栏在右图标栏左侧） | 图标栏贴边 | fork(ui): move activity strips to window edges | 人工判断 | 已实施 |
| 5 | `crates/workspace/src/status_bar.rs` → `StatusBar::add_strip_item` / `strip_item_views`、`SidebarStatus::query` 的 `show_toggle`（false） | 诊断指示器移入右侧图标栏；隐藏状态栏右下角 Threads 展开按钮 | fork(ui): move activity strips to window edges | 人工判断 | 已实施 |
| 5 | `crates/zed/src/zed.rs` → 删除 `search_button` 的状态栏注册，诊断指示器改用 `add_strip_item` | 去掉左下角搜索按钮 | fork(ui): move activity strips to window edges | 人工判断 | 已实施 |
| 5 | `crates/ui/src/traits/styled_ext.rs` 的 `rounded_lg` 等 | 圆角统一 | 待定 | 人工判断 | 待核实 |
| 4 | `assets/icons/*.svg`、`crates/icons/src/icons.rs` | 图标风格 | 待定 | fork | 待实施 |
| 3 | `assets/settings/default.json` → `status_bar.show_active_file`（true）、`line_endings_button`（true）、`active_encoding_button`（enabled） | 底栏左侧显示当前文件路径，右侧显示行尾符与编码 | fork(settings): status bar shows path, line ending and encoding | fork | 已实施 |
| 4 | `crates/fork_status_bar/`（新 crate：`IndentIndicator` 缩进指示器）、`Cargo.toml`（workspace members/依赖）、`crates/zed/Cargo.toml` | 底栏右侧显示缩进（Spaces/Tabs 与宽度） | fork(ui): add indentation indicator to status bar | fork | 已实施 |
| 5 | `crates/zed/src/zed.rs` → 状态栏 `indent_indicator` 注册（放在 `edit_prediction_ui` 之前） | 挂载缩进指示器 | fork(ui): add indentation indicator to status bar | 人工判断 | 已实施 |
| 5 | `crates/workspace/src/workspace.rs` → `Workspace::render_activity_strip` 中 `side == Left` 分支追加 `extensions` 条目（`IconName::Blocks`，动作 `zed_actions::Extensions`） | 左侧图标栏新增插件市场按钮，点击打开 Extensions 面板 | fork(ui): add extensions button to left activity strip | 人工判断 | 已实施 |
| 3 | `assets/settings/default.json` → `buffer_font_size`（15 改为 13） | 编辑器字体默认调小 | fork(settings): smaller editor font and hide nav history buttons | fork | 已实施 |
| 3 | `assets/settings/default.json` → `tab_bar.show_nav_history_buttons`（改为 false） | 隐藏编辑窗口的前进/后退箭头 | fork(settings): smaller editor font and hide nav history buttons | fork | 已实施 |
| 6 | `crates/project_panel/src/project_panel.rs` → `render_entry` 中文件名与路径分隔符的 `Label`（`LabelSize::Custom(rems(0.875))`）；`get_item_color` 移除 `focused`；`render_entry` 中 `bg_color`/`bg_hover_color` 对选中项使用 `marked` 背景色，`border_color`/`border_hover_color` 不再对选中项绘制 outline（仅保留校验错误颜色） | 文件面板字体略小；选中项由 outline 改为背景变色 | fork(project_panel): smaller font and background selection | 人工判断 | 已实施 |

### 待确认事项（实施前需要核实）
- 面板默认显隐：当前逻辑由 `crates/workspace/src/dock.rs` 中序列化的 `visible` 状态决定，是否可以只通过默认设置控制首次启动的显隐，还需要进一步验证。
- 圆角：`rounded_lg` 等来自 gpui 的样式方法，是否有统一配置入口需要确认；如果没有，只能逐个组件修改，冲突风险较高。
- 按钮位置：状态栏和标题栏的按钮布局代码分别位于 `crates/workspace/src/status_bar.rs` 和 `crates/title_bar/src/title_bar.rs`，具体改动方式需要在实施前阅读。

## 八、初始化检查清单

- [ ] 添加 `upstream` remote 并 fetch tags
- [ ] 开启 `rerere`
- [ ] 从 `main` 切出 `fork/main`
- [ ] 确认台账中每个待核实项
- [ ] 按层级顺序实施：先 3 层，再 2 层，最后 4、5 层
