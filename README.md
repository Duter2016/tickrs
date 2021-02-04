# tick-rs
[![Actions Status](https://github.com/tarkah/tickrs/workflows/Test/badge.svg)](https://github.com/tarkah/tickrs/actions)

Realtime ticker data in your terminal 📈 Built with Rust. Data sourced from Yahoo! Finance.

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
Realtime ticker data in your terminal 📈

USAGE:
    tickrs [FLAGS] [OPTIONS]

FLAGS:
    -p, --enable-pre-post    Enable pre / post market hours for graphs
    -h, --help               Prints help information
        --hide-help          Hide help icon in top right
        --hide-prev-close    Hide previous close line on 1D chart
        --hide-toggle        Hide toggle block
    -x, --show-x-labels      Show x-axis labels
        --summary            Start in summary mode
        --trunc-pre          Truncate pre market graphing to only 30 minutes prior to markets opening
    -V, --version            Prints version information

OPTIONS:
    -s, --symbols <symbols>...                 Comma separated list of ticker symbols to start app with
    -t, --time-frame <time-frame>              Use specified time frame when starting program and when new stocks are
                                               added [default: 1D]  [possible values: 1D, 1W, 1M, 3M, 6M, 1Y, 5Y]
    -i, --update-interval <update-interval>    Interval to update data from API (seconds) [default: 1]
```

## Yahoo hosts

```
#Yahoo Start
98.137.11.163 yahoo.com
98.137.11.163 www.yahoo.com
87.248.118.22 finance.yahoo.com
87.248.118.22 bats.video.yahoo.com
87.248.118.22 yep.video.yahoo.com
87.248.118.22 assets.video.yahoo.net
180.222.102.158 geo.yahoo.com
69.147.88.8 query1.finance.yahoo.com
69.147.88.7 query2.finance.yahoo.com
98.137.11.144 sp.analytics.yahoo.com
54.202.68.156 streamer.finance.yahoo.com
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

### Windows

Use [Windows Terminal](https://www.microsoft.com/en-us/p/windows-terminal-preview/9n0dx20hk701) to properly display this app.

## Acknowledgments
- [fdehau](https://github.com/fdehau) / [tui-rs](https://github.com/fdehau/tui-rs) - great TUI library for Rust
- [cjbassi](https://github.com/cjbassi) / [ytop](https://github.com/cjbassi/ytop) - thanks for the inspiration!
