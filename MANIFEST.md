# Overview
Described in here is a description of the source modules. 
# Manifest
- ``src``
    - ``entry`` - The entrypoint of the binary. Waits for the needed modules to be loaded before each ``src/subscribers`` submodule subscribes. Afterwards, spawns the ``src/server``.
    - ``battle`` - The heart of this module is ``BattleContext``, which keeps track of battle events and dispatches the events to clients. ``BattleContext::handle_event`` is only called in ``src/subscribers/battle`` to handle and process the incoming event.
    - ``lib`` - Contains some macros and the address of the necessary module handle in order to initialize this module's own statics.
    - ``models``
        - ``events`` - Contains the ``Event`` variant struct enums that both ``src/subscribers/battle`` and ``src/battle`` uses to handle and process battle events.
        - ``misc`` - Miscellaneous and non-specific data structures. Only used to store data that dispatched packets use.
        - ``packets`` - Contains packets to dispatch to clients. Some packets include ``Heartbeat``, ``Error``, and other various battle events.
    - ``kreide`` - Game integration.
        - ``statics`` - **[IMPORTANT FOR MAINTAINERS]** Needs to be updated.
        - ``helpers`` - Useful helper functions for the ``subscribers``.
        - \.\.Default::default() - Bindings.
    - ``server`` - Typical server things. Dispatches heartbeat every second and periodically dispatches game events to every client. 
     - ``subscribers``
        - ``battle``
            - Subscribes to battle events. Take a gander at your own time.
        - ``directx``
            -  Subscribes to the Directx 11 screen drawing event. Not intrinsic to the game itself. Utilizes a forked ``egui-directx11`` backend created/updated/inspired by ``unknowntrojan``, ``sy1ntexx``, and ``ohchase``. The implementation is a little rusty, so might cause intermittent and random crashes, but it is what allows the egui overlay in ``src/ui`` to render on the screen.
    -  ``ui`` - Overlay powered by [egui](https://github.com/emilk/egui)!