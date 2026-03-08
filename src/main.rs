//! # Rust Systems Programming — Memory Concepts
//!
//! Hands-on examples of low-level systems programming concepts in Rust:
//! - Stack vs Heap allocation
//! - Custom smart pointers
//! - Memory layout
//! - Unsafe Rust (with safety docs)
//! - Custom allocator patterns

use std::alloc::{alloc, dealloc, Layout};
use std::fmt;
use std::ptr::NonNull;

// ─────────────────────────────────────────────
//  1. Stack vs Heap Demonstration
// ─────────────────────────────────────────────

fn demo_stack_vs_heap() {
    println!("\n=== 1. Stack vs Heap ===");

    // Stack: fixed size, fast, auto-freed
    let stack_int: i32 = 42;
    let stack_arr: [u8; 4] = [1, 2, 3, 4];
    println!("Stack int: {} at {:p}", stack_int, &stack_int);
    println!("Stack arr: {:?} at {:p}", stack_arr, &stack_arr);

    // Heap: dynamic size, manual or RAII managed
    let heap_string = String::from("Hello, Heap!");
    let heap_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Heap String: {} at {:p}", heap_string, heap_string.as_ptr());
    println!("Heap Vec: {:?} at {:p}", heap_vec, heap_vec.as_ptr());

    // Box<T>: explicitly put value on heap
    let boxed = Box::new(100i32);
    println!("Boxed i32: {} at {:p}", boxed, &*boxed);
} // heap_string, heap_vec, boxed all dropped here

// ─────────────────────────────────────────────
//  2. Custom Smart Pointer
// ─────────────────────────────────────────────

/// A simple smart pointer that tracks how many times it's dereferenced
struct TrackingPointer<T> {
    value: Box<T>,
    deref_count: usize,
}

impl<T> TrackingPointer<T> {
    fn new(value: T) -> Self {
        println!("[TrackingPointer] Allocated");
        Self {
            value: Box::new(value),
            deref_count: 0,
        }
    }

    fn get(&mut self) -> &T {
        self.deref_count += 1;
        &self.value
    }

    fn access_count(&self) -> usize {
        self.deref_count
    }
}

impl<T: fmt::Display> fmt::Display for TrackingPointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TrackingPointer({}, accessed {} times)", self.value, self.deref_count)
    }
}

impl<T> Drop for TrackingPointer<T> {
    fn drop(&mut self) {
        println!("[TrackingPointer] Freed (was accessed {} times)", self.deref_count);
    }
}

fn demo_custom_smart_pointer() {
    println!("\n=== 2. Custom Smart Pointer ===");

    let mut ptr = TrackingPointer::new(42i32);
    println!("Value: {}", ptr.get());
    println!("Value: {}", ptr.get());
    println!("Value: {}", ptr.get());
    println!("Access count: {}", ptr.access_count());
    println!("{}", ptr);
    // ptr is dropped here, triggers our custom Drop
}

// ─────────────────────────────────────────────
//  3. Memory Layout
// ─────────────────────────────────────────────

#[repr(C)] // Force C-compatible memory layout
struct CCompatStruct {
    a: u8,
    b: u32,
    c: u8,
}

struct RustStruct {
    a: u8,
    b: u32,
    c: u8,
}

fn demo_memory_layout() {
    println!("\n=== 3. Memory Layout ===");

    println!("Size of u8:   {} bytes", std::mem::size_of::<u8>());
    println!("Size of u32:  {} bytes", std::mem::size_of::<u32>());
    println!("Size of u64:  {} bytes", std::mem::size_of::<u64>());
    println!("Size of f64:  {} bytes", std::mem::size_of::<f64>());
    println!("Size of bool: {} bytes", std::mem::size_of::<bool>());
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
    println!("Size of &str: {} bytes (ptr + len)", std::mem::size_of::<&str>());
    println!("Size of String: {} bytes (ptr + len + cap)", std::mem::size_of::<String>());

    println!("\nPadding comparison:");
    println!(
        "CCompatStruct (#[repr(C)]): {} bytes (with C-style padding)",
        std::mem::size_of::<CCompatStruct>()
    );
    println!(
        "RustStruct (default):       {} bytes (Rust reorders fields optimally)",
        std::mem::size_of::<RustStruct>()
    );
}

// ─────────────────────────────────────────────
//  4. Raw Pointers (Unsafe Rust)
// ─────────────────────────────────────────────

/// # Safety
/// Demonstrates raw pointer operations. Safe because:
/// - We create the pointer from a valid reference
/// - We never dereference a null pointer
/// - We never access freed memory
fn demo_raw_pointers() {
    println!("\n=== 4. Raw Pointers (Unsafe) ===");

    let mut value: i32 = 100;
    let raw_ptr: *mut i32 = &mut value as *mut i32;

    println!("Value before: {}", value);
    println!("Raw pointer address: {:p}", raw_ptr);

    // SAFETY: raw_ptr is valid — it came from a mutable reference
    // and value is still in scope
    unsafe {
        *raw_ptr = 999;
        println!("Value via raw pointer write: {}", *raw_ptr);
    }

    println!("Value after: {}", value);
}

// ─────────────────────────────────────────────
//  5. Manual Heap Allocation with Global Allocator
// ─────────────────────────────────────────────

fn demo_manual_allocation() {
    println!("\n=== 5. Manual Heap Allocation ===");

    // Allocate space for 4 i32s on the heap manually
    let layout = Layout::array::<i32>(4).unwrap();

    // SAFETY: layout is non-zero size, and we check for null
    let ptr = unsafe { alloc(layout) as *mut i32 };

    if ptr.is_null() {
        eprintln!("Allocation failed!");
        return;
    }

    // Write to the allocated memory
    unsafe {
        *ptr.add(0) = 10;
        *ptr.add(1) = 20;
        *ptr.add(2) = 30;
        *ptr.add(3) = 40;

        println!(
            "Manually allocated array: [{}, {}, {}, {}]",
            *ptr.add(0), *ptr.add(1), *ptr.add(2), *ptr.add(3)
        );

        // MUST free manually — no RAII here
        dealloc(ptr as *mut u8, layout);
        println!("Memory deallocated successfully");
    }
}

// ─────────────────────────────────────────────
//  6. Reference Counting (Rc / Arc)
// ─────────────────────────────────────────────

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn demo_reference_counting() {
    println!("\n=== 6. Reference Counting ===");

    // Rc<T> — single-threaded shared ownership
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);

    println!("Rc strong count: {}", Rc::strong_count(&shared));
    clone1.borrow_mut().push(4);
    clone2.borrow_mut().push(5);
    println!("Shared vec: {:?}", shared.borrow());

    // Arc<T> — multi-threaded shared ownership
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            *c.lock().unwrap() += 1;
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Arc counter after 5 threads: {}", *counter.lock().unwrap());
}

// ─────────────────────────────────────────────
//  Main
// ─────────────────────────────────────────────

fn main() {
    println!("🦀 Rust Systems Programming — Memory Concepts");
    println!("{}", "═".repeat(50));

    demo_stack_vs_heap();
    demo_custom_smart_pointer();
    demo_memory_layout();
    demo_raw_pointers();
    demo_manual_allocation();
    demo_reference_counting();

    println!("\n{}", "═".repeat(50));
    println!("✅ All demos completed.");
}
