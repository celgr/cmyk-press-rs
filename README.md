# CMYK Press

CMYK Pressは、印刷っぽい表現（カラーハーフトーンや版ズレ）ができるRust製のAfterEffectsプラグインです。

※開発途中のリポジトリです。

## ビルド

### macOS

必要な環境は以下のとおりです。

- Xcode Command Line Tools（`clang`、`codesign`、`PlistBuddy` を使用します）
- Rust stable toolchain（`cargo` を使用します）
- GitHub の git 依存関係を取得できるネットワーク環境

以下のコマンドを実行します。

```bash
bash ./scripts/build_macos_release.sh
```

ビルドされたプラグインは以下に出力されます。

```text
rust/target/release/CMYK Press.plugin
```

### Windows

必要な環境は以下のとおりです。

- Windows 10 / 11 x64
- Rust MSVC toolchain（`cargo` と `rustc` を使用します）
- Visual Studio 2022 Build Tools（`Microsoft.VisualStudio.Workload.VCTools` と `link.exe` を使用します）

`winget` が使える場合は、以下のコマンドでインストールできます。

```powershell
winget install --id Rustlang.Rust.MSVC --exact
winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --override "--wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

インストール後、新しい PowerShell または「x64 Native Tools Command Prompt for VS 2022」を開き、以下のコマンドを実行します。

```powershell
.\scripts\build_windows_release.ps1
```

PowerShell の実行ポリシーで止まる場合は、以下のコマンドで実行します。

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\build_windows_release.ps1
```

`link.exe` が見つからない場合は、Visual Studio の C++ Build Tools が入っていないか、MSVC の開発者環境が読み込まれていません。「x64 Native Tools Command Prompt for VS 2022」から再実行してください。

ビルドされたプラグインは以下に出力されます。

```text
rust/target/release/CMYK Press.aex
```

## テスト

After Effects を起動せずに実行できる最小限の検証方法は、以下のとおりです。

```bash
cd rust
AESDK_ROOT= cargo test
```
