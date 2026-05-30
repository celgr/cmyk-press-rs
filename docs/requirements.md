# CMYK Press for After Effects — 要件定義・実装仕様書

- **Document status:** Draft v0.2
- **Target:** After Effects Effect Plug-in
- **Primary implementation language:** Rust
- **Host integration:** Adobe After Effects C++ SDK + thin C/C++ shim
- **Initial platform priority:** macOS / Apple Silicon
- **Secondary platform:** Windows
- **Last updated:** 2026-05-30

---

## 0. このドキュメントの目的

After Effects上で、CMYK分版、版ズレ、網点、紙色を利用した印刷表現を行うエフェクト **CMYK Press** を開発する。

本ドキュメントは、実装時に迷わないための要件定義、UI仕様、レンダリング仕様、Rust構成、GPU構成、受け入れ条件、将来拡張をまとめたものとする。

---

## 1. プロダクト方針

### 1.1 目標

AEへ適用した直後から、**CMYKの円形ドット網点が見える印刷表現**を作れるエフェクトにする。

初期状態では、以下を自動的に有効にする。

```text
Halftone Enable = On
Dot Shape = Circle
View = Composite
Random Registration = Off
```

つまり、エフェクトを適用しただけで「通常のRGB画像」ではなく、**CMYKドット印刷風の見た目**になる。

### 1.2 主要用途

- 雑誌、新聞、ポスター風の印刷表現
- CMYKドットのモーショングラフィックス
- リソグラフ風表現
- 版ズレ表現
- MV、リリックビデオ、広告映像
- IllustratorのCMYK表現に近いルックのAE内プレビュー

### 1.3 AE内部での扱い

After Effects内部ではRGB画像として処理する。

```text
AE RGB Input
  ↓
RGB → CMYK相当値
  ↓
C / M / Y / K各版を加工
  ↓
版ズレ
  ↓
各版を円形ドット網点化
  ↓
紙色の上へ再合成
  ↓
AE RGB Output
```

---

## 2. デフォルト動作

## 2.1 初回適用時の見た目

エフェクト適用直後は、以下の状態とする。

| 項目 | 初期値 |
|---|---:|
| View | `Composite` |
| Cyan | `100%` |
| Magenta | `100%` |
| Yellow | `100%` |
| Black | `100%` |
| Paper Color | `White` |
| Random Registration | `Off` |
| Halftone | `On` |
| Dot Shape | `Circle` |
| Dot Size | `8 px` |
| Dot Gain | `0` |
| Softness | `0.1` |
| C Angle | `15°` |
| M Angle | `75°` |
| Y Angle | `0°` |
| K Angle | `45°` |
| Rendering Backend | `Auto` |
| Quality | `Full` |

### 2.2 デフォルトプリセット名

```text
Default CMYK Dots
```

### 2.3 重要な仕様

- 網点は初期状態で有効
- 網点形状は円形ドットが初期値
- 線、四角、菱形は切り替え可能
- 版ズレは初期状態では無効
- 版ズレはユーザーが必要に応じて有効化
- 版ズレのランダム値は時間変化しない

---

## 3. MVP機能一覧

## 3.1 CMYK変換

- RGB → CMYK相当値
- CMYK → RGB再合成
- C / M / Y / K個別調整
- 黒生成
- UCR
- 紙色
- 元画像とのブレンド
- アルファ保持

## 3.2 網点

- デフォルトで有効
- 円形ドットがデフォルト
- C / M / Y / K各版を個別に網点化
- 各版の角度を個別指定
- Size
- Dot Gain
- Softness
- Offset
- Circle / Square / Line / Diamond
- GPU対応
- CPU対応
- Draft / Full品質

## 3.3 版ズレ

- 手動版ズレ
- 固定ランダム版ズレ
- Seed
- Amount X / Y
- C / M / Y / K個別ON/OFF
- 時間変化なし
- CPU / GPU一致
- MFR対応

## 3.4 GPU

- GPU Smart Render
- GPU Device Setup / Setdown
- GPU失敗時CPUフォールバック
- macOS対応
- Windows対応
- CPU / GPU差分テスト

