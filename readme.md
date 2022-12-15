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
- This library will be programmed in a strongly typed programming language that supports compilation to WebAssembly such as Typescript or AssemblyScript 
- Development will be test driven. Tests will be designed based upon the MIDI Specification
- Continuous integration platforms will compile all code to WASM modules 
- WASM Modules will be transpiled into readable C to support bare metal implementations 


- Modern programming languages  offer significant value when connected with modern build tools
- Advanced runtime environments enable delivery of binary executables to arbitrary platforms.
    - We can use one codebase and target all platforms and languages


## Why Typescript? 
- Typescript is what is known as a "**transpiler language**". 
- Transpiler languages provides access to a low level functional format that enables them to output code of a different language, to provide cross compatibility with legacy code
- In Typescript this low level format is known as the [Typescript Abstract Syntax Tree](https://ts-ast-viewer.com/#code/FAYw9gdgzgLgBAQwCZLgXjgCgQLggVwFsAjAUwCcAaOYvIs8gSnQD44BvYObuc0mfOQiI4AahoBuYAF8pwZEkwBmAEzUAbABZGEoA)
- Access is provided to the Typescript AST via the [Typescript Compiler API](https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API)

## Why Assemblyscript? 
- Assemblyscrip

## Concerns
- "Javascript is so ugly! its hot garbage on top of cold garbage!" 
    - Agreed! Typescript is not Javascript. Typescript is a superset of the ES6 syntax standard 
    - Coding standards in Typescript are enforced with a `tsconfig.json` file which 
- "Do I need to know typescript to use this library?"
    - To utilize the library, no new languages will be needed. You will be able to simply import the library into your project and instantiate your midi structures. 

## Considerations
- Typescript is not the only language that was considered for this project. 
- There are a number of other Transpiler based langauges that have been gaining popularity, such as ReScript and Elm, however none have gained as much traction as Typescript.
- Typescript is the only modern strongly typed language that has a transpiler project that transpiled properly to human readable, testable C

### Relevant Links

https://petersalomonsen.com/articles/wasm2c/wasm2c.html
https://github.com/wasm3/embedded-wasm-apps

[The Typescript Compiler API](https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API)

[An example of the Typescript Abstract Syntax Tree](https://ts-ast-viewer.com/#code/FAYw9gdgzgLgBAQwCZLgXjgCgQLggVwFsAjAUwCcAaOYvIs8gSnQD44BvYObuc0mfOQiI4AahoBuYAF8pwZEkwBmAEzUAbABZGEoA)

[Typescript to C Transpiler](https://github.com/andrei-markeev/ts2c)

[WASM to C Transpiler](https://github.com/WebAssembly/wabt/tree/main/wasm2c)

[Typescript to Lua Transpiler](https://typescripttolua.github.io/)

[Typescript to WASM runtime](https://deno.land/)


### Potentially Helpful Tools 
#### Build Automation


https://github.com/qilingframework/qiling
https://github.com/nviennot/stm32-emulator
