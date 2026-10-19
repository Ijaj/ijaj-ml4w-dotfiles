#!/usr/bin/env python3
"""Strata: focus-recency depth. The focused window is crisp; every window you used longer ago
steps down in opacity and border tone, along the wallpaper palette."""
import os
import select
import sys

sys.path.insert(0, os.path.dirname(os.path.realpath(__file__)))
import common as c

REFRESH_EVENTS = {"activewindowv2", "openwindow", "closewindow", "movewindowv2", "workspacev2", "fullscreen", "focusedmonv2"}
base_inactive = [0.9]
last = {}


def ranks():
    cl = [w for w in c.clients() if w.get("mapped")]
    cl.sort(key=lambda w: w.get("focusHistoryID", 999))
    return cl


def apply():
    pal = c.palette()
    if not pal:
        return
    cmds = []
    for r, w in enumerate(ranks()):
        a = w["address"]
        if r == 0:
            op, col = f"1.0 {base_inactive[0]:.2f}", None
        else:
            depth = min(1.0, (r - 1) / 4)
            op = f"1.0 {max(0.5, 0.94 - 0.11 * (r - 1)):.2f}"
            col = c.rgba(c.mix(pal["primary"], pal["outline_variant"], depth), 1.0 - 0.45 * depth)
        key = (op, col)
        if last.get(a) == key:
            continue
        last[a] = key
        cmds.append(c.set_prop(a, "opacity", op))
        cmds.append(c.set_prop(a, "inactive_border_color", col or "unset"))
    c.batch(cmds)


def setup():
    base_inactive[0] = c.option("decoration:inactive_opacity").get("float", 0.9)
    apply()


def loop(stop):
    gen = c.events()
    # events() blocks on recv; apply from a short-timeout reader so SIGTERM is honored
    import threading
    flag = threading.Event()

    def reader():
        for ev, _ in gen:
            if ev in REFRESH_EVENTS:
                flag.set()

    threading.Thread(target=reader, daemon=True).start()
    while not stop["flag"]:
        if flag.wait(0.5):
            flag.clear()
            apply()


def cleanup():
    cmds = []
    for w in c.clients():
        cmds.append(c.set_prop(w["address"], "opacity", f"1.0 {base_inactive[0]:.2f}"))
        cmds.append(c.set_prop(w["address"], "inactive_border_color", "unset"))
    c.batch(cmds)


sys.exit(c.run_until_term(setup, loop, cleanup))