---

## 4. UI仕様

```text
CMYK Press
├── Preset
│   └── Default CMYK Dots
│
├── Conversion
│   ├── Mode
│   ├── View
│   ├── Preserve Alpha
│   └── Blend With Original
│
├── Ink Amount
│   ├── Cyan
│   ├── Magenta
│   ├── Yellow
│   └── Black
│
├── Paper
│   ├── Color
│   └── Brightness
│
├── Halftone
│   ├── Enable
│   ├── Dot Shape
│   ├── Size
│   ├── Unit
│   ├── Dot Gain
│   ├── Softness
│   ├── Cyan Angle
│   ├── Magenta Angle
│   ├── Yellow Angle
│   ├── Black Angle
│   ├── Offset X
│   └── Offset Y
│
├── Registration Offset
│   ├── Cyan Offset
│   ├── Magenta Offset
│   ├── Yellow Offset
│   └── Black Offset
│
├── Random Registration
│   ├── Enable
│   ├── Seed
│   ├── Amount X
│   ├── Amount Y
│   ├── Affect Cyan
│   ├── Affect Magenta
│   ├── Affect Yellow
│   └── Affect Black
│
└── Rendering
    ├── Backend
    ├── Quality
    ├── Edge Mode
    └── Expand Bounds
```

---

## 5. パラメータ仕様

## 5.1 Preset

| ID | UI名 | 型 | 初期値 | 選択肢 |
|---|---|---:|---:|---|
| `preset` | Preset | enum | `Default CMYK Dots` | `Default CMYK Dots`, `Clean CMYK`, `Newspaper`, `Risograph`, `Custom` |

MVPでは `Default CMYK Dots` と `Custom` のみでもよい。

---

## 5.2 Conversion

| ID | UI名 | 型 | 初期値 | 範囲・選択肢 |
|---|---|---:|---:|---|
| `conversion_mode` | Mode | enum | `Simple` | `Simple`, `ICC Soft Proof` |
| `view_mode` | View | enum | `Composite` | `Composite`, `Cyan`, `Magenta`, `Yellow`, `Black`, `Ink Coverage`, `Original / Result Split` |
| `preserve_alpha` | Preserve Alpha | bool | `true` | `true`, `false` |
| `blend_original` | Blend With Original | float | `0.0` | `0.0..1.0` |

MVPでは `Simple` のみ必須。`ICC Soft Proof` は将来機能。

---

## 5.3 Ink Amount

| ID | UI名 | 型 | 初期値 | 範囲 |
|---|---|---:|---:|---|
| `cyan_amount` | Cyan | float | `1.0` | `0.0..2.0` |
| `magenta_amount` | Magenta | float | `1.0` | `0.0..2.0` |
| `yellow_amount` | Yellow | float | `1.0` | `0.0..2.0` |
| `black_amount` | Black | float | `1.0` | `0.0..2.0` |

---

## 5.6 Paper

| ID | UI名 | 型 | 初期値 | 範囲 |
|---|---|---:|---:|---|
| `paper_color` | Color | RGB | `(1.0, 1.0, 1.0)` | `0.0..1.0` |
| `paper_brightness` | Brightness | float | `1.0` | `0.0..2.0` |

---

## 5.7 Halftone

### デフォルト設定

| ID | UI名 | 型 | 初期値 | 範囲・選択肢 |
|---|---|---:|---:|---|
| `halftone_enabled` | Enable | bool | `true` | `true`, `false` |
| `halftone_shape` | Dot Shape | enum | `Circle` | `Circle`, `Square`, `Line`, `Diamond` |
| `halftone_size` | Size | float | `8.0` | `1.0..1000.0` |
| `halftone_unit` | Unit | enum | `Pixels` | `Pixels`, `Lines Per Inch` |
| `halftone_dot_gain` | Dot Gain | float | `0.0` | `-1.0..1.0` |
| `halftone_softness` | Softness | float | `0.1` | `0.0..1.0` |
| `halftone_c_angle` | Cyan Angle | float | `15°` | `0..180°` |
| `halftone_m_angle` | Magenta Angle | float | `75°` | `0..180°` |
| `halftone_y_angle` | Yellow Angle | float | `0°` | `0..180°` |
| `halftone_k_angle` | Black Angle | float | `45°` | `0..180°` |
| `halftone_offset_x` | Offset X | float | `0.0` | 任意 |
| `halftone_offset_y` | Offset Y | float | `0.0` | 任意 |

