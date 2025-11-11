# zapewnij, że ten moduł ładuje się z Rustowego .so
from algo.main_mod import graphs as _graphs
globals().update({k: getattr(_graphs, k) for k in dir(_graphs) if not k.startswith("_")})
