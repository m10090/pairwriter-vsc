import os

targets = {
    "aarch64-apple-darwin": "darwin-arm64",
    "x86_64-apple-darwin": "darwin-x64",
    "x86_64-pc-windows-msvc": "win32-x64-msvc",  # windows is not compiling
    "x86_64-unknown-linux-gnu": "linux-x64-gnu",
    "i686-unknown-linux-gnu": "linux-arm64-gnu",
}


def compile():
    # windows x64
    if os.name == "nt":
        target = "x86_64-pc-windows-msvc"
        os.system(f"npm run cross-build -- --target {target}")
        platform = targets[target]
        os.system(f"mv index.node ./platforms/{platform}/index.node")

    # macos x64
    if os.name == "posix" and os.uname().sysname == "Darwin":
        for target in ["x86_64-apple-darwin", "aarch64-apple-darwin"]:
            os.system(f"npm run cross-build -- --target {target}")
            platform = targets[target]
            os.system(f"mv index.node ./platforms/{platform}/index.node")
    # linux x64
    if os.name == "posix" and os.uname().sysname == "Linux":
        for target in ["x86_64-unknown-linux-gnu", "i686-unknown-linux-gnu"]:
            os.system(f"npm run cross-build -- --target {target}")
            platform = targets[target]
            os.system(f"mv index.node ./platforms/{platform}/index.node")


compile()
