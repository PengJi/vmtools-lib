import ctypes

dll = ctypes.CDLL("c:\\foo.dll")

print(dll.check_svt_process())