### 必須仕様

- `halftone_enabled = true` を初期値とする
- `halftone_shape = Circle` を初期値とする
- 円形ドットを基本表現とする
- C / M / Y / K各版へ個別に網点化する
- 各版に異なる角度を設定できる
- 版ズレ後の座標を使って網点を評価する
- GPU処理対象とする
- CPU処理でも同等の見た目を出す
- Dot Gainでドットの太りを調整する
- Softnessで境界の滑らかさを調整する
- プレビュー縮小時のちらつきを抑える

### UI上の名称

`Dot Shape` の先頭項目は、ユーザー向けには以下のように表示してもよい。

```text
Dot
Square
Line
Diamond
```

内部enumは以下とする。

```text
Circle = 0
Square = 1
Line = 2
Diamond = 3
```

---

## 5.8 Registration Offset

| ID | UI名 | 型 | 初期値 | 範囲 |
|---|---|---:|---:|---|
| `cyan_offset` | Cyan Offset | point | `(0.0, 0.0)` | `-100..100 px` |
| `magenta_offset` | Magenta Offset | point | `(0.0, 0.0)` | `-100..100 px` |
| `yellow_offset` | Yellow Offset | point | `(0.0, 0.0)` | `-100..100 px` |
| `black_offset` | Black Offset | point | `(0.0, 0.0)` | `-100..100 px` |

サブピクセル対応。

---

## 5.9 Fixed Random Registration

| ID | UI名 | 型 | 初期値 | 範囲 |
|---|---|---:|---:|---|
| `random_registration_enabled` | Enable | bool | `false` | `true`, `false` |
| `random_seed` | Seed | int | `0` | `0..2147483647` |
| `random_amount_x` | Amount X | float | `3.0` | `0.0..100.0 px` |
| `random_amount_y` | Amount Y | float | `3.0` | `0.0..100.0 px` |
| `random_affect_cyan` | Affect Cyan | bool | `true` | `true`, `false` |
| `random_affect_magenta` | Affect Magenta | bool | `true` | `true`, `false` |
| `random_affect_yellow` | Affect Yellow | bool | `true` | `true`, `false` |
| `random_affect_black` | Affect Black | bool | `false` | `true`, `false` |

### 必須仕様

```text
Random Offset = hash(Seed, Plate ID, Axis ID)
```

次の値へ依存してはならない。

```text
Current Time
Frame Number
FPS
Thread ID
Render Order
System Clock
```

---

## 5.10 Rendering

| ID | UI名 | 型 | 初期値 | 選択肢 |
|---|---|---:|---:|---|
| `render_backend` | Backend | enum | `Auto` | `Auto`, `CPU`, `GPU` |
| `quality` | Quality | enum | `Full` | `Draft`, `Full` |
| `edge_mode` | Edge Mode | enum | `Transparent` | `Transparent`, `Clamp Edge` |
| `expand_bounds` | Expand Bounds | bool | `true` | `true`, `false` |

---

## 6. レンダリング順序

処理順序は以下で固定する。

```text
1. AE RGB入力
2. アンプリマルチプライ
3. 各版用サンプリング座標を計算
4. 手動版ズレを適用
5. 固定ランダム版ズレを適用
6. 各版用RGBをサンプリング
7. RGB → CMYK
8. 黒生成
9. UCR
10. C / M / Y / K量調整
11. Total Ink Limit
12. 各版を網点化
13. 紙色の上へCMYKインクを再合成
14. View Modeを適用
15. 元画像とのブレンド
16. アルファを適用
17. 再プリマルチプライ
18. AE RGB出力
```

### 重要

網点化は版ズレの後に行う。

