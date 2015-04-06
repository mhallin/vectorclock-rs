**********************
Vector Clocks for Rust
**********************

.. image:: https://travis-ci.org/mhallin/vectorclock-rs.svg?branch=master
   :target: https://travis-ci.org/mhallin/vectorclock-rs

A `Vector Clock`_ is a data structure and algorithm for detecting partial ordering of events in
distributed systems. This is an implementation for Rust.

----

Usage
=====

Add ``vectorclock`` to your Cargo.toml:

.. code:: toml

   [dependencies]
   vectorclock = "*"

The data structure is contained in the ``VectorClock<HostType>`` generic struct. You specialize this
struct based on how you identify your processes, via IP addresses, usernames, Uuids, or anything
else.

Look at the tests in clock.rs_ for usage examples.


.. _Vector Clock: http://en.wikipedia.org/wiki/Vector_clock
.. _clock.rs: src/clock.rs
