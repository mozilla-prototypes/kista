// Copyright 2018 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate rkv;
extern crate tempdir;

use rkv::{
	MANAGER,
	Rkv,
};

use self::tempdir::TempDir;

use std::fs;

use std::sync::{
    Arc,
};

#[test]
// Identical to the same-named unit test, but this one confirms that it works
// via the public MANAGER singleton.
fn test_same() {
    let root = TempDir::new("test_same_singleton").expect("tempdir");
    fs::create_dir_all(root.path()).expect("dir created");

    let p = root.path();
    assert!(MANAGER.read().unwrap().get(p).expect("success").is_none());

    let created_arc = MANAGER.write().unwrap().get_or_create(p, Rkv::new).expect("created");
    let fetched_arc = MANAGER.read().unwrap().get(p).expect("success").expect("existed");
    assert!(Arc::ptr_eq(&created_arc, &fetched_arc));
}
