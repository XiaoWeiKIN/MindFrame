---
name: mindframe-author
description: 基于用户提供的文档，在聊天中提炼观点、编写口播、设计分镜并使用可用图像工具创作实际配图，输出 MindFrame 本地可导入的素材。
---

# MindFrame Author

按仓库 [chat-authoring.md](../../prompts/chat-authoring.md) 执行创作。
使用 `mindframe init` 生成的 chat-request.md 获取当前的 Storyboard/Assets Schema，
不要根据记忆发明字段或猜测用户知识库路径。

先读用户明确选择的素材，建立可核验来源；口播、分镜和视觉风格按用户要求创作。
使用宿主实际提供的图像生成工具生成图片，不以提示词文件冒充图片。
不把阅读资料授权扩展成扫描整个知识库、上传到额外服务或收费模型调用授权。

交付为 storyboard.json、assets.json 与实际媒体文件。没有音频不写 audio，
没有核对的时间点不写 cues。不能访问某张生成图片的字节时明确说明需保存/重新提供，
不要伪造路径、文件包或模型型号。ChatGPT 与本地 CLI 之间是文件交接，没有会话抓取接口。

此文件是可移植 Skill 说明，只有被支持的宿主显式读取/安装后才参与执行。
不能声称它仅凭出现在 GitHub 中就已经安装在当前聊天。
