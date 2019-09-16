// TODO: Define how conditionally compiled code should be handled by call-graph generators.

pub mod lib {
    use traits::lib::FooTrait;
    use traits::lib::BarTrait;

    // There exist two versions of function 'foo' and which one is compiled depends on whether
    // feature foo is defined during compilation. If it is, 'foo' returns the result of calling
    // the 'method' method of trait object x. Otherwise, it returns the empty string. To achieve
    // this 'foo' calls 'foo_private', which is also conditionally compiled.
    #[cfg(feature = "foo")]
    pub fn foo(x: &dyn FooTrait) -> String {
        foo_private(x)
    }

    #[cfg(not(feature = "foo"))]
    pub fn foo(x: &dyn FooTrait) -> String {
        foo_private(x)
    }

    // 'foo_private' works as an indirection step. The 'foo' function that is compiled when
    // the foo feature is enabled calls the 'foo_private' function compiled when the foo
    // feature is enabled.
    #[cfg(feature = "foo")]
    fn foo_private(x: &dyn FooTrait) -> String {
        x.method()
    }

    #[cfg(not(feature = "foo"))]
    fn foo_private(_x: &dyn FooTrait) -> String {
        String::new()
    }

    // There exist two versions of function 'bar' similar to those of function 'foo'.
    #[cfg(feature = "bar")]
    pub fn bar(x: &dyn BarTrait) -> String {
        x.another_method()
    }

    #[cfg(not(feature = "bar"))]
    pub fn bar(_x: &dyn BarTrait) -> String {
        String::new()
    }

}
