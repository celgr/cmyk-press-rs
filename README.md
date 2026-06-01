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

Windows では、以下の環境が必要です。

- Windows 10 / 11 x64
- Rust MSVC toolchain
  - `cargo` と `rustc` が使えること
- Visual Studio 2022 Build Tools
  - `Microsoft.VisualStudio.Workload.VCTools` を含めること
  - MSVC リンカ `link.exe` が使えること

`winget` が使える環境では、以下のようにインストールできます。

```powershell
winget install --id Rustlang.Rust.MSVC --exact
winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --override "--wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

インストール後、新しい PowerShell または「x64 Native Tools Command Prompt for VS 2022」を開いて、以下を実行します。

```powershell
.\scripts\build_windows_release.ps1
```

PowerShell の実行ポリシーでスクリプトが止まる場合は、以下のように実行します。

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\build_windows_release.ps1
```

`link.exe` が見つからない場合は、Visual Studio の C++ Build Tools が入っていないか、MSVC の開発者環境が読み込まれていません。「x64 Native Tools Command Prompt for VS 2022」から再実行してください。

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