```text
Registration Offset
  ↓
Halftone
```

これにより、実際に版がずれたような見た目になる。

---

## 7. 色変換仕様

## 7.1 Simple RGB → CMYK

RGBは `0.0..1.0` に正規化する。

```text
K = 1 - max(R, G, B)

C_raw = (1 - R - K) / max(1 - K, epsilon)
M_raw = (1 - G - K) / max(1 - K, epsilon)
Y_raw = (1 - B - K) / max(1 - K, epsilon)
```

完全な黒の場合:

```text
if K >= 1 - epsilon:
  C = 0
  M = 0
  Y = 0
```

---

## 7.2 Total Ink Limit

```text
total = C + M + Y + K
```

### Preserve K

```text
remaining = max(total_ink_limit - K, 0)
scale = remaining / max(C + M + Y, epsilon)

C *= scale
M *= scale
Y *= scale
```

### Scale All

```text
scale = total_ink_limit / total

C *= scale
M *= scale
Y *= scale
K *= scale
```

### Soft Clip

滑らかな圧縮式を利用する。CPUとGPUで同じ式を利用する。

---

## 8. 網点アルゴリズム

## 8.1 基本処理

各版ごとに、ピクセル座標を角度に応じて回転する。

```text
rotated_x =  cos(angle) * x + sin(angle) * y
rotated_y = -sin(angle) * x + cos(angle) * y
```

セル位置:

```text
cell = fract(rotated_position / frequency)
```

セル中心からの距離:

```text
distance = length(cell - 0.5)
```

インク量に応じて円形ドット半径を変える。

```text
radius = ink_to_radius(ink_amount, dot_gain)
coverage = smoothstep(radius + softness, radius - softness, distance)
```

## 8.2 デフォルト形状: Dot

ユーザー向けには `Dot` と表示する。

内部では円形として扱う。

```text
Dot = Circle
```

距離関数:

```text
distance = length(cell - 0.5)
```

## 8.3 Square

```text
distance = max(abs(cell.x - 0.5), abs(cell.y - 0.5))
```

## 8.4 Line

```text
distance = abs(cell.y - 0.5)
```

## 8.5 Diamond

```text
distance = abs(cell.x - 0.5) + abs(cell.y - 0.5)
```

## 8.6 アンチエイリアス

MVPでは以下を使う。

- `smoothstep`
- サブピクセル座標
- Softness
- Draft / Full品質
- 高周波抑制

将来候補:

- デリバティブベースAA
- supersampling
- ミップマップ
- 解像度連動
- モアレ抑制
- temporal stability補正

---

## 9. 固定ランダム版ズレ

## 9.1 仕様

版ズレのランダム値は時間で変化しない。

```rust
pub fn hash_u32(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^= value >> 16;
    value
}

pub fn random_signed(seed: u32, plate_id: u32, axis_id: u32) -> f32 {
    let h = hash_u32(seed ^ (plate_id * 31) ^ axis_id);
    let normalized = h as f32 / u32::MAX as f32;
    normalized * 2.0 - 1.0
}
```

### Plate ID

```text
C = 1
M = 2
Y = 3
K = 4
```

### Offset

```text
offset_x = random_signed(seed, plate_id, 0) * amount_x
offset_y = random_signed(seed, plate_id, 1) * amount_y
```

---

## 10. Rustアーキテクチャ

## 10.1 方針

Adobe Effects SDKとの接続は薄いC++ shimへ閉じ込める。

```text
After Effects
  ↓
Adobe Effects SDK
  ↓
Thin C++ Shim
  ↓ C ABI
Rust Core
  ├── cmyk-math
  ├── cmyk-cpu
  ├── cmyk-gpu
  └── cmyk-cli
```

## 10.2 ディレクトリ構成

