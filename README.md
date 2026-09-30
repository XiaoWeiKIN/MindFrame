# MindFrame

**把知识创作整理成剪映素材包。配音、字幕、转场、特效与最终成片在剪映完成。**

你提供文档，在聊天中审改口播、分镜、概念、论点和公式，再生成真正需要的背景与插图。
MindFrame 校验实际文件并整理剪辑交接，不把程序测试片当发布作品，也不默认生成 MP4。

```mermaid
flowchart LR
    A[选定原文] --> B[聊天创作：口播 / 重点文字 / 实际图片]
    B --> C[MindFrame 导入与校验]
    C --> D[剪映素材包]
    D --> E[剪映：配音 / 字幕 / 图层 / 转场 / 成片]
```

## 安装与主流程

需要支持 edition 2024 的 Rust。素材主流程不需要模型 API Key、Node、FFmpeg、录音、时间点或剪映安装。
第一次编译仍需下载 Rust 依赖。在仓库根目录执行：

```bash
cargo install --path crates/mindframe-cli --force
mindframe init /path/to/article.md --out projects/article --preset douyin
```

`init` 接受一篇 UTF-8 Markdown（上限64 KiB），生成原文快照、行号副本和 `chat-request.md`。
将 `source.md` 和请求发到聊天中。请求完整内嵌 [Visual Director](skills/mindframe-visual-director/SKILL.md)、
工作表、[整篇口播规则](skills/mindframe-author/references/narration.md)、[主图与母版评审](skills/mindframe-visual-director/references/visual-review.md)及当前真实 Schema。已有项目不会自动更新旧请求；需要新版指导时初始化新目录或补充 Skill 文件。
PDF/Word 需先整理成忠实的 Markdown；CLI 不直接解析它们。

聊天完成后保存实际文件，最小输入仍只有两份 JSON 和真实主图：

```text
chat-output/
├── storyboard.json
├── assets.json
├── images/
│   └── background.png
├── layers.json          # 可选：独立叠加图片清单
├── overlays/            # 可选：layers.json 明确引用的真实图片
│   └── formula.png
└── motion.json          # 可选：逐句剪辑指导，不代表必须渲染
```

```bash
mindframe import projects/article --from ./chat-output
mindframe validate projects/article
mindframe export projects/article --target jianying --out dist/article
```

主流程到这里结束。没有录音、没有 cues 也能生成完整素材包，不为了导出素材要求先完成配音。

## 导出后拿什么去剪辑

```text
dist/article/
├── narration-review.md  # 逐幕来源、实际口播与画面强调；人工审阅，不自动评分
├── narration.txt        # 全片纯口播：不加标题、场景 ID、Markdown 注释
├── scripts/             # 001-scene-id.txt：逐镜纯口播
├── screen-text/         # 同编号：可复制的后期重点文字，不是字幕
├── images/              # 主画面，按场景编号
├── overlays/            # 有独立叠加图片才出现，原字节保留
├── cover.png            # 可选；扩展名以实际素材为准
├── edit-guide.md         # 总制作指南
├── edit-notes/           # 逐镜文件、画面意图和口播触发说明
├── shot-list.csv         # 保留原七列格式及场景顺序
├── subtitles.txt         # 无时间轴的字幕文本
├── subtitles.srt         # 仅有实际录音与完整已核对 cues 时生成
├── script.md             # 审稿版，不建议整份直接拿去配音
├── key-points.md
├── storyboard.md         # 与 edit-guide.md 相同的派生说明，保留旧文件名
├── storyboard.json
├── assets.json
├── layers.json           # 可选，路径已正规化
├── motion.json           # 可选，保留原编辑计划
├── material-report.json
└── README.txt
```

先用 `narration-review.md` 审阅全文的专业判断、铺垫和解释，再看 `edit-guide.md` 和素材报告。将图片分别作为素材导入，用纯口播文本准备声音；配音定稿后再安排字幕，
按逐镜说明放置重点与附加图。`screen-text/` 只包含用户明确提供的后期叠字，空文件表示未提供；不会从
图片上猜字或自动把标题再叠一次。

**这是素材包，不是剪映原生工程。** JSON/CSV/Markdown 不会自动排轨；ZIP 也不能一键打开为剪映工程。
具体导入、配音或字幕入口依用户版本核验，本项目没有宣称完成你的剪映 UI 验收。

## 单一编辑源与分层

`content/storyboard.json` 是口播/观点的唯一编辑源；`content/assets.json` 维护主图和 `screen_text`。
派生 TXT/Markdown 不是另一套编辑源。改 JSON 后重新导出到新目录，不能只改旧 `script.md`。

每个 scene 必须有一张显式映射的主图；同一真实背景可以被多个 scene 复用。支持 PNG/JPEG/WebP，
每文件最多50 MiB，边长不超过8192像素、解码预算256 MiB。图片扩展名必须匹配实际字节。
导入不拉伸、裁切、重画或“修复”公式，只有元数据与真实文件校验。

