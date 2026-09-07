"""Generate the original H template mark using the existing Tauri icon pipeline.

Adapted from macos-icon.gen.py (codeg v0.30.4, Apache-2.0; see NOTICE).
macOS tints the alpha channel. No Pillow dependency is required.
"""

from pathlib import Path
import shutil
import subprocess
import tempfile

ICONS_DIR = Path(__file__).resolve().parent
REPO_ROOT = ICONS_DIR.parents[1]


def main():
    with tempfile.TemporaryDirectory() as tmp:
        tmp_dir = Path(tmp)
        source = tmp_dir / "tray.svg"
        source.write_text(
            '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 44 44">'
            '<path d="M12 9V35M32 9V35M12 22H32" fill="none" '
            'stroke="black" stroke-width="5" stroke-linecap="round"/></svg>'
        )
        subprocess.run(
            ["pnpm", "tauri", "icon", str(source), "-o", str(tmp_dir), "-p", "44"],
            cwd=REPO_ROOT,
            check=True,
        )
        shutil.copyfile(tmp_dir / "44x44.png", ICONS_DIR / "tray-icon-template.png")


if __name__ == "__main__":
    main()
