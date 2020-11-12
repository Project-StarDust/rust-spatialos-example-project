# rust-spatialos-example-project
Example project for our rust unnoficial SpatialOS SDK

***Mirroring [this project](https://github.com/spatialos/CExampleProjec)***

# SpatialOS Rust example project

***All of the informations below are done or planned**

## Dependencies

This project contains workers written in rust and use [cargo](https://doc.rust-lang.org/cargo/)
as their build system. Your system needs to have [rust installed](https://www.rust-lang.org/tools/install).

It is recommended to use [sccache](https://crates.io/crates/sccache) to avoid recompilation of the same libraries for differents workers.


## Quick start

Build the project and start it with the default launch configuration:

```
spatial worker build --target windows
spatial local launch --runtime_version=14.5.4
```

(Replacing `windows` with `macos` on macOS, or `linux` on Linux).

This will launch SpatialOS locally with a single rust "physics" worker that updates the position of
a single entity. You may also see a 2nd entity called "physics-worker" created. This entity
represents the managed worker itself.

Note: If you run `spatial worker build` without a `--target` parameter (or with the wrong target
parameter), then rust won't be able to compile the executable for each platforms, resulting in a failed build.

Now, you can connect either one of the two rust client workers (one implemented using "direct"
serialization, the other implemented using "vtable" serialization). These workers can be
launched with the following commands:

* Client (direct): `spatial local worker launch client_direct local`
* Client (vtable): `spatial local worker launch client_vtable local`

## Scenario

This project is used to showcase our unnoficial rust SDK API and how it can be used to implement a simple client
worker which visualizes the state of a single entity whose position is updated by a "physics"
worker. As serialization in the rust SDK API can be implemented in two different ways, we provide two
implementations of the same worker in `workers/c_client_direct` and `workers/c_client_vtable`.
Either one of these can be used as a basis for further experimentation, and the client worker that's
not being used can easily be deleted without breaking any other functionality.

When a client worker connects, it sends a command to the `physics` rust worker (on the `sample.Login`
component). The `physics` rust worker then modifies the entity's write ACLs to delegate component 1001
(`sample.ClientData`) to the client, using the `CallerWorkerAttributes` field of the
`CommandRequestOp`. This causes the entity to be checked out by the client worker, and the client
worker will begin to receive component updates for position changes. The physics worker will also
begin to send a simple command to the client every few seconds. Due to the entity's QBI query,
the client worker will check in/out another entity due to its movement.

## Snapshot

The snapshot exists in both JSON and binary format in the `snapshots` folder. There is no script
to generate the snapshot as the snapshot was written by hand in JSON format, but it's possible
to make simple changes to the JSON snapshot and regenerate the binary snapshot from it. To update the
binary snapshot after making a change, run the following command:

```
spatial project history snapshot convert --input-format=text --input=snapshots/default.json --output-format=binary --output=snapshots/default.snapshot
```
