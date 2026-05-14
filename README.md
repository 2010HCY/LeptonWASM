# Lepton WASM

[简体中文](./README.md) | [English](./README.EN.md)

Lepton 是由 Dropbox 开发的 JPG 无损压缩工具，可在不损失画质的前提下将文件体积缩减约 20%。**Lepton WASM** 是基于微软开源的 [lepton_jpeg_rust](https://github.com/microsoft/lepton_jpeg_rust) 编译的 WebAssembly 版本，旨在为浏览器端提供高效的图片压缩与即时预览能力。

![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/platform-WASM-blue.svg)

## 特性

- 在浏览器前端直接还原 `.lep` 为 `.jpg` 显示，画质零损失。
- 支持 WASM SIMD (128位) 指令集，解码速度提升 25%。
- 提供“仅查看版 (Viewer)”与“全能版 (Full)”两种编译方案。

![Lep在线查看示例](./images/Lep在线查看示例.png)
![Lepton 在线无损压缩、解压JPEG](./images/Lepton%20在线无损压缩、解压JPEG.png)

## 1. 快速构建

### 环境准备
-   **Rust 工具链** (1.89+)
-   **wasm-pack**: 用于编译和打包 WASM。
    ```bash
    cargo install wasm-pack
    ```

### 手动编译选项
| 版本 | 功能 | 编译命令 |
| :--- | :--- | :--- |
| 查看版 (Viewer) | 仅解码Lep文件 | `wasm-pack build --target web -- --features viewer` |
| 全能版 (Full) | 编解码 | `wasm-pack build --target web -- --features full` |

---

## 2. 性能与指令集加速 (SIMD)

由于 WASM 运行在虚拟机环境，无法直接使用 x86 的 AVX 指令集。我使用 **WASM SIMD (128-bit)** 实现硬件加速：

```
RUSTFLAGS="-C target-feature=+simd128" wasm-pack build --target web
```

> Tips: 建议同时存放两个版本的 `.wasm`。在 JS 中通过 `wasm-feature-detect` 检测环境，动态加载SIMD或兼容版以获得极致性能。

---

## 3. 前端接入指南

### 基础 API

```javascript
import init, { lep_to_jpg, jpg_to_lep, get_version } from './pkg/lepton_wasm.js';

async function run() {
    await init(); // 初始化 WASM 模块
  
    // .lep -> .jpg
    const jpgData = lep_to_jpg(lepUint8Array);
    const blob = new Blob([jpgData], { type: 'image/jpeg' });
    document.getElementById('my-img').src = URL.createObjectURL(blob);
}
```

编译后生成的胶水代码提供了以下核心接口：

- `init()`: 初始化 WASM 模块。必须在使用前调用。
- `lep_to_jpg(data: Uint8Array) -> Uint8Array`: 
  - 输入：.lep 文件的二进制数组。
  - 输出：.jpg 文件的二进制数组。
- `jpg_to_lep(data: Uint8Array) -> Uint8Array`: (仅 Full 版)
  - 输入：.jpg 文件的二进制数组。
  - 输出：.lep 文件的二进制数组。
- `get_version()`: 返回核心库版本。

### 多线程处理 (Web Worker)

由于解码是计算密集型任务，直接在主线程运行会导致 UI 卡顿，造成需要所有图片都解码完毕才能操作网页、显示图片。强烈建议在 Web Worker 中调用解码接口：
1. 主线程发送 `.lep` 二进制数据给 Worker。
2. Worker 在后台调用 `lep_to_jpg`。
3. Worker 将结果传回，主线程仅负责渲染。

---

## 4. Demo 演示

[WebAssembly SIMD 检测](https://page.hcyhub.com/%E5%B0%8F%E5%B7%A5%E5%85%B7/SIMD%E6%94%AF%E6%8C%81%E6%80%A7%E6%A3%80%E6%B5%8B/)

[Lep在线查看示例](https://page.hcyhub.com/%E5%B0%8F%E5%B7%A5%E5%85%B7/Lepton%20Web/DemoLepViewer/)

[Lepton 在线无损压缩、解压JPEG](https://page.hcyhub.com/%E5%B0%8F%E5%B7%A5%E5%85%B7/Lepton%20Web/DemoTinyLep/)

你可以参考仓库中的 `DemoLepViewer` 、 `DemoTinyLep` 里的做法。利用 Web Components 封装，开发者只需像这样书写即可在网页展示压缩图：

```html
<lep-img src="images/vacation.lep"></lep-img>
```

---

## Lepton 实用工具全家桶

为了让 Lepton 能够既要压缩后的小体积又要如未压缩JPG一样的浏览、预览体验，可以下载如下三件套：

1. **[TinyLep](https://github.com/2010HCY/TinyLep)**: JPG 批量无损压缩工具，拖入文件夹即可完成瘦身。
2. **[LepViewer](https://github.com/2010HCY/LepViewer)**：双击即可像普通图片一样即时预览 `.lep` 文件，无需手动解压再打开查看。
3. **[LepThumb](https://github.com/2010HCY/LepViewer/tree/main/LepThumb)**：Windows 资源管理器缩略图插件，直接在文件夹中预览 `.lep` 缩略图
4. **LeptonWASM**: 即本项目，让网站或 Web 应用原生支持预览和处理 Lepton 格式。

---

## 致谢

本项目使用了Dropbox Lepton。特别感谢 **Microsoft 团队** 开源了对 Lepton 工具的 Rust 移植与重构 [lepton_jpeg_rust](https://github.com/microsoft/lepton_jpeg_rust)

## 赞赏

如果这些工具对你有帮助，欢迎打赏支持：

![PayQrcode](./images/PayQrcode.jpg)