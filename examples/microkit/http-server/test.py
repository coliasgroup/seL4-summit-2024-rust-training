#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

import time
from requests import Session

from harness import Simulation

HTTP_URL_BASE = 'http://localhost:8080'
HTTPS_URL_BASE = 'https://localhost:8443'

sim = Simulation.from_args()

sim.child.expect('completed system invocations', timeout=5)

time.sleep(3)

sim.flush_read()

try:
    for url_base in [HTTP_URL_BASE, HTTPS_URL_BASE]:
        sess = Session()
        url = url_base + '/About/'
        r = sess.get(url, verify=False, timeout=5)
        print(r.status_code)
        r.raise_for_status()
finally:
    sim.flush_read()
