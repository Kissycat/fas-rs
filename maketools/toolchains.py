#!/bin/python3
#
# Copyright 2023 shadow3aaa@gitbub.com
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
import os
from pathlib import Path
from maketools.misc import *

if os.getenv("TERMUX_VERSION") is not None:
    __cargo = "cargo"
    __strip = "strip"
    __clang_plusplus = "clang++"
elif (__ndk_home := os.getenv("ANDROID_NDK_HOME")) is not None:
    __bins = (
        Path(__ndk_home)
        .joinpath("toolchains")
        .joinpath("llvm")
        .joinpath("prebuilt")
        .joinpath("linux-x86_64")
        .joinpath("bin")
        .joinpath("llvm-strip")
    )
    __cargo = "cargo ndk -p 31 -t arm64-v8a"
    __strip = bins.joinpath("llvm-strip")
    __clang_plusplus = __bins.joinpath("aarch64-linux-android31-clang++")
else:
    eprint("Missing env: ANDROID_NDK_HOME")
    exit(-1)


def cargo(arg: str):
    os.system("{} {}".format(__cargo, arg))


def strip(arg: str):
    os.system("{} {}".format(__strip, arg))


def clang_plusplus(arg: str):
    os.system("{} {}".format(__clang_plusplus, arg))