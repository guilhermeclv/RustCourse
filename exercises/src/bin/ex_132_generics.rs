
// Fill in the blanks to make it work
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(s: SGen<T>)->T {return s.0;} // Implement generic for SGen<T>

fn main() {
    // Using the non-generic functions
    let s = S(A);
    let s_obj = SGen(A); // Concrete type `SGen<S>`.
    reg_fn(s);          // Concrete type.
    gen_spec_t(s_obj);   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(23)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    let xx = generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    let xxx = generic(SGen('a'));

    println!("Success!");
}