独立插图、公式图、关系图通过可选 `layers.json` 交接，不向旧 `assets.json` 添加未知字段。
下例仅展示格式，引用的实际图片必须先存在：

```json
{
  "schema_version": 1,
  "overlays": [
    {
      "scene_id": "scene-01",
      "id": "state-formula",
      "file": "overlays/formula.png",
      "label": "状态转移公式；已逐符号校对",
      "utterance_index": 1
    }
  ]
}
```

`utterance_index` 可省略；存在时必须对应该 scene 的真实口播索引，不能把整篇句号数量当索引。
每镜最多12层，id 在镜头内唯一。透明性按实际像素报告，不透明图片也可作为独立图解；
**独立 PNG 不是剪映原生文字层**，改公式仍需原始文本与重新排版。

## motion.json 现在主要用于剪辑指导

保留原有 text/stat/relation/matrix/formula 状态快照格式。每步绑定 narration 的零基索引，
用同一 id 表达同一对象。`export` 把它译成可读的“新增、保持、更新、移除”说明，例如：

```text
讲到 narration[1] 时：接着调整一次行动。
保持 you，不要重复入场。
更新 wang：18:00 → 19:00，并标为重点。
```

指导可在无录音时导出：没有秒数，只标语义触发。配音后在剪映设置实际入场、停留和动效。
不强制每2–4秒切一次，不接受模型生成的 JavaScript/CSS、任意像素坐标或可执行动画脚本。
无 motion.json 的简单项目同样受支持。

## 素材报告不是发布证书

`validate` 与 `material-report.json` 报告主图、封面、叠加图的尺寸和真实透明性；低于参考画布、比例不符、
不透明叠加图、未提供封面会提示复核，而不是擅自放大或生成占位素材。

参考画布：douyin 1080×1920、bilibili 1920×1080，是项目工作约定，不是已核实的平台要求。
小图标不按全屏尺寸判错。就算文件通过，也没有自动证明图片美感、文字正确、公式语义、手机阅读或发布效果。

[素材包说明](docs/editor-pack.md) 和 [视觉创作工作流](docs/visual-director.md) 说明文件与职责。
工作流固定“背景层 / 重点层 / 口播层 / 字幕层”，按用户选择设计风格，Ocean Depth / 深海星辰为可选示例；
以文字为主，不为了每个概念重新做密集海报。

## 可选录音与真实字幕

正常素材包只需要文字与图片。有实际 PCM16 单/双声道 WAV（最多256 MiB、一小时）时，可在 assets.json
提供 audio；只有声音仍不自动推算字幕。完整 cues 须精确匹配每个 narration、顺序不重叠且在录音范围内。
满足这些结构检查后才导出 SRT，不代表已实听核对。

更换配音、剪句或改语速后，旧 SRT 必须重做。成片 MP4 已烧入的字幕不能靠替换 SRT 恢复成编辑图层。

## 已有视频功能仅保留为可选预览

`mindframe preview` 只用于内部结构检查，旧 `motion` 是兼容别名，原 `motion.mp4` 输出名保留。
它不参与素材主流程，也不会从无录音文本生成假配音或时长。输出带 `PREVIEW.txt` 和
`preview-report.json`（publish_ready=false）。含 layers.json 的包目前不支持预览合成，明确报错而非静默丢层。

使用方法和依赖见 [可选预览](docs/preview.md)。旧 `plan/build/produce/render/demo` 路线保留，
不再作为产品主线；部分命令会调用配置的收费 API，不能自动从素材路径跳过去。
[旧路线文档](docs/legacy-video.md) 和历史媒体验证保留，不能把本次收敛当成依赖风险已修复。

## 校验与隐私边界

素材导入/导出拒绝覆盖已有目录，失败不留下半份输出；不并发写同一项目。
路径穿越、符号链接、损坏文件、不完整映射与未知字段会明确失败。
完整原文、聊天请求、API 配置、任意工作笔记和预览 MP4 不被导出；选取的原文引文仍在关键点文件中。
visual-plan.md、review.md 与额外动态背景须单独保存，不自动打包私有笔记。

MindFrame 不连接聊天会话、不抓 Cookie、不把订阅当 API 额度。图像由当前聊天工具生成；
只有工具暴露型号选择时才能保证具体型号，文件存在不证明由某个模型生成。

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
python3 scripts/chat_smoke.py --out output/chat-smoke
```

主流程测试使用无密钥、空 PATH 的实际 CLI，合成图和静音 WAV 只证明协议行为。
原有 `scripts/check.py`、legacy 与 Motion 媒体 CI 保留，不用删断言取得通过。
当前工作记录：[editor-materials-first](docs/exec-plans/active/ep-001_v1/editor-materials-first.md)；
父 [EP-001](docs/exec-plans/active/ep-001_v1/EXECPLAN.md) 保持 active，不伪造正式归档。

口播生成与审阅改进记录：[narration-authoring](docs/exec-plans/active/ep-001_v1/narration-authoring.md)。
