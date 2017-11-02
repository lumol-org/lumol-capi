use std::ptr;
use std::ffi::CStr;
use std::os::raw::*;

use std::ops::{Deref, DerefMut};

use lumol::sys::{System, Particle};
use lumol::types::Vector3D;

use status::lml_status;

#[allow(non_camel_case_types)]
pub struct lml__system__private__(System);

impl lml__system__private__ {
    fn new() -> lml__system__private__ {
        lml__system__private__(System::new())
    }
}

impl Deref for lml__system__private__ {
    type Target = System;
    fn deref<'b>(&'b self) -> &'b System {
        &self.0
    }
}

impl DerefMut for lml__system__private__ {
    fn deref_mut<'b>(&'b mut self) -> &'b mut System {
        &mut self.0
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct lml_system_t {
    handle: *mut lml__system__private__,
    pub natoms: usize,
    pub masses: *mut f64,
    pub charges: *mut f64,
    pub positions: *mut [f64; 3],
    pub velocities: *mut [f64; 3],
}

struct SystemGuard<'a> {
    system: &'a mut lml_system_t,
}

impl<'a> Deref for SystemGuard<'a> {
    type Target = lml__system__private__;
    fn deref<'b>(&'b self) -> &'b lml__system__private__ {
        unsafe {
            &*self.system.handle
        }
    }
}

impl<'a> DerefMut for SystemGuard<'a> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut lml__system__private__ {
        unsafe {
            &mut *self.system.handle
        }
    }
}

impl<'a> Drop for SystemGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            self.system.update();
        }
    }
}

impl lml_system_t {
    fn null() -> lml_system_t {
        lml_system_t {
            handle: ptr::null_mut(),
            natoms: 0,
            masses: ptr::null_mut(),
            charges: ptr::null_mut(),
            positions: ptr::null_mut(),
            velocities: ptr::null_mut(),
        }
    }

    unsafe fn new(handle: *mut lml__system__private__) -> lml_system_t {
        let mut system = lml_system_t::null();
        system.handle = handle;
        system.update();
        return system;
    }

    unsafe fn get(&mut self) -> SystemGuard {
        SystemGuard {
            system: self
        }
    }

    unsafe fn update(&mut self) {
        let system: &mut lml__system__private__ = &mut *self.handle;
        self.natoms = system.size();
        self.masses = system.particles_mut().mass.as_mut_ptr();
        self.charges = system.particles_mut().charge.as_mut_ptr();
        self.positions = system.particles_mut().position.as_mut_ptr() as *mut _;
        self.velocities = system.particles_mut().velocity.as_mut_ptr() as *mut _;
    }
}

#[no_mangle]
pub unsafe extern fn lml_system() -> lml_system_t {
    lml_system_t::new(Box::into_raw(Box::new(lml__system__private__::new())))
}

#[no_mangle]
pub unsafe extern fn lml_system_add_particle(system: &mut lml_system_t,
                                             name: *const c_char,
                                             position: [f64; 3],
                                             velocity: [f64; 3]) -> lml_status {
    let name = try!(CStr::from_ptr(name).to_str());
    let mut particle = Particle::new(name);
    particle.position = Vector3D::from(position);
    particle.velocity = Vector3D::from(velocity);

    system.get().add_particle(particle);
    return lml_status::LML_SUCCESS;
}

#[no_mangle]
pub unsafe extern fn lml_system_free(system: lml_system_t) {
    Box::from_raw(system.handle);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    use std::slice;
    use std::ffi::CString;

    impl lml_system_t {
        pub unsafe fn masses(&self) -> &[f64] {
            slice::from_raw_parts(self.masses, self.natoms)
        }

        pub unsafe fn charges(&self) -> &[f64] {
            slice::from_raw_parts(self.charges, self.natoms)
        }

        pub unsafe fn positions(&self) -> &[[f64; 3]] {
            slice::from_raw_parts(self.positions, self.natoms)
        }

        pub unsafe fn velocities(&self) -> &[[f64; 3]] {
            slice::from_raw_parts(self.velocities, self.natoms)
        }
    }

    #[test]
    fn pointers() {
        unsafe {
            let mut system = lml_system();
            assert!(system.handle != ptr::null_mut());

            let name = CString::new("F").unwrap();
            let status = lml_system_add_particle(&mut system, name.as_ptr(), [1.0, 2.0, 3.0], [0.1, 0.2, 0.3]);
            assert_eq!(status, lml_status::LML_SUCCESS);

            let status = lml_system_add_particle(&mut system, name.as_ptr(), [0.0; 3], [1.0; 3]);
            assert_eq!(status, lml_status::LML_SUCCESS);

            assert_eq!(system.natoms, 2);

            assert_eq!(system.masses()[0], 18.9984032);
            assert_eq!(system.masses()[1], 18.9984032);

            assert_eq!(system.charges()[0], 0.0);
            assert_eq!(system.charges()[1], 0.0);

            assert_eq!(system.positions()[0], [1.0, 2.0, 3.0]);
            assert_eq!(system.positions()[1], [0.0, 0.0, 0.0]);

            assert_eq!(system.velocities()[0], [0.1, 0.2, 0.3]);
            assert_eq!(system.velocities()[1], [1.0, 1.0, 1.0]);

            lml_system_free(system);
        }
    }
}
