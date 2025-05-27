# Overview
Described in here is a description of the source modules. 
# Manifest
- ``src``
    - ``entry`` - The entrypoint of the binary. Initializes the overlay from ``src/overlay`` and Waits for the needed modules to be loaded before each ``src/subscribers`` submodule subscribes. Afterwards, spawns the ``src/server``.
    - ``battle`` - The heart of this module is ``BattleContext``, which keeps track of battle events and dispatches the events to clients. ``BattleContext::handle_event`` is only called in ``src/subscribers/battle`` to handle and process the incoming event.
    - ``lib`` - Contains some macros and the address of the necessary module handle in order to initialize this module's own statics.
    - ``logging`` - Just logging implementation for console/file/UI
    - ``models``
        - ``events`` - Contains the ``Event`` variant struct enums that both ``src/subscribers/battle`` and ``src/battle`` uses to handle and process battle events.
        - ``misc`` - Miscellaneous and non-specific data structures. Only used to store data that dispatched packets use.
        - ``packets`` - Contains packets to dispatch to clients. Some packets include ``Heartbeat``, ``Error``, and other various battle events.
    - ``kreide`` - Game integration.
        - ``statics`` - **[IMPORTANT FOR MAINTAINERS]** Needs to be updated.
        - ``helpers`` - Useful helper functions for the ``subscribers``.
        - \.\.Default::default() - Bindings.
    - ``overlay`` - Creates Directx11 overlay.
    - ``server`` - Typical server things. Dispatches heartbeat every second and periodically dispatches game events to every client. 
     - ``subscribers``
        - ``battle``
            - Subscribes to battle events. Take a gander at your own time.
    -  ``ui`` - Overlay powered by [egui](https://github.com/emilk/egui)!