// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//


#[crate_id = "forkjoin#0.10-pre"];
#[crate_type = "rlib"];
#[crate_type = "dylib"];
#[license = "MIT/ASL2"];
#[doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
      html_favicon_url = "http://www.rust-lang.org/favicon.ico",
      html_root_url = "http://static.rust-lang.org/doc/master")];


extern mod extra;
extern mod native;

pub mod workqueue;

// exported funs
pub fn print_testing(){
    println!("says hello kanki!");
}



#[cfg(test)]
mod test {
    use forkjoin;

    #[test]
    fn compile_test () {
        print_testing();
    }
    
}