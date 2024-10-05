<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Using a Serial Device

In this chapter, we will build a root task that interacts with a serial device.
Start by navigating to and running this chapter's example, which, so far, doesn't do anything interesting.

```
cd workspaces/root-task/serial-device
make simulate
```

The module at {{#gh_link @-5 (workspaces/root-task/serial-device/src/)device.rs}} implements a higher-level interface over the serial device's MMIO registers, whose physical base address is:

{{#fragment_with_gh_link "rust,ignore" @-5 workspaces/root-task/serial-device/src/main.rs:16:16}}

Our first goal will be to map the serial device's MMIO registers into the root task's address space.

After that, we will set up the root task's access to the serial device's interrupt, whose value is:

{{#fragment_with_gh_link "rust,ignore" @-5 workspaces/root-task/serial-device/src/main.rs:18:18}}

Finally, we will implement a simple loop that echoes serial input to serial output.

{{#step 5.A}}

First, add some familiar snippets that we will use for allocating CSlots and kernel objects:

{{#fragment_with_gh_link "rust,ignore" @5.A workspaces/root-task/serial-device/src/main.rs:34:44}}

{{#fragment_with_gh_link "rust,ignore" @5.A workspaces/root-task/serial-device/src/main.rs:22:27}}

{{#step 5.B (exercise)}}

`largest_kernel_ut` will be useful for allocating kernel objects whose backing physical addresses don't matter to us, but we must allocate the frame which contains the serial device's MMIO registers at a particular physicall address (`SERIAL_DEVICE_MMIO_PADDR`).
Furthermore, the seL4 API distinguishes between _general purpose untyped_ _device untyped_. General purpose untyped is backed by normal memory, and can be used to create any type of object. Device untyped is not backed by normal memory, and can only be used to create frames.
See the last two paragraphs of {{#manual_link #2.4 (Kernel Memory Allocation)}} for more information.
So, we must allocate the serial device MMIO frame from the particular initial device untyped that contains `SERIAL_DEVICE_MMIO_PADDR`.

**Exercice:** Identify this initial untyped in the `bootinfo`. We will need a corresponding `sel4::cap::Untyped` along with the untyped's physical address (or `sel4::UntypedDesc`, which contains the physical address) for the next step.

{{#step 5.C}}

The untyped we've identified contains the frame we are targeting, but that frame may be somewhere in the middle of the region of device memory the untyped covers.
To allocate the frame at `SERIAL_DEVICE_MMIO_PADDR`, we must allocate dummy objects from this untyped until its watermark is at `SERIAL_DEVICE_MMIO_PADDR`.

This `trim_untyped` function takes the untyped capability, its physical address, the desired physical address, and two empty slots for temporarily holding dummy objects.
We need two slots because the kernel resets an untyped's watermark if it has no live children.
So, we must always keep one child around so that our progress on advancing the watermark is never lost.

{{#fragment_with_gh_link "rust,ignore" @5.C workspaces/root-task/serial-device/src/main.rs:43:49}}

{{#fragment_with_gh_link "rust,ignore" @5.C workspaces/root-task/serial-device/src/main.rs:68:95}}

{{#step 5.D}}

`device_ut_cap` is now primed; the physical address of the next allocation will be `SERIAL_DEVICE_MMIO_PADDR`.

**Exercise:** Allocate a small frame object (`sel4::cap_type::Granule`) from `device_ut_cap`.

If your `sel4::cap::Granule` is called `serial_device_frame_cap`, then the following assertion should succeed:

{{#fragment_with_gh_link "rust,ignore" @5.D workspaces/root-task/serial-device/src/main.rs:68:71}}

{{#step 5.E (exercise)}}

**Exercise:** Using code from [Step 4.B](./address-space.html#step-4b), [Step 4.C](./address-space.html#step-4c), and [Step 4.D](./address-space.html#step-4d-exercise), map `serial_device_frame_cap` into the root task's virtual address space.

You should now be able interact with the serial device's MMIO registers.
Try printing "Hello, World!" to the serial console with something like:

{{#fragment_with_gh_link "rust,ignore" @5.E workspaces/root-task/serial-device/src/main.rs:100:106}}

where `serial_device_mmio_page_addr: *mut _` is a pointer to where the MMIO registers are mapped in the root task's virtual address space.  

{{#step 5.F (exercise)}}

Interrupts are delivered to userspace via notifications.
A `IRQHandler` capability represents the authority to manage a particular interrupt.
Specifically, an `IRQHandler` capability ({{#rustdoc_link root-task sel4/cap/type.IrqHandler.html `sel4::cap::IrqHandler`}}) has the following methods:
- {{#manual_link [`seL4_IRQHandler_SetNotification()`] #10.3.4.3}}:
Associate the interrupt with the given notification. Userspace can call `seL4_Wait()` or `seL4_Poll()` on this notification to receive the interrupt.
- {{#manual_link [`seL4_IRQHandler_Clear()`] #10.3.4.3}}:
Disassociate the notification associated with this interrupts.
- {{#manual_link [`seL4_IRQHandler_Ack()`] #10.3.4.3}}:
Tell the kernel to pass on acknowledgement of this interrupt to the interrupt controller.

The Rust bindings for these methods are:
- {{#rustdoc_link root-task sel4/cap/type.IrqHandler.html#method.irq_handler_set_notification `sel4::cap::IrqHandler::irq_handler_set_notification()`}}
- {{#rustdoc_link root-task sel4/cap/type.IrqHandler.html#method.irq_handler_clear `sel4::cap::IrqHandler::irq_handler_clear()`}}
- {{#rustdoc_link root-task sel4/cap/type.IrqHandler.html#method.irq_handler_ack `sel4::cap::IrqHandler::irq_handler_ack()`}}

The root task spawns with a special `IRQControl` capability ({{#rustdoc_link root-task sel4/cap/type.IrqControl.html `sel4::cap::IrqControl`}}) which can be used to create `IRQHandler` capabilities with {{#manual_link [`seL4_IRQControl_Get()`] #10.3.3.1}} ({{#rustdoc_link root-task sel4/cap/type.IrqControl.html#method.irq_control_get `sel4::cap::IrqControl::irq_control_get()`}}).

The intent behind this API is that a highly-privileged component will hold an `IRQControl` capability, which it will use to distribute more finely-grained `IRQHandler` capabilities to less privileged components for the interrupts they will manage.

The root task can access its `IRQControl` capability with {{#rustdoc_link root-task sel4/init_thread/slot/constant.IRQ_CONTROL.html `sel4::init_thread::slot::IRQ_CONTROL.cap()`}}

**Exercise:** Use `sel4::init_thread::slot::IRQ_CONTROL.cap()` to create a `sel4::cap::IrqHandler` for `SERIAL_DEVICE_MMIO_PADDR`.

{{#step 5.G (exercise)}}

**Exercise:** Create a notification object from `largest_kernel_ut` and associate it with the `IRQHandler` you just created using {{#rustdoc_link root-task sel4/cap/type.IrqHandler.html#method.irq_handler_set_notification `sel4::cap::IrqHandler::irq_handler_set_notification()`}}.

{{#step 5.H (exercise)}}

**Exercise:** Use `serial_device: Device`, your `IRQHandler`, and the notification you associated with the interrupt to write a loop that echoes serial input to serial output.

Use `serial_device.clear_all_interrupts()` and `irq_handler_cap.irq_handler_ack()` in sequence to clear the interrupt at the device and interrupt controller levels respectively.
Note that you should do this at the beginning of the loop in case your loop enters with an interrupt already pending.

Use `irq_notification_cap.wait()` to wait for hte interrupt.

Use `serial_device.get_char()` and `serial_device.put_char()` to read and write data.