```text
cmyk-press/
├── Cargo.toml
├── README.md
├── docs/
│   └── requirements.md
│
├── crates/
│   ├── cmyk-math/
│   │   └── src/
│   │       ├── color.rs
│   │       ├── ink.rs
│   │       ├── registration.rs
│   │       ├── halftone.rs
│   │       └── hash.rs
│   │
│   ├── cmyk-cpu/
│   │   └── src/
│   │       ├── render.rs
│   │       └── sampling.rs
│   │
│   ├── cmyk-gpu/
│   │   ├── src/
│   │   │   ├── backend.rs
│   │   │   ├── params.rs
│   │   │   ├── macos.rs
│   │   │   └── windows.rs
│   │   └── shaders/
│   │       ├── cmyk_press_mac.*
│   │       ├── cmyk_press_dx.hlsl
│   │       └── cmyk_press_cuda.cu
│   │
│   ├── ae-plugin/
│   │   └── src/
│   │       ├── entry.rs
│   │       ├── params.rs
│   │       ├── cpu_render.rs
│   │       ├── gpu_render.rs
│   │       ├── errors.rs
│   │       └── ffi.rs
│   │
│   └── cmyk-cli/
│       └── src/main.rs
│
├── cpp-shim/
│   ├── include/
│   │   └── cmyk_press_bridge.h
│   └── src/
│       └── cmyk_press_bridge.cpp
│
├── tests/
│   ├── fixtures/
│   ├── golden-images/
│   └── integration/
│
└── scripts/
    ├── install-macos.sh
    ├── install-windows.ps1
    └── compare-renders.py
```

---

## 11. 共通パラメータ構造体

```rust
#[repr(C)]
pub struct CmykPressParams {
    pub cyan_amount: f32,
    pub magenta_amount: f32,
    pub yellow_amount: f32,
    pub black_amount: f32,

    pub total_ink_limit: f32,

    pub cyan_offset: [f32; 2],
    pub magenta_offset: [f32; 2],
    pub yellow_offset: [f32; 2],
    pub black_offset: [f32; 2],

    pub random_registration_enabled: u32,
    pub random_seed: u32,
    pub random_amount: [f32; 2],
    pub random_plate_mask: u32,

    pub halftone_enabled: u32,
    pub halftone_frequency: f32,
    pub halftone_shape: u32,
    pub halftone_dot_gain: f32,
    pub halftone_softness: f32,
    pub halftone_angles: [f32; 4],
    pub halftone_offset: [f32; 2],

    pub paper_color: [f32; 3],
    pub paper_brightness: f32,
    pub paper_tint: f32,

    pub preserve_alpha: u32,
    pub view_mode: u32,
    pub quality: u32,
}
```

### デフォルト値

```rust
impl Default for CmykPressParams {
    fn default() -> Self {
        Self {
            cyan_amount: 0.85,
            magenta_amount: 0.85,
            yellow_amount: 0.85,
            black_amount: 0.90,

            total_ink_limit: 2.40,

            cyan_offset: [0.0, 0.0],
            magenta_offset: [0.0, 0.0],
            yellow_offset: [0.0, 0.0],
            black_offset: [0.0, 0.0],

            random_registration_enabled: 0,
            random_seed: 0,
            random_amount: [3.0, 3.0],
            random_plate_mask: 0b0111,

            halftone_enabled: 1,
            halftone_frequency: 8.0,
            halftone_shape: 0,
            halftone_dot_gain: 0.0,
            halftone_softness: 0.1,
            halftone_angles: [15.0, 75.0, 0.0, 45.0],
            halftone_offset: [0.0, 0.0],

            paper_color: [1.0, 1.0, 1.0],
            paper_brightness: 1.0,
            paper_tint: 0.0,

            preserve_alpha: 1,
            view_mode: 0,
            quality: 1,
        }
    }
}
```

---

## 12. AE SDK統合

## 12.1 使用するセレクタ

```text
PF_Cmd_GLOBAL_SETUP
PF_Cmd_PARAM_SETUP
PF_Cmd_SEQUENCE_SETUP
PF_Cmd_SEQUENCE_SETDOWN
PF_Cmd_SMART_PRE_RENDER
PF_Cmd_SMART_RENDER
PF_Cmd_GPU_DEVICE_SETUP
PF_Cmd_GPU_DEVICE_SETDOWN
PF_Cmd_SMART_RENDER_GPU
```

## 12.2 SmartFX

