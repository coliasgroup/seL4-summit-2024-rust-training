#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

from harness import Simulation

sim = Simulation.from_args()
sim.child.expect('banscii>', timeout=3)
sim.child.sendline('Hello, World!')
sim.child.expect('banscii>', timeout=1)
print()
