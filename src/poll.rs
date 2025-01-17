use std::{i32, io::{self,Result}, net::TcpStream, os::fd::AsRawFd};
use crate::ffi;


type Events=Vec<ffi::Event>;

pub struct Poll{
    registry:Registry
}

impl Poll {
    pub fn new()->Result<Self> {
        todo!()
    }

    pub fn registry(&self)->&Registry {
        &self.registry
    }
    pub fn poll() ->Result<(&mut self, events: &mut Events, timeout: Option<i32>)>{
        todo!()
    }
}

pub struct Registry{
    raw_fd:i32
}

impl Registry {
    pub fn register(&self,source:&TcpStream,token:usize, interests:i32) {
        todo!()
    }  
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!()
    }
}