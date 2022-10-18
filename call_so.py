#!/usr/bin/env python
import ctypes

test = ctypes.cdll.LoadLibrary("/home/codestore/lang/rust/api_lib/target/debug/libfoo.so")

print(test.check_svt_process())
