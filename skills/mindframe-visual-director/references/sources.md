# 参考实现与本项目差异

查阅日期：2026-09-29。以下是一手仓库文档，不以 Star 数推断可用性或传播效果。
本 Skill 独立撰写，借鉴工作流思路，不安装、不调用、不复制上游脚本，也不继承其特定宿主/模型假设。

| 参考 | 本次核对内容 | 采用的设计思路 |
|---|---|---|
| [baoyu-article-illustrator](https://github.com/JimLiu/baoyu-skills/blob/main/skills/baoyu-article-illustrator/SKILL.md) | 文件blob 87e27e6d83f420605de2f6992454d71874d8dc44 | 信息类型、风格与配色分开考虑；每图保留独立说明 |
| [baoyu-infographic](https://github.com/JimLiu/baoyu-skills/blob/main/skills/baoyu-infographic/SKILL.md) | 信息图布局与风格分层、忠实保存源内容 | 关系决定图形结构，不用氛围替代信息 |
| [codex-image](https://github.com/philipbankier/codex-image-skill/blob/main/.agents/skills/codex-image/SKILL.md) | 文件blob 07f2029e409d61c74d09b84344814fb007ef9cc3；[README](https://github.com/philipbankier/codex-image-skill/blob/main/README.md) blob 841a56b4fe3a62474d3199ee3f19b3055d96b3c6 | 明确阅读层级、共享视觉方向、生成后检查 |

上游分别提供[Jim Liu的MIT许可](https://github.com/JimLiu/baoyu-skills/blob/main/LICENSE)和[Philip Bankier的MIT许可](https://github.com/philipbankier/codex-image-skill/blob/main/LICENSE)。此次未引入其代码或成段文本；后续直接复制上游文件时应保留相应版权与许可声明，不能把本说明当作再分发许可副本。

MindFrame 的取舍：以已选原文为唯一知识来源；概念、金句与公式不可被摘要抹平；社媒页独立生成而非默认高密度总览；图文成品和剪辑背景分开验收。无新增 provider、后台作业或模型调用。

型号、安装和能力均以当前宿主实际暴露的工具为准。上游 README 中的型号描述不证明当前聊天可选该型号。口播文字、真实录音、字幕时间点和语音对齐也分别记录，不互相替代。
