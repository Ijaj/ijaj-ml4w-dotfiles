"""Shared helpers for the live (reactive) theme daemons: Hyprland IPC, wallpaper palette, color math."""
import glob
import json
import os
import re
import signal
import socket
import subprocess
import sys
import time

HOME = os.path.expanduser("~")
COLORS = f"{HOME}/.config/waybar/colors.css"
STATE = f"{HOME}/.local/state/ml4w-custom/live"
os.makedirs(STATE, exist_ok=True)


# ---- Hyprland ---------------------------------------------------------------

def _sig():
    sig = os.environ.get("HYPRLAND_INSTANCE_SIGNATURE")
    if sig:
        return sig
    rt = os.environ.get("XDG_RUNTIME_DIR", f"/run/user/{os.getuid()}")
    socks = glob.glob(f"{rt}/hypr/*/.socket2.sock")
    if not socks:
        return None
    return os.path.basename(os.path.dirname(max(socks, key=os.path.getmtime)))


def wait_for_hyprland(timeout=60):
    end = time.time() + timeout
    while time.time() < end:
        if _sig():
            return True
        time.sleep(1)
    return False


def _env():
    e = dict(os.environ)
    s = _sig()
    if s:
        e["HYPRLAND_INSTANCE_SIGNATURE"] = s
    return e


def hctl(*args):
    r = subprocess.run(["hyprctl", *args], capture_output=True, text=True, env=_env())
    return r.stdout


def lua(code):
    """Run Lua in the compositor (this Hyprland build has no legacy `hyprctl keyword`)."""
    return hctl("eval", code)


def batch(cmds):
    if cmds:
        hctl("--batch", "; ".join(cmds))


def clients():
    try:
        return json.loads(hctl("clients", "-j"))
    except ValueError:
        return []


def set_prop(addr, prop, value):
    return f'dispatch hl.dsp.window.set_prop({{prop="{prop}", value="{value}", window="address:{addr}"}})'


def option(name):
    try:
        return json.loads(hctl("getoption", name, "-j"))
    except ValueError:
        return {}


def events():
    """Yield (event, data) lines from Hyprland's event socket; reconnects forever."""
    while True:
        s = _sig()
        path = f"{os.environ.get('XDG_RUNTIME_DIR', f'/run/user/{os.getuid()}')}/hypr/{s}/.socket2.sock" if s else None
        if not path or not os.path.exists(path):
            time.sleep(2)
            continue
        try:
            sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
            sock.connect(path)
            buf = b""
            while True:
                chunk = sock.recv(4096)
                if not chunk:
                    break
                buf += chunk
                while b"\n" in buf:
                    line, buf = buf.split(b"\n", 1)
                    ev, _, data = line.decode(errors="replace").partition(">>")
                    yield ev, data
        except OSError:
            pass
        time.sleep(2)


# ---- Palette and colors -----------------------------------------------------

def palette():
    pal = {}
    try:
        with open(COLORS) as f:
            for l in f:
                m = re.match(r"\s*@define-color ([a-z_0-9]+) #([0-9a-fA-F]{6})", l)
                if m:
                    h = m.group(2)
                    pal[m.group(1)] = tuple(int(h[i:i + 2], 16) for i in (0, 2, 4))
    except OSError:
        pass
    return pal


def mix(a, b, t):
    t = max(0.0, min(1.0, t))
    return tuple(round(x + (y - x) * t) for x, y in zip(a, b))


def rgba(c, alpha=1.0):
    return "rgba(%02x%02x%02x%02x)" % (*c, round(alpha * 255))


def css_hex(c):
    return "#%02x%02x%02x" % c


def parse_gradient(opt):
    """getoption gradient string 'AARRGGBB AARRGGBB 73deg' -> ([(r,g,b,a)...], angle)."""
    cols, angle = [], 0
    for tok in opt.get("gradient", "").split():
        if tok.endswith("deg"):
            angle = int(tok[:-3])
        else:
            v = int(tok, 16)
            cols.append(((v >> 16) & 255, (v >> 8) & 255, v & 255, (v >> 24) & 255))
    return cols, angle


def lua_gradient(cols, angle):
    items = ", ".join('"rgba(%02x%02x%02x%02x)"' % c for c in cols)
    return f"{{ colors = {{ {items} }}, angle = {angle} }}"


def set_active_border(cols, angle):
    lua(f"hl.config({{ general = {{ col = {{ active_border = {lua_gradient(cols, angle)} }} }} }})")


def set_shadow_color(rgba_str):
    lua(f'hl.config({{ decoration = {{ shadow = {{ color = "{rgba_str}" }} }} }})')


# ---- Process helpers --------------------------------------------------------

def run_until_term(setup, loop, cleanup):
    """setup(); loop() until SIGTERM/SIGINT; cleanup() always (restores the compositor state)."""
    stop = {"flag": False}

    def handler(*_):
        stop["flag"] = True

    signal.signal(signal.SIGTERM, handler)
    signal.signal(signal.SIGINT, handler)
    if not wait_for_hyprland():
        print("Hyprland not running", file=sys.stderr)
        return 1
    try:
        setup()
        loop(stop)
    finally:
        cleanup()
    return 0
