#[macro_use]
extern crate neon;

use neon::prelude::*;

fn middleware(mut ctx: FunctionContext) -> JsResult<JsUndefined> {
    // If we also need the request object:
    // let req = ctx.argument::<JsObject>(0)?;
    let res = ctx.argument::<JsObject>(1)?;
    let next = ctx.argument::<JsFunction>(2)?;

    // Create a JsString with the Neon helper
    let message = ctx.string("Hello from Rust");
    res.set(&mut ctx, "neonMessage", message)?;

    // Empt argument list for next(), must be Vec<Handle>
    let args: Vec<Handle<JsValue>> = vec![];
    // Get a null reference with the Neon helper
    let null = ctx.null();
    // Call serverMiddleware's next() function
    next.call(&mut ctx, null, args)?;

    // Must return JsUndefined
    Ok(ctx.undefined())
}

// Neon has no support for default exports yet
register_module!(mut ctx, {
    ctx.export_function("middleware", middleware)
});

