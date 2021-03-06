============
Game of Life
============

`Conway's Game of Life`_ implementation in Rust programming language.

.. image:: screenshots/gosper-glider-gun.png

How to run?
===========

To compile and run project in development mode use the following command:

.. code-block:: bash

   cargo run

In order to build a release version (with compile-time optimizations) use the
following command:

.. code-block:: bash

   cargo build --release

Controls
========

- Press ``spacebar`` to toggle game mode: editing or simulating
- When in editing mode (the one which is active when the binary is run)
  left-click on the cell to toggle its state.

.. _`Conway's Game of Life`: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