```text
PF_OutFlag2_SUPPORTS_SMART_RENDER
```

## 12.3 Multi-Frame Rendering

```text
PF_OutFlag2_SUPPORTS_THREADED_RENDERING
```

### MFRルール

- レンダリング中にグローバル可変状態を持たない
- Seedは決定論的に扱う
- フレーム番号に依存しない
- 時刻に依存しない
- レンダリング順序に依存しない
- Thread IDに依存しない
- sequence_dataへの書き込みを避ける

---

## 13. GPUレンダリング

## 13.1 GPU対象機能

| 機能 | MVP GPU対応 |
|---|---|
| RGB → CMYK | 必須 |
| CMYK → RGB | 必須 |
| 黒生成 | 必須 |
| UCR | 必須 |
| Total Ink Limit | 必須 |
| 紙色 | 必須 |
| 手動版ズレ | 必須 |
| 固定ランダム版ズレ | 必須 |
| 円形ドット網点 | 必須 |
| Square / Line / Diamond | 必須 |
| 分版プレビュー | 必須 |
| ICC | CPUフォールバック可 |
| 紙テクスチャ | 将来 |
| インクにじみ | 将来 |

## 13.2 GPUフォールバック

```text
GPU Available
  ├── Yes → GPU Smart Render
  └── No  → CPU Smart Render
```

## 13.3 バックエンド

```text
macOS
└── PoCでAE GPU SDKサンプルを確認し採用方式を確定

Windows
├── DirectX / HLSL
└── CUDA Driver APIは必要性を評価
```

---

## 14. CPUレンダリング

### 役割

- GPU非対応時のフォールバック
- CLI
- ゴールデン画像生成
- GPU出力比較
- ICC Soft Proof
- デバッグ

### 最適化

- 内部は `f32`
- ピクセルループ内でメモリアロケーションしない
- 網点なしの高速パス
- 版ズレなしの高速パス
- SIMD化を検討
- Rayon導入は計測後に判断

---

## 15. アルファ仕様

```text
Input Premultiplied RGBA
  ↓
Unpremultiply
  ↓
CMYK / Registration / Halftone
  ↓
Apply Alpha
  ↓
Premultiply
  ↓
Output
```

### 必須条件

- 透明PNGで黒縁が出ない
- 半透明境界で色が破綻しない
- 版ズレではみ出した領域を描画できる
- `Expand Bounds` 対応
- `Transparent` と `Clamp Edge` を切り替え可能

---

## 16. Quality

| モード | 用途 | 内容 |
|---|---|---|
| `Draft` | 編集プレビュー | 高周波抑制、簡易AA |
| `Full` | 最終レンダリング | サブピクセル、正確な角度、フル品質AA |

将来候補:

```text
Auto
Supersampled
Print Preview
```

---

## 17. CLI

```bash
cargo run -p cmyk-cli -- \
  input.png \
  output.png \
  --halftone \
  --shape dot \
  --frequency 8 \
  --cyan-angle 15 \
  --magenta-angle 75 \
  --yellow-angle 0 \
  --black-angle 45
```

### CLIのデフォルト

CLIもAEと同様に、網点を初期状態で有効にする。

```text
halftone = true
shape = dot
```

網点を無効化する場合:

```bash
--no-halftone
```

---

## 18. テスト要件

## 18.1 デフォルト動作テスト

| ID | 条件 |
|---|---|
| TEST-DEFAULT-01 | エフェクト適用直後に網点が表示される |
| TEST-DEFAULT-02 | Dot ShapeがDotになっている |
| TEST-DEFAULT-03 | C / M / Y / Kに異なる角度が適用される |
| TEST-DEFAULT-04 | Composite表示になっている |
| TEST-DEFAULT-05 | Random RegistrationはOffになっている |
| TEST-DEFAULT-06 | GPU有効時にドット網点がGPU処理される |
| TEST-DEFAULT-07 | CPUとGPUのドット位置が概ね一致する |

## 18.2 網点テスト

