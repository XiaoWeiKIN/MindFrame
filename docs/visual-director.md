# 使用 MindFrame Visual Director

这是创作工作流，不是新的图像后端。重点是把已写好的精华文章转成独立图文页，保留概念、金句、公式与论证边界。没有增加收费 API、后台服务、JSON 协议或新的渲染命令。

```mermaid
flowchart LR
    A[选定原文] --> B[锚点与来源]
    B --> C[逐页文字与关系图]
    C --> D[统一视觉规则]
    D --> E[宿主图像工具逐页生成]
    E --> F[实际图片检查]
    F --> G[现有 MindFrame 导入与导出]
```

## 通过 CLI 取得完整工作流

从包含本次改动的版本重新安装，然后使用一个新输出目录：

```bash
cargo install --path crates/mindframe-cli --force
mindframe init /path/to/article.md --out projects/article-visual --preset douyin
```

把 `projects/article-visual/source.md` 和 `chat-request.md` 交给聊天助手。新版请求在编译时内嵌完整的视觉导演与逐页工作表，并附上实际 Storyboard/Assets Schema。在仓库之外运行已安装 CLI 也不需要联网获取这些文件。

已有项目的 `chat-request.md` 不会自动改变；不要覆盖已有目录。可以用同一原文初始化新目录，或将新 Skill 与模板作为补充发给助手。导入格式没有变，不需要迁移旧素材。

示例请求：

> 按 MindFrame Visual Director，把附件做成8张独立抖音图文，保留路径依赖、反身性和原文核心公式。先确定各页文字与逻辑关系，再逐页生成。原文是内容依据，不再压成摘要；不要做九宫格。每页的准确文字、公式和必要限定都需要审核。

如果只需要剪辑配图，明确说“剪辑配图模式，正文后期叠字”；图文成品则需要每页本身完整可读。页数和9:16是工作选择，不是对平台规格或流量的保证。

## 文件与职责

| 文件 | 用途 | CLI 是否自动导入/导出 |
|---|---|---|
| storyboard.json | 唯一内容编辑源，包含旁白与引用 | 是，遵守已有Schema |
| assets.json与实际图片 | 场景对应、真实图像与需要后期叠加的文字 | 是 |
| visual-plan.md | 必留概念/金句/公式，P0/P1/P2，构图、风格、关系 | 否，单独保存 |
| 逐页提示与review.md | 实际生成说明、候选图、审图观察、修订状态 | 否，单独保存 |

P0/P1/P2、formula、importance、layout不是当前可任意加入JSON的新字段；使用Markdown工作记录，不破坏严格导入协议。图文已印在图片上的字，不要再填成重复的后期叠字。

正式素材齐全后仍执行现有命令：

```bash
mindframe import projects/article-visual --from ./chat-output
mindframe validate projects/article-visual
mindframe export projects/article-visual --target jianying --out dist/article-visual
```

每页一张真实图片，不把切割的低清九宫格冒充独立成图。没有图片不提供假路径，没有录音和核对时间点不生成假SRT。现有CLI不负责生成图片、排版或验证实际语义/美感。

## 独立使用 Skill

完整目录为 `skills/mindframe-visual-director/`。在支持文件阅读的聊天中提供 `SKILL.md` 和 `references/page-plan.md` 即可读取工作流；在支持Skill安装的宿主中按该宿主的机制加载完整目录。仓库中存在文件不等于已自动安装，更不代表可以指定当前工具未暴露的图像型号。

完整示例见 [walkthrough](../skills/mindframe-visual-director/examples/walkthrough.md)，采用原创合成材料，含分支、闭环和公式三页方案。示例没有生成图片，视觉检查均为unverified；其JSON用于测试，缺少实际图片时不可直接导入。

## 验证边界

`cargo test --workspace` 会覆盖新请求的内嵌内容、仓库外离线运行、示例引用/公式/现有协议兼容，以及实际合成图导入导出。`cargo clippy --workspace --all-targets -- -D warnings` 检查Rust代码。

这些检查不证明图像模型会完全遵守说明，也不证明中文排版、哲学解读或传播效果。真实生图后仍需逐页查看，并记录pass/revise/unverified及观察。当前改动不安装第三方Skill、不提交用户知识库、不改原生剪映工程或自动发布能力。
