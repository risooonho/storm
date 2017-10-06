use gl;
use std::mem;
use std::cmp;
use std::ptr;

use render::enums::buffer_type::*;

pub struct Buffer<T> {
    id: u32,
    dirty: bool,
    buffer_min: usize,
    buffer_max: usize,
    buffer_capacity: usize,
    buffer_type: BufferType,
    items: Vec<T>,
}

impl<T> Buffer<T> {
    const ELEMENT_SIZE: usize = mem::size_of::<T>();
    const DEFAULT_CAPACITY: usize = 16;
    const DEFAULT_SIZE: usize = Buffer::<T>::ELEMENT_SIZE * Buffer::<T>::DEFAULT_CAPACITY;

    pub fn new(buffer_type: BufferType) -> Buffer<T> {
        unsafe {
            let items: Vec<T> = Vec::<T>::with_capacity(Buffer::<T>::DEFAULT_CAPACITY);
            let mut id = mem::uninitialized::<u32>();
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(buffer_type.to_gl_enum(), id);
            gl::BufferData(
                buffer_type.to_gl_enum(),
                Buffer::<T>::DEFAULT_SIZE as isize,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            Buffer {
                id: id,
                dirty: false,
                buffer_min: 0,
                buffer_max: 0,
                buffer_capacity: Buffer::<T>::DEFAULT_CAPACITY,
                buffer_type: buffer_type,
                items: items,
            }
        }
    }

    pub fn add(&mut self, item: T) {
        let start = self.items.len();
        self.items.push(item);
        if self.dirty {
            self.buffer_min = cmp::min(self.buffer_min, start);
            self.buffer_max = cmp::max(self.buffer_max, start + 1);
        } else {
            self.dirty = true;
            self.buffer_min = start;
            self.buffer_max = start + 1;
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type.to_gl_enum(), self.id);
        }
    }

    pub fn sync(&mut self) {
        unsafe {
            if self.dirty {
                gl::BindBuffer(self.buffer_type.to_gl_enum(), self.id);
                self.dirty = false;
                if self.buffer_capacity < self.items.capacity() {
                    let length = (Buffer::<T>::ELEMENT_SIZE * self.items.capacity()) as isize;
                    let offset = self.items.as_ptr() as *const _;
                    gl::BufferData(
                        self.buffer_type.to_gl_enum(),
                        length,
                        offset,
                        gl::DYNAMIC_DRAW,
                    );
                    self.buffer_capacity = self.items.capacity();
                } else {
                    let start = (Buffer::<T>::ELEMENT_SIZE * self.buffer_min) as isize;
                    let length = (Buffer::<T>::ELEMENT_SIZE * (self.buffer_max - self.buffer_min)) as isize;
                    let offset = self.items.as_ptr().offset(self.buffer_min as isize) as *const _;
                    gl::BufferSubData(self.buffer_type.to_gl_enum(), start, length, offset);
                }
            }
        }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        println!("Dropping Buffer");
        // unsafe {
        //     gl::DeleteVertexArrays(1, self.vao as *const _);
        //     gl::DeleteBuffers(1, self.vbo as *const _);
        // }
        println!("Dropped Buffer");
    }
}