- Dot
- Square
- Line
- Diamond
- Frequency
- Dot Gain
- Softness
- Angle
- Offset
- 透明画像
- 4K
- Draft
- Full
- GPU
- CPU

## 18.3 版ズレテスト

- Seed固定
- Seed変更
- フレーム番号非依存
- CPU / GPU一致
- K版初期対象外
- 透明境界

---

## 19. 受け入れ条件

| ID | 条件 |
|---|---|
| AC-01 | AEのエフェクト一覧から適用できる |
| AC-02 | 適用直後から円形ドット網点が表示される |
| AC-03 | 初期Dot ShapeがDotになっている |
| AC-04 | C / M / Y / Kの網点角度が個別設定されている |
| AC-05 | 網点をOFFにするとクリーンなCMYK再合成になる |
| AC-06 | Dot / Square / Line / Diamondを切り替えられる |
| AC-07 | Frequencyを変更するとドットサイズが変わる |
| AC-08 | Dot Gainを変更するとドットが太るまたは細くなる |
| AC-09 | Softnessを変更するとドット境界が変わる |
| AC-10 | C / M / Y / K量を変更できる |
| AC-11 | 黒生成量を変更できる |
| AC-12 | Total Ink Limitを変更できる |
| AC-13 | 紙色を変更できる |
| AC-14 | 手動版ズレを各版へ適用できる |
| AC-15 | 固定ランダム版ズレをSeedで切り替えられる |
| AC-16 | 同じSeedでフレームが変わっても版ズレが動かない |
| AC-17 | GPUで網点と版ズレを処理できる |
| AC-18 | GPU利用不可時にCPUへフォールバックする |
| AC-19 | 透明画像で黒縁が出ない |
| AC-20 | 8 / 16 / 32-bpcで動作する |
| AC-21 | MFR有効時に結果が変わらない |
| AC-22 | 4K素材でクラッシュしない |
| AC-23 | CLIでもAEと同じデフォルト網点を出せる |
| AC-24 | CPU / GPUの見た目差が許容範囲内である |

---

## 20. 実装フェーズ

## Phase 0 — PoC

```text
[ ] AE SDKサンプルをビルド
[ ] C++ shimからRustを呼ぶ
[ ] RGB passthrough
[ ] Slider 1つ
[ ] SmartFX CPU
[ ] GPUサンプル
[ ] macOS GPU方式を確認
```

## Phase 1 — デフォルトドットCPU版

```text
[ ] RGB → CMYK → RGB
[ ] 紙色
[ ] 黒生成
[ ] UCR
[ ] Total Ink Limit
[ ] Dot網点
[ ] デフォルトでHalftone On
[ ] デフォルトでDot Shape = Dot
[ ] C / M / Y / K角度
[ ] Frequency
[ ] Dot Gain
[ ] Softness
[ ] CLI
[ ] ゴールデン画像
```

## Phase 2 — 版ズレCPU版

```text
[ ] 手動版ズレ
[ ] 固定ランダム版ズレ
[ ] Seed
[ ] C / M / Y / K個別ON/OFF
[ ] 透明境界
[ ] Expand Bounds
```

## Phase 3 — GPU版

```text
[ ] GPU device setup
[ ] GPU smart render
[ ] Dot網点
[ ] Square / Line / Diamond
[ ] 手動版ズレ
[ ] 固定ランダム版ズレ
[ ] CPU / GPU差分比較
[ ] CPUフォールバック
```

## Phase 4 — AE最適化

```text
[ ] 8-bpc
[ ] 16-bpc
[ ] 32-bpc Float
[ ] MFR
[ ] ROI
[ ] Draft / Full
[ ] 4K
[ ] 8K
```

## Phase 5 — 将来拡張

```text
[ ] ICC Soft Proof
[ ] 紙テクスチャ
[ ] インクにじみ
[ ] インクかすれ
[ ] エッジ荒れ
[ ] プリセット
[ ] 特色インク
[ ] 分版レイヤー生成
```

---

## 21. 将来機能候補

## 21.1 網点

