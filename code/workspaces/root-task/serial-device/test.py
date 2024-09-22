#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

import time

from harness import Simulation

sim = Simulation.from_args()
sim.child.expect('echo> ', timeout=5)
time.sleep(1)
sim.child.send('xxx')
sim.child.expect('\[x\]\[x\]\[x\]', timeout=5)
print()
