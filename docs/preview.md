# 可选结构预览（非成片交付）

默认使用 init → import → validate → export，把素材交给剪映。
preview 只保留为内部检查工具，不扩展成新的配音或视频制作系统。

```bash
mindframe preview projects/article --out output/article-preview --preset douyin --renderer renderer/remotion --scale 0.25
```

旧命令 mindframe motion 为兼容别名；参数与旧输出名 motion.mp4/cover.png 保留。
运行需要本机已有 Node/Remotion 和可用浏览器，以及实际 WAV 与全部已核对 cues。
与以前一样，首次准备 renderer 依赖/浏览器可能联网；素材主流程没有这项要求。
没有声音或时间点就明确失败，不补测试音色、不按字数估时、不偷偷转到收费 API。

输出额外包含 PREVIEW.txt 与 preview-report.json，其中 purpose=structural_preview、publish_ready=false。
全尺寸渲染也不等于可发布作品。文件校验、实图观察、实播和实听是不同的验收事项。

旧预览不支持 layers.json 的附加位图合成。遇到分层包在启动 Node 前明确失败，避免静默丢层。
不要为了预览删掉正式包的图层再宣称完整展示；正式制作继续用 export 的全部素材。

本次没有改动 Remotion 视觉模板、增加渲染能力、修复已知依赖安全告警或验证你的剪映版本。
