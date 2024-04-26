#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

import sys
import argparse
import pexpect

class Simulation:
    def __init__(self, child):
        self.child = child

    @classmethod
    def from_args(cls):
        parser = argparse.ArgumentParser()
        parser.add_argument('cmd', nargs=argparse.REMAINDER)
        args = parser.parse_args()
        child = pexpect.spawn(args.cmd[0], args.cmd[1:], encoding='utf-8')
        child.logfile = sys.stdout
        return cls(child)
