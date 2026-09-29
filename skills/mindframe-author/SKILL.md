---
name: mindframe-author
description: 基于用户提供的文档，在聊天中组织观点、编写口播并使用视觉导演逐页生成概念图文或视频配图，输出 MindFrame 本地可导入的素材。
---

# MindFrame Author

按仓库 [chat-authoring.md](../../prompts/chat-authoring.md) 执行创作。
使用 `mindframe init` 生成的 chat-request.md 获取当前的 Storyboard/Assets Schema，
不要根据记忆发明字段或猜测用户知识库路径。新版请求已内嵌完整的视觉导演工作流与工作表，
拿到请求时不需要另装 Skill 或再次读取仓库才能继续。

文章图文、概念卡与知识视频配图进入 [mindframe-visual-director](../mindframe-visual-director/SKILL.md)。
作者负责忠实内容与单一分镜编辑源；导演负责概念/金句/公式锚点、分页、视觉关系、共享风格、
单页生图和实际图片检查。精华原文不反复摘要，不把作者的系统类比包装成古人名言或定律。

先读用户明确选择的素材，建立可核验来源。图文成品与剪辑配图分开交付：前者每页需完整可读，
后者可以保留叠字；不以九宫格总览、提示词或无字背景冒充整组图文成品。
使用宿主实际提供的图像生成工具，不把阅读授权扩展成整库扫描、额外服务上传或收费 API 调用。

交付为 storyboard.json、assets.json 与实际媒体文件。导演的 P0/P1/P2、公式原文与审图记录
放在 visual-plan.md / review.md 中，不能加入未经支持的 JSON 字段；这些工作文件需另行保存，
当前 import/export 不会自动复制。没有音频不写 audio，没有核对时间点不写 cues。
不能访问某张图片的字节时明确说明缺失，不伪造路径、文件包、完成状态或模型型号。

此文件是可移植 Skill 说明，只有被宿主显式读取/安装后才参与执行；不能声称仅因出现在
GitHub 中就已自动安装。聊天与本地 CLI 仍通过实际文件交接，没有会话抓取接口。
