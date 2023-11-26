/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::framework::itest;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Object)]
pub struct MyClass {
    #[base]
    base: Base<Object>,

    first_called_pre: bool,
    first_called_post: bool,
    second_called: bool,
}

#[godot_api]
impl MyClass {
    #[signal]
    fn some_signal();

    #[func]
    fn first_calls(&mut self) {
        self.first_called_pre = true;
        self.base_mut().call("second".into(), &[]);
        self.first_called_post = true;
    }

    #[func]
    fn first_signal(&mut self) {
        self.first_called_pre = true;
        self.base_mut().emit_signal("some_signal".into(), &[]);
        self.first_called_post = true;
    }

    #[func]
    fn second(&mut self) {
        self.second_called = true;
    }
}

#[itest]
fn reentrant_call_succeeds() {
    let mut class = MyClass::alloc_gd();

    assert!(!class.bind().first_called_pre);
    assert!(!class.bind().first_called_post);
    assert!(!class.bind().second_called);

    class.call("first_calls".into(), &[]);

    assert!(class.bind().first_called_pre);
    assert!(class.bind().first_called_post);
    assert!(class.bind().second_called);

    class.free()
}

#[itest]
fn reentrant_emit_succeeds() {
    let mut class = MyClass::alloc_gd();

    let callable = class.callable("second");
    class.connect("some_signal".into(), callable);

    assert!(!class.bind().first_called_pre);
    assert!(!class.bind().first_called_post);
    assert!(!class.bind().second_called);

    class.call("first_signal".into(), &[]);

    assert!(class.bind().first_called_pre);
    assert!(class.bind().first_called_post);
    assert!(class.bind().second_called);

    class.free()
}
