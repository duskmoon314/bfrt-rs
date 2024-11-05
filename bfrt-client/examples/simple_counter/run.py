import argparse
import subprocess
import glob
import os

from fabric import Connection
from rich import print

parser = argparse.ArgumentParser(description="Simple Counter Example Run Script")
parser.add_argument(
    "-r",
    "--remote",
    metavar="REMOTE",
    type=str,
    help="Whether to compile and run on remote host",
)
parser.add_argument(
    "--remote-path",
    default="documents/simple_counter",
    help="Path on remote host to store files",
)


def main():
    args = parser.parse_args()

    # First compile rust code locally
    print("[green]Compiling rust code locally[/green]")
    res = subprocess.run(
        [
            "cargo",
            "build",
            "-r",
            "--package",
            "bfrt-client",
            "--example",
            "simple_counter",
            "--target=x86_64-unknown-linux-musl",
        ],
        capture_output=True,
    )

    if res.returncode != 0:
        print("[red]Failed to compile rust code[/red]")
        print(res.stderr)
        return

    print("[green]Rust code compiled successfully[/green]")

    if args.remote:
        c = Connection(args.remote)

        print("[green]Copying files to remote host[/green]")
        c.run(f"mkdir -p {args.remote_path}")

        c.put(
            "../../../target/x86_64-unknown-linux-musl/release/examples/simple_counter",
            args.remote_path,
        )
        p4src_files = glob.glob("p4src/**/*", recursive=True)
        p4src_files = [f for f in p4src_files if os.path.isfile(f)]
        for f in p4src_files:
            # Make dir if it doesn't exist
            c.run(f"mkdir -p {args.remote_path}/{os.path.dirname(f)}")
            c.put(f, f"{args.remote_path}/{f}")

        print("[green]Compiling p4 code on remote host[/green]")
        c.run(f"mkdir -p {args.remote_path}/build")

        with c.cd(f"{args.remote_path}/build"):
            c.run(
                f"cmake $SDE/p4studio/ -DCMAKE_INSTALL_PREFIX=$SDE_INSTALL -DCMAKE_MODULE_PATH=$SDE/cmake -DP4_NAME=simple_counter -DP4_PATH=$HOME/{args.remote_path}/p4src/simple_counter.p4",
                env={
                    "SDE": "~/bf-sde-9.7.5",
                    "SDE_INSTALL": "~/bf-sde-9.7.5/install",
                    "PATH": "~/bf-sde-9.7.5/install/bin:$PATH",
                },
            )
            c.run("make simple_counter -j && make install")

            print("[green]TODO: Run the program on remote host[/green]")

    else:
        print("[yellow]TODO: compile and run locally[/yellow]")


if __name__ == "__main__":
    main()
