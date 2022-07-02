# [nseproxy](https://github.com/TechfaneTechnologies/nseproxy)
Nseproxy is a rust proxy server for extracting data from National Stock Exchange (India)

## Installation
Download the [nseproxy.exe](https://github.com/TechfaneTechnologies/nseproxy/release/latest/download/nseproxy.exe) from release section

or install it with rust via `cargo install --git https://github.com/TechfaneTechnologies/nseproxy` 

## Usage
Duble Click the `nseproxy.exe` or run the `nseproxy.exe` from Cmd/PowerShell/Bash/Zsh/Terminal Etc. It will run a local proxy server at port 3000. You can access the same with `http://127.0.0.1:3000/` or `http://localhost:3000/`.

Now to use it effectively as nseproxy, replace the URL `https://www.nseindia.com` with `http://127.0.0.1:3000/` or `http://localhost:3000/`.

## Example
To Access marketStatus we were supposed to fetch with `https://www.nseindia.com/api/marketStatus` Now with this proxy tool you can use `http://127.0.0.1:3000/api/marketStatus` to access the same data.

Same goes with any other Api call, For Example to get chart data of NIFTYBANK use this URL `http://127.0.0.1:3000/api/chart-databyindex?index=NIFTY%20BANK&indices=true&preopen=true`

