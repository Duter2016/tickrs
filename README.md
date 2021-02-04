# tick-rs
[![Actions Status](https://github.com/tarkah/tickrs/workflows/Test/badge.svg)](https://github.com/tarkah/tickrs/actions)

Realtime ticker data in your terminal 📈 Built with Rust. Data sourced from Yahoo! Finance.
终端中的实时股票行情数据收录器📈，使用Rust构建。数据来自Yahoo!金融。

  - [Installation](#installation)
  - [Usage](#usage)
    - [Windows](#windows)
  - [Acknowledgments](#acknowledgments)

<img src="./assets/demo.gif">

## Installation

### Binary

Download the latest [release](https://github.com/tarkah/tickrs/releases/latest) for your platform

### Cargo

```
cargo install tickrs
```

### AUR

```
yay -S tickrs-git
```

## Usage

```
tickrs
终端中的实时股票行情数据收录器 📈

USAGE:
    tickrs [FLAGS] [OPTIONS]

FLAGS:
    -p, --enable-pre-post    启用图表的交易前/交易后时间（pre/post market hours）
    -h, --help               打印帮助信息
        --hide-help          在右上角隐藏帮助图标
        --hide-prev-close    在一维图形上隐藏之前的收盘价
        --hide-toggle        隐藏切换块
    -x, --show-x-labels      显示 x 轴标签
        --summary            以摘要模式启动
        --trunc-pre          将开盘前图表截断至开市前仅30分钟
    -V, --version            打印版本信息

OPTIONS:
    -s, --symbols <symbols>...                 逗号分隔的股票代码符号列表，用于启动应用
    -t, --time-frame <time-frame>              使用指定的时间框架时，启动程序和当新股票[默认值: 1D]  [可选值: 1D, 1W, 1M, 3M, 6M, 1Y, 5Y]
    -i, --update-interval <update-interval>    从API更新数据的时间间隔（秒） [默认值: 1]
```

## Yahoo hosts

```
#Yahoo Start
98.137.11.163 yahoo.com
98.137.11.163 www.yahoo.com
87.248.118.22 finance.yahoo.com
87.248.118.22 streamer.finance.yahoo.com
87.248.118.22 bats.video.yahoo.com
87.248.118.22 yep.video.yahoo.com
87.248.118.22 assets.video.yahoo.net
180.222.102.158 geo.yahoo.com
69.147.88.8 query1.finance.yahoo.com
69.147.88.7 query2.finance.yahoo.com
98.137.11.144 sp.analytics.yahoo.com
98.136.103.27 udc.yahoo.com
69.147.88.7 video-api.yql.yahoo.com
87.248.118.22 yimg.com
87.248.118.22 s.yimg.com
212.102.50.49 cdn.rawgit.com
106.10.248.150 secure.yahoo.com
69.147.88.9 edge-mcdn.secure.yahoo.com
87.248.118.24 edge-mcdn-beacon.secure.yahoo.com
69.147.88.7 dns-w6wicpayn.sombrero.yahoo.net
152.195.38.41 edgecast-vod.yahoo.net
77.238.180.76 mcdn-report.wc.yahoodns.net
68.180.134.137 r-a508bfnpsnreport.wc.yahoodns.net
87.248.106.201 r-bfr1k8pnbgreport.wc.yahoodns.net
87.248.107.200 r-c1auiw5wbjreport.wc.yahoodns.net
87.248.106.201 v-a508bfnpsn.wc.yahoodns.net
119.161.10.88 v-bfr1k8pnbg.wc.yahoodns.net
209.73.179.126 v-c1auiw5wbj.wc.yahoodns.net
152.195.55.192 consent.cmp.oath.com
#Yahoo End
```
## Stickrs.desktop

下载Linux版本，解压至某个目录，然后建一个启动快捷方式。

```
[Desktop Entry]
Name=tickrs
Exec=~/opt/tickrs/tickrs 
Type=Application
Terminal=true
Comment=Yahoo股票图形看盘
Icon=~/opt/tickrs/tickrs.jpeg
```

### Windows

Use [Windows Terminal](https://www.microsoft.com/en-us/p/windows-terminal-preview/9n0dx20hk701) to properly display this app.

## Acknowledgments
- [fdehau](https://github.com/fdehau) / [tui-rs](https://github.com/fdehau/tui-rs) - great TUI library for Rust
- [cjbassi](https://github.com/cjbassi) / [ytop](https://github.com/cjbassi/ytop) - thanks for the inspiration!
