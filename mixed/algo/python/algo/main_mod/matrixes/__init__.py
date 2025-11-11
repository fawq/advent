from algo.main_mod import matrixes as _matrixes
globals().update({k: getattr(_matrixes, k) for k in dir(_matrixes) if not k.startswith("_")})
