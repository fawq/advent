from algo.main_mod import positions as _positions
globals().update({k: getattr(_positions, k) for k in dir(_positions) if not k.startswith("_")})
