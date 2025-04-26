pub use adding::add_curry;

pub fn twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| f(f(x))
}