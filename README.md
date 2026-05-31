# CMYK Press

CMYK Press は、CMYK 印刷風の色分解、固定された版ズレ、円形の網点、紙色、および元画像とのブレンドを実現する Rust 製の After Effects ネイティブエフェクトです。

- モード: `Default CMYK Dots`
- 合成方法: `Composite`
- 網点処理: 有効
- 網点形状: 円形
- 固定ランダム版ズレ: 無効
- 品質: フル品質

## エフェクト情報

- 表示名: `CMYK Press`
- Match Name: `CMYK Press`
- カテゴリ: `Stylize`
- macOS 版の出力先: `rust/target/release/CMYK Press.plugin`

## ビルド

macOS では、以下のコマンドを実行します。

```bash
bash ./scripts/build_macos_release.sh
```

ビルドされたプラグインは、以下の場所に出力されます。

```text
rust/target/release/CMYK Press.plugin
```

Windows では、Visual Studio Build Tools と Rust MSVC toolchain のある PowerShell で以下を実行します。

```powershell
.\scripts\build_windows_release.ps1
```

ビルドされた `.aex` は、以下の場所に出力されます。

```text
rust/target/release/CMYK Press.aex
```

## テスト

After Effects を起動せずに実行できる最小限の検証方法は、以下のとおりです。

```bash
cd rust
AESDK_ROOT= cargo test
```