- Ellipse
- Cross
- Custom Texture
- FMスクリーン風
- 版ごとのFrequency
- 版ごとのDot Gain
- 版ごとのSoftness
- LPI / DPI連動
- Supersampling
- モアレ抑制
- プレビュー解像度連動
- カメラスケール変化への安定化

## 21.2 印刷質感

- Paper Texture
- 紙繊維
- 黄ばみ
- Ink Bleed
- Ink Spread
- Ink Blur
- Ink Dryness
- インクかすれ
- 網点欠け
- コピー機ノイズ
- スキャンノイズ
- トナー粒子

## 21.3 版ズレ

- 版ごとの回転
- 版ごとのスケール
- 版ごとのブラー
- 局所歪み
- 時間固定のランダム歪みマップ
- マスクによる局所版ズレ

## 21.4 カラー管理

- ICCファイル読み込み
- RGB入力プロファイル
- CMYK出力プロファイル
- Rendering Intent
- Black Point Compensation
- Simulate Paper Color
- Simulate Black Ink
- Japan Color
- SWOP
- FOGRA
- Little CMS

## 21.5 ワークフロー

- Default CMYK Dotsプリセット
- Clean CMYKプリセット
- Newspaperプリセット
- Risographプリセット
- Copy Machineプリセット
- 2色印刷
- 3色印刷
- 特色インク
- Export Plates
- C / M / Y / Kレイヤー自動生成
- AEGPメニュー
- スクリプト連携
- Premiere Pro対応

---

## 22. 実装時の優先チェックリスト

```text
[ ] Halftoneの初期値をtrueにする
[ ] Dot Shapeの初期値をDot / Circleにする
[ ] 初回適用だけでCMYKドットが表示されることを確認
[ ] CLIもデフォルトでDot網点を表示
[ ] 網点OFF時にClean CMYK表示へ戻る
[ ] C / M / Y / Kの角度を別々に保持
[ ] CPUとGPUの距離関数を共通仕様にする
[ ] Dot GainとSoftnessをCPU / GPUで揃える
[ ] Seedを時間へ依存させない
[ ] MFRで結果が変わらないことを確認
[ ] 透明画像の境界を確認
[ ] 4KでGPU動作を確認
```

---

## 23. 未確定事項

| ID | 項目 | 方針 |
|---|---|---|
| TBD-01 | macOS GPUバックエンド | AE SDK GPUサンプル確認後に確定 |
| TBD-02 | Windows GPU優先順位 | DirectX / HLSLを第一候補 |
| TBD-03 | Frequency初期値 | `8 px`を仮採用。視認性を確認して調整 |
| TBD-04 | 網点AA | smoothstepで不足する場合は解析的AAまたはsupersampling |
| TBD-05 | LPI | AE上のDPI概念が曖昧なためPixelsを標準とする |
| TBD-06 | Expand Bounds | SmartFX ROIと合わせてPoCで確認 |
| TBD-07 | ICCキャッシュ | AE Compute CacheとRustキャッシュを比較 |

---

## 24. 参考資料

### Adobe After Effects C++ SDK Guide

- SmartFX  
  https://ae-plugins.docsforadobe.dev/smartfx/smartfx/

- Command Selectors  
  https://ae-plugins.docsforadobe.dev/effect-basics/command-selectors/

- Building GPU Effects  
  https://ae-plugins.docsforadobe.dev/intro/gpu-build-instructions/

- Multi-Frame Rendering in AE  
  https://ae-plugins.docsforadobe.dev/effect-details/multi-frame-rendering-in-ae/

- Parameters  
  https://ae-plugins.docsforadobe.dev/effect-basics/parameters/

### ICC / Color Management

- Little CMS  
  https://www.littlecms.com/

- Rust `lcms2` crate  
  https://crates.io/crates/lcms2

---

## 25. 変更履歴

| Version | Date | 内容 |
|---|---|---|
| 0.1 | 2026-05-30 | Rust、GPU、固定ランダム版ズレ、網点、将来拡張を含む初版 |
| 0.2 | 2026-05-30 | 網点をデフォルト有効化。Dot / Circleを初期形状へ変更。初回適用時からCMYKドットが表示される仕様へ変更 |
