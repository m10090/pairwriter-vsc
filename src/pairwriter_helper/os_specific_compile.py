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
        os.system(f"cargo build --target {target}  --message-format=json -r> cargo.log")
        os.system("npm run postcargo-build")
        platform = targets[target]
        os.system(f"mv index.node ./platforms/{platform}/index.node")
        return

    # macos x64

    if os.uname().sysname == "Darwin":
        for target in ["x86_64-apple-darwin", "aarch64-apple-darwin"]:
            os.system(
                f"cargo build --target {target}  --message-format=json -r> cargo.log"
            )
            os.system("npm run postcargo-build")
            platform = targets[target]
            os.system(f"mv index.node ./platforms/{platform}/index.node")
        return
    # linux x64
    for target in ["x86_64-unknown-linux-gnu", "i686-unknown-linux-gnu"]:
        os.system(
            f"cross build --target {target} --message-format=json -r> cross.log"
        )
        os.system("npm run postcross-build")
        platform = targets[target]
        os.system(f"mv index.node ./platforms/{platform}/index.node")
    return


compile()
