# ⚭ tonic-prost-playground

This is a _playground_ educational repository for early learning/experimentation with the [Tonic](https://github.com/hyperium/tonic)/[PROST](https://github.com/tokio-rs/prost) stack. None of the provided software is production ready, nor intended to be upgraded. 

The playground implements a trivial two-service model we call the [_witch's pot_](#Witch's-Pot:-Visualization). The workspace contains a single _submodule_ which defines protocol buffers for a `Tasks` service. The `TasksServer` is implemented in TypeScript.

The [_greeter_](https://github.com/phasewalk1/tonic-prost-playground/tree/main/greeter) directory contains relevant _Tonic_/_Prost_ code which defines a second gRPC service, the `Greeter`. A novel connection is made between the Rust and TypeScript server within [server.rs](https://github.com/phasewalk1/tonic-prost-playground/blob/main/greeter/src/server.rs),
specifically, the asynchronous function `say_hello` - the defintion of `Greeter`'s RPC method.

Run Locally
---
We'd like to note that the executions happening under the hood here are not necessarily _interesting_ or _dynamic_ at all; they weren't designed that way. The software provided in this repository was designed only to demonstrate a method for connecting two gRPC servers (server-to-server RPC calls).

First, we need to sync the submodule
```sh
git submodule init && git submodule sync
git submodule update
```
Now we can install the `npm` dependencies for the TypeScript `Tasks` service.

```sh
cd tasks/ && npm i
```

Ok, let's start the `TasksServer` by

```sh
npm run serverstart
```

Now that we have a running `TasksServer`, let's try and hook our `Greeter` up. We'll first move into the `greeter/` directory

```sh
cd ../greeter
```

(assuming you were still in `/tasks`), and then, we'll run the `GreeterServer`

```sh
cargo serve
```

This is an alias provided in [config.toml](https://github.com/phasewalk1/tonic-prost-playground/blob/main/greeter/.cargo/config.toml) to quickly build and run our server code. We should now have two independent services running, a `TasksServer` and a `GreeterServer`. Let's use our `GreeterClient` to talk to our `GreeterServer` now,

```sh
cargo client
```

This is another alias provided that begins the main execution flow of this example. Here, we run a `GreeterClient` instance that calls an RPC method at the `GreeterServer`.
The `GreeterServer` then instantiates a `TasksClient` and uses it to make an RPC call at the `TasksServer`, bundling its return value into its own and returning the bundle to the `GreeterClient`.

## Witch's Pot: Visualization
Here's a visualization of the very simple execution architecture we just exercised above. We call it the _witch's pot_ model.


<img align="middle" src="doc/archi.JPG">
