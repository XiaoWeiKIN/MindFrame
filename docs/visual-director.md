# 使用 MindFrame Visual Director

同一份原文可走两种制作路径：**文字优先的知识讲解**，或**独立阅读的图文页**。
这是创作Skill，不是新的图像/视频后端。它保留概念、论点、金句、公式与限定，组织表达与素材交接。

## 知识讲解：本轮沉淀的路径

用户要简单背景、核心要义突出、后加口播和字幕时，采用text-first-lecture：

```mermaid
flowchart LR
    A[指定原文] --> B[概念 / 论点 / 公式与来源]
    B --> C[讲解稿与重点层]
    C --> D[无字背景 + 独立排版]
    C --> E[实际配音后对齐字幕]
    D --> F[剪辑中分层合成]
    E --> F
```

背景层负责气氛，重点层负责概念/公式，口播负责展开，字幕跟随实际声音。
背景可以整期复用，不因换一句话重新生图；文字与公式不烘焙到背景里。

预设 **Ocean Depth / 深海星辰**：墨蓝底、柔白字、少量暖金，低对比海面或稀疏星空，中央阅读区安静。
海面慢动、星点缓移属于运动设计；静态图不能算循环视频。先验证代表镜头，再扩展其余镜头。
风格可替换，分层方法不变。参考截图不证明时序，也不授权复制账号标识。
画幅按用户已确认的母版，不因“抖音”自动切竖屏；项目preset不会改变真实图片尺寸。

可直接给助手的请求：

> 按MindFrame Visual Director的知识讲解模式处理附件。保留原文概念、论点、金句和公式，少字但不丢限定。采用深海星辰风格，16:9母版，无字背景复用，重点文字/公式独立排版，口播字幕另做。先给讲解稿、重点层与逐句触发方案；生成素材时区分静态底图、动态背景和最终视频，不做九宫格或密集海报。

这只是示例请求，不是CLI参数；没有新增`--style`或动画渲染命令。

## 通过CLI取得完整工作流

从包含本次修改的分支安装，在新目录初始化：

```bash
cargo install --path crates/mindframe-cli --force
mindframe init /path/to/article.md --out projects/article-visual --preset douyin
```

把生成的source.md和chat-request.md交给聊天助手。请求内嵌完整Skill和逐镜工作表，再附实际JSON Schema。
指定目标母版画幅；preset是已有提示，不代表已裁切或确认平台规格。
既有请求不自动升级，不覆盖原项目；用同一原文新建目录或补充新版Skill。

独立使用时，提供`skills/mindframe-visual-director/SKILL.md`及`references/page-plan.md`。
文件在仓库里不代表已安装到聊天宿主，也不赋予未暴露的图像型号选择能力。

## 已支持与仍需制作的部分

| 产物/行为 | 当前边界 |
|---|---|
| 原文、口播与来源引用 | 使用现有Storyboard；script.md为派生输出 |
| 静态无字底图与重点叠字文本 | 现有Assets支持实际PNG/JPEG/WebP与screen_text |
| 同一底图跨镜头复用 | 每个scene显式引用真实文件，导入会复制为场景文件 |
| 逐句字幕 | 仅实际WAV与核对过的句级cues可导出SRT；无音频不估时 |
| 语义触发、P0/P1/P2、LaTeX、布局、风格 | 记录于visual-plan.md，不擅加JSON字段 |
| 实际背景MP4、词级字幕、透明图层与多轨合成 | 当前素材CLI不支持，需另外制作与交接 |
| 实图、实播与听音审核 | review.md分项记录pass/revise/unverified，不能由CI替代 |

导入可用的位图、稿件和可选录音后，仍使用原命令：

```bash
mindframe import projects/article-visual --from ./chat-output
mindframe validate projects/article-visual
mindframe export projects/article-visual --target jianying --out dist/article-visual
```

动态背景不写入assets.images。visual-plan/review及额外媒体不会自动复制，另行保存。
新素材格式不能直接走旧produce/render；此改动没有打通新格式的自动视频合成，也不是剪映原生工程。

## 图文成品模式仍保留

明确要求滑动图文时每页自身完整可读，按源文分页并独立生成，不以九宫格或无字底图冒充成品。
[原图文示例](../skills/mindframe-visual-director/examples/walkthrough.md)使用合成文本，未包含真实生图。
新路径是另一种呈现方式，不改变用户已经选定的图文订单。

## 验证

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

新增测试检查：仓库外无密钥init完整带出分层/风格/时间边界；同一真实位图跨镜头导入导出且保留叠字；
不接受未经实现的动态字段。它们不证明模型遵循风格，不证明背景循环、字幕同步或视频质量。
旧有CI和媒体门禁不放宽；没有增加依赖、API调用、协议字段或发布行为。
