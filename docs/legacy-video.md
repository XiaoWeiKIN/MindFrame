# 可选旧视频管线

新主流程是 init/import/export 的本地素材交接。此处记录保留的旧 API/Remotion 路径，
不把它作为新素材项目的自动后端，也不保证与任意宣称兼容的 API 服务通用。

需要 Node.js 22、FFmpeg/ffprobe、中文字体和可用浏览器。先执行：

```bash
npm --prefix renderer/remotion install
cp mindframe.example.toml mindframe.toml
```

按本机配置设置 API 凭据（不要提交到 Git）。旧方式：

```bash
mindframe plan article.md --out output/legacy
# 审核 output/legacy/storyboard.json
mindframe produce output/legacy --preset both
# 已有完整音轨/时间轴后修改视觉，可不再调用模型：
mindframe render output/legacy --preset both
```

或者 `mindframe build article.md --out output/legacy --preset both` 一次完成。
`mindframe demo --out output/demo --preset both --scale 0.5` 使用人工分镜和本地 eSpeak，
不代表在线模型效果，且仍需要本地媒体依赖。

旧目录为根层 source.md、storyboard.json、assets/、timeline.json、voice.wav、
subtitles.srt 和各布局的 MP4/封面。新 project.json + content/ 素材项目不可传给 produce/render。
旧 API 配音逐句生成、测量并按30fps补齐；这与素材模式的用户提供时间点是不同契约。

本机没有 Rust 工具链时用 GitHub Actions 验证编译，不应把未执行的结果标成通过。
历史提交 e3bfca7 的 CI 已通过编译和静态测试，但在媒体 H.264/yuv420p 断言处失败；
保留该验收，不通过删除或放宽断言掩盖问题。以新提交的实际 CI 和 EP 记录为准。
