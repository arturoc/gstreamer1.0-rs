#Gstreamer 1.0 Rust bindings

Idiomatic bindings for Gstreamer on Rust.

To use it you need to install GStreamer first, in linux it's usually installed with the distribution or avaliable through the package manager in other platforms from: http://gstreamer.freedesktop.org/data/pkg/

The wrapped objects do the ref/unref of the native GObjects themselves when references are dropped.

Most of the communication with the library can be done through mpsc::Receivers to get messages, samples...

To build the examples:

```bash
cargo test
``` 

`target/examples/appsink`

- creates a pipeline that opens the default audio input and calculates the rms of the incomming sound.

`target/examples/gst-launch gstreamer-pìpeline`

- creates any gstreamer pipeline, to test for example: `target/examples/gst-launch videotestsrc ! autovideosink`

`target/examples/playbin mediafile`

- creates a playbin opening and playing back the path passed as argument

`target/examples/v4l2`

- linux only, opens the default camera and shows the output through the default video sink (uses autovideosink)