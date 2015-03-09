#Gstreamer 1.0 Rust bindings

Idiomatic bindings for Gstreamer on Rust.

To use it you need to install GStreamer first, in linux it's usually installed with the distribution or avaliable through the package manager in other platforms from: http://gstreamer.freedesktop.org/data/pkg/

The wrapped objects do the ref/unref of the native GObjects themselves when references are dropped.

Most of the communication with the library can be done through mpsc::Receivers to get messages, samples... 