# 使用 MindFrame Visual Director

这是一套**素材创作与剪辑交接工作流**。它不自动控制剪映，不默认生成 MP4，也不要求音频或 cues。

```mermaid
flowchart LR
    A[选定原文] --> B[概念 / 论点 / 公式 / 来源]
    B --> C[口播与屏幕重点分离]
    C --> D[提炼机制与图解提示词]
    D --> U[用户生图并回传]
    U --> E[实图校对与素材打包]
    E --> F[剪映完成声音 / 字幕 / 动效 / 成片]
```

## 取得当前指导

```bash
cargo install --path crates/mindframe-cli --force
mindframe init /path/to/article.md --out projects/article --preset douyin
```

上传 source.md 和 chat-request.md。新版 init 在编译时内嵌作者提示、完整 Skill、工作表以及
整篇口播规则、主图与母版评审规则以及 Storyboard/Assets/Motion/Layers Schema；在仓库外运行无需重新上网查这些文件。
旧请求不会自动改写，使用新目录初始化或把新 Skill 作为补充。

示例要求：

> 按知识讲解模式准备剪映素材包，采用深海星辰风格。保留原文的路径依赖、反身性和状态方程。
> 口播与重点文字分开，先按原文与口播提炼必要图解，提供可独立使用的生成提示词与文件清单。
> 我生成后打包回传，你验收并映射分镜，交按口播触发的剪辑说明。不默认生图、生成 MP4 或测试配音。

## 分工

背景负责低干扰的阅读氛围；重点层呈现概念、公式和必要关系；口播负责解释；字幕由实际声音决定。
保持简单，不把所有段落画进海报，不强制每镜更换背景，也不为纯文字概念生成无用装饰。

主图设计、生成提示或回传图评审参考 [主图与母版评审](../skills/mindframe-visual-director/references/visual-review.md)。
先区分背景、解释图与封面；背景负责承托内容，解释图负责关系。用户自行生成时只交建议和评审。
可先验证一张母版，再按真实布局需要扩展；不固定四张，不为保持新鲜感重复换装饰。
裸图、实际内容叠加、手机阅读及播放效果分别观察，不用风格或文件校验推断传播效果。

storyboard.json 是唯一口播编辑源。assets.json 是主图与后期叠字清单。
layers.json 可选，登记实际叠加图片；motion.json 可选，组织逐句状态，导出人能照着做的剪辑指导。
新增 id、保留 id、改变内容和省略 id 分别表现为新增/保持/更新/移除说明，不是剪映自动动作。

准确公式字符串保留；需要独立位图时另行排版和校对。PNG不能逐字修改，不能拿它冒充原生文字层。
默认不配音，声音与语速定稿后再做字幕；没有录音不造秒数或 SRT。

## 提示词交付与用户回传

默认按 Skill 的“图解提示词与回传闭环”执行，使用[提示词模板](../skills/mindframe-visual-director/references/diagram-prompts.md)；新版 init 完整内嵌模板，仓库外也可使用。diagram-prompts.md 交统一风格、逐图准确文字、节点/箭头、限定、
布局、文件名及交付要求；visual-plan.md 记录原文定位和当前口播的 scene_id/narration 索引与原句。
图片数量按解释任务确定，不固定张数；纯文字金句与公式高亮优先在剪映完成。同一图的显示状态不默认新增生图。
用户回传实际图片即可，ZIP 是便利形式而非强制条件，不要求生成工具写 MindFrame JSON。
提示词阶段不运行缺图包的 import，不把目标文件名当实际素材；回传验收后补映射，缺图与待修图明确报告。
diagram-prompts.md 与工作记录单独交付，当前 CLI 不自动保留。用户明确要求代为生图时可沿用实际图像工具流程。

## 实际文件与验收

[素材包文档](editor-pack.md) 说明 narration.txt、scripts/、screen-text/、edit-notes/ 和叠加图。
只有存在的真实文件才写清单；PNG/JPEG/WebP均需真实格式。透明性按像素报告，不由后缀猜测。
visual-plan.md 与 review.md 单独保存，不自动公开工作笔记。

CLI校验文件、映射和范围；Skill的内容/公式/审美检查必须看过真实素材后才能记录 pass。
未知或未看过写 unverified，待修写 revise；CI通过不等于成品美观、声音自然或发布效果好。

独立使用时提供完整 skills/mindframe-visual-director/ 目录。仓库里有文件不等于宿主已安装，
不代表能选择未暴露的图像型号。保留历史图文示例，它只是合成协议示范，不是默认视频交付。
只有明确需要内部结构样片时参考[预览文档](preview.md)，不绕过项目另写临时成片脚本。
