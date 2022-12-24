# midi2idim
#### midi-2-inter-device-interconnect-manager




# The Future of MIDI
## it's a jungle out there..

- There is currently a wide breadth of MIDI libraries that support MIDI 1.0, and they all have different implementation details
- With the ratification of the MIDI 2.0 standard, MIDI-CI and  the Universal MIDI Packet (MIDI-UMP) enable devices to implement entirely new, and previously unplanned functionality on top of the MIDI 2.0 standard
- For all new functionality, device manufacturers and app designers will need to provide a specification and example code in order to enable new devices to adopt new functionality
- Device manufacturers and app designers who wish to implement new functionality on a platform on which example code is not available will be required to re-implement the functionality specification on their platform.
- With the removal of a electrical transport specification from MIDI 2.0, the number of platforms and architectures that will be capable of supporting MIDI applications has increased by a significant factor
- The breadth of platforms, if coupled with disparate implementations, will result in DIY developers being left in the dust, islands of compatibility between manufacturers, and unforeseen incompatibilities that manufacturers will need to support non technical musicians in diagnosing issues with unknown hardware

## what could we do?
- Implement the fundamental MIDI-CI protocol, as described in the **M2-101-UM MIDI Capabilities Inquiry (MIDI-CI) version 1.1** specification. 
- This library will need to be programmed in a strongly typed programming language that supports compilation compatibility with all endpoints where MIDI2 will be used. 
- Development must be test driven to ensure compatibility with the MIDI Specification, and to ensure cross compatibility between new protocols that are implemented on top of MIDI2


- Modern programming languages offer significant value when connected with modern build tools
- Advanced runtime environments enable delivery of binary executables to arbitrary platforms.
- We can use one codebase and target all platforms and languages

