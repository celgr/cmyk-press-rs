# CMYK Press

CMYK Pressは、印刷っぽい表現ができるRust製のAfterEffectsプラグインです。

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
