import importlib
import sys

# 1️⃣ Wymuś załadowanie Rustowego modułu .so
import algo.main_mod as _main_mod

# 2️⃣ Wstrzyknij wszystkie submoduły dynamiczne z Rust
for _sub in ("graphs", "matrixes", "positions", "utils"):
    mod = getattr(_main_mod, _sub, None)
    if mod is not None:
        sys.modules[__name__ + "." + _sub] = mod
        globals()[_sub] = mod
