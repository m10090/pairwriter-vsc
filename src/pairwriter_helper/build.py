import os

targets = {
    "aarch64-apple-darwin": "darwin-arm64",
    "x86_64-apple-darwin": "darwin-x64",
    "x86_64-pc-windows-msvc": "win32-x64-msvc",  # windows is not compiling
    "x86_64-unknown-linux-gnu": "linux-x64-gnu",
    "i686-unknown-linux-gnu": "linux-arm64-gnu",
}
# you could need to create windows and macos image from cross-toolchain


# add CROSS_CONFIG
os.environ["CROSS_CONFIG"] = "./crates/pairwriter_helper/cross.toml"
for target, platform in targets.items():
    print(f"Building for {target}...")
    os.system(f"npm run cross-build -- --target {target}")
    os.system(f"mv index.node ./platforms/{platform}/index.node")
