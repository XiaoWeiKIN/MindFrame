---
name: mindframe-author
description: 基于指定原文编写忠实讲解稿，并通过视觉导演组织文字优先的知识视频分层素材或独立图文；保留概念、金句、公式与来源，交接现有MindFrame文件协议。
---

# MindFrame Author

按 [chat-authoring.md](../../prompts/chat-authoring.md) 创作。
先读取 `mindframe init` 的chat-request.md与原文，取得实际Storyboard/Assets Schema，不能凭记忆造字段。
请求已内嵌完整 [mindframe-visual-director](../mindframe-visual-director/SKILL.md) 与工作表；无需额外安装才能阅读。

作者负责单一口播编辑源与论证忠实，导演负责信息层级、模式、风格和实际素材检查。
用户要视频、突出文字/公式、字幕随口播出现时进入知识讲解(text-first-lecture)，不用密集带字海报。
背景层 / 重点层 / 口播层 / 字幕层分开；无字背景可整期复用，Ocean Depth / 深海星辰为可替换风格。
不从静态截图确认背景动画；动图要实际制作与播放，口播要实际录音，字幕同步要实际对齐。

用户明确要滑动图文时每页完整可读，独立制作，不用九宫格替代。用户仅讨论项目或沉淀Skill时不额外生图。
精华原文不反复摘要，不把作者的类比包装成哲学定论，不用“必然成功”等口号替换原文限定。
只读指定素材，不扫描整库、不上传额外服务、不隐式调用收费API。真实需要生图时调用宿主实际工具。

storyboard.json是唯一口播编辑源。P0/P1/P2、LaTeX、语义触发、分层与运动意图放visual-plan.md；
review.md分别记录静态、运动与同步检查。当前CLI只接实际位图、可选WAV和句级cues，
不接MP4背景、动画字段或多轨工程；这些另行交接，工作记录也不被import/export自动复制。
无录音不写audio，无核对时间点不写cues。不伪造媒体、路径、模型型号或完整视频完成状态。

仓库中的Skill不等于已安装到当前宿主；读取/安装以宿主机制为准。聊天与CLI仍通过实际文件交接。
