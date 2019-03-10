
**A word of caution**: As noted in the [Nuxt.js presentation][pres] at Vue.js 
Amsterdam 2019, the next major version of Nuxt will likely feature a new backend 
services framework much better than what's currently offered by [Nuxt.js serverMiddleware][server-middleware].

Nevertheless, `serverMiddleware` can be a handy feature if you want to quickly 
embed a backend API into a Nuxt application without proxying requests to a
separate API server (or resorting to programmatic Nuxt embedded in another 
Node.js server). See [this article by Alexander Lichter][manniL] for more 
considerations. 

[pres]: https://www.youtube.com/watch?v=Ad5FF3BEY00
[server-middleware]: https://nuxtjs.org/api/configuration-servermiddleware/
[manniL]: https://blog.lichter.io/posts/my-take-on-using-nuxt-with-an-api/

# Nuxt serverMiddleware in Rust

Nuxt's underlying [connect][connect] server works with Express.js-compatible 
middleware, that is, a function that takes three paramaters: `request`, 
`response` and `next`. In order to use Rust serverMiddleware, all we need is
a native Node.js module that exports a function with that signature.

[connect]: https://github.com/senchalabs/connect

The [Neon bindings for Node.js][neon] make this surprisingly easy. All I had
to do to get the `middleware` folder of this sample repo started was typing
`neon new middleware`. A good hour of reading through the docs and I was able
to get a simple server middleware that adds a property to the response object:

```rust
fn middleware(mut ctx: FunctionContext) -> JsResult<JsUndefined> {
    let res = ctx.argument::<JsObject>(1)?;
    let next = ctx.argument::<JsFunction>(2)?;
    let message = ctx.string("Hello from Rust");
    res.set(&mut ctx, "neonMessage", message)?;
    let args: Vec<Handle<JsValue>> = vec![];
    let null = ctx.null();
    next.call(&mut ctx, null, args)?;
    Ok(ctx.undefined())
}
```

[neon]: https://neon-bindings.com

Running, in a nutshell:

```
yarn add global neon-cli --dev
cd middleware/
neon build
nuxt dev
```

Go to http://localhost:3000/.
