---
name: mindframe-author
description: 基于用户明确提供的文档，在聊天中创作口播、分镜、屏幕重点与实际图片，交付可由 MindFrame 导出为剪映素材包的文件。配音、字幕时序和成片交给剪映。
---

# MindFrame Author

按 [chat-authoring.md](../../prompts/chat-authoring.md) 与 [Visual Director](../mindframe-visual-director/SKILL.md) 创作。
默认是素材包，不是自动视频任务。不默认生成 MP4，不用测试音色替正式配音，不要求音频或 cues。

通过 mindframe init 获取完整 chat-request.md，内含当前 Storyboard/Assets 以及可选 Motion/Layers Schema。
不根据记忆发明字段，不扫描用户整库。原文提供思想，传播改写保留概念、金句、公式、逻辑关系与限定。
原文引用、改写、比喻分开记录，不把作者的总结冒充古人名言。

创作或改写知识讲解口播时，读取 [口播与整篇审稿规则](references/narration.md)。面向未读原文的观众
直接讲知识；用案例和机制铺垫，保留精确的专业判断，再解释术语与现实含义。审查全文的理解路径，
不只润色一处例句；逐字朗读、导读等用户指定体裁按其要求处理。

storyboard.json 是唯一口播编辑源。assets.json 关联主图和后期重点文字；需要独立附加图片时用
layers.json 登记真实位图。按需要提供 motion.json 的逐句状态，供导出剪辑指导，不主动执行 renderer。

生成素材使用宿主真实图像工具。正文、字幕不要烧进背景；公式保留准确文本，需要位图时单独校对。
口播与最终声音不同；只有真实录音和核对时间点才提供音频/cues。语音变化后必须重新对齐。

CLI 派生 narration.txt、scripts/、screen-text/、edit-notes/ 等文件，用户不必手写重复稿件。
narration-review.md 将逐幕来源、真实口播与画面强调放在一起供审阅，不自动判定口播质量。
visual-plan.md 和 review.md 单独保存，不自动导出任意私有笔记。没有图片不造路径，不把提示词当素材。
文件在仓库存在不等于宿主自动安装；不承诺未暴露的模型型号、剪映界面控制或原生草稿输出。
