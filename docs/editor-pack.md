# 剪映素材包

默认交付到素材包为止，不默认生成 MP4。主流程不要求音频或 cues。

## 制作入口

```bash
mindframe init article.md --out projects/article --preset douyin
# 把 source.md 与 chat-request.md 发到聊天中，保存实际创作文件到 chat-output/
mindframe import projects/article --from chat-output
mindframe validate projects/article
mindframe export projects/article --target jianying --out dist/article
```

每一步只处理显式文件，不连接剪映界面。没有安装剪映也可以准备素材。

## 复制什么

narration.txt 是纯口播全文；scripts/ 每镜一份纯口播，没有自动添加的标题、编号或 Markdown。
screen-text/ 与 scripts/ 使用同一场景编号，只含后期重点文字；空文件表示未提供，别从图片意图猜内容。
script.md 是审稿版。导出文本从当前 JSON 生成，修改派生文件不会回写工程。

images/ 是主画面，overlays/ 是可选附加位图，cover.* 可选。透明像素和尺寸见 material-report.json。
图像字节原样复制；有透明通道不代表一定有透明像素，不透明图解会提示遮挡。

edit-guide.md 汇总镜头内容，edit-notes/ 可单独打开。motion.json 的每一步会转成
“讲到 narration[n] 的哪句原话时，新增/保持/更新/移除哪些对象”。公式、关系、矩阵原样保留为
制作信息，不是已经排好版的剪映对象。没有音频就不写任何估算秒数。

## 可选附加图层

Layers Schema 随 init / schema 命令生成。layers.json 位于素材目录根部：

```json
{
  "schema_version": 1,
  "overlays": [
    {"scene_id": "scene-01", "id": "formula", "file": "overlays/formula.png", "label": "状态转移公式", "utterance_index": 1}
  ]
}
```

这个示例说明结构，不代表已经生成 formula.png。必须有实际图片才能导入。
scene_id 必须存在；id 在该镜头内唯一，每镜最多12层；utterance_index 可省略或引用有效零基口播索引。
file 是包内相对路径，不能用网页 URL、附件 ID、路径穿越或符号链接。支持 PNG/JPEG/WebP，保留真实编码。

不要把 layers 填到 assets.json。旧 Assets v1 保持严格；独立 sidecar 避免改变原有字段的含义。
旧版本 CLI 不理解新的 sidecar，因此带叠加图的包必须用更新后的版本处理；不宣称旧二进制可以识别新功能。

## 在剪映中完成

将主图和所需叠加图导入，按镜头编号摆放。用纯口播准备声音，在声音定稿后制作字幕；
按说明放重点文字、图解，再选择转场与效果。JSON/Markdown/CSV 不是自动轨道，素材包不是原生草稿。
我们没有验证具体版本的菜单路径、SRT导入入口或任何付费配音套餐。

已有音频和完整已核对时间点才导出 subtitles.srt。新配音或改语速后重新做时间轴，不能沿用旧 SRT。
音频文字一致和时长合法只证明结构，通过不表示实听同步或公式/内容正确。

## 复核与隐私

报告中的画布/比例/透明性/封面提醒不会修改文件，也不是质量分数。小的独立图标不按全屏分辨率判错。
检查大字、公式、字幕留白和手机实际阅读尺寸；合法 PNG 也可能不好看。
完整原文与任意笔记不导出，关键点保留选取引文。visual-plan.md / review.md 仍需另存。
不把含烧入字幕的 MP4 当作可分层工程，也不把 eSpeak/CI 测试片当成发布作品。
