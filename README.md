# Restruct-Studio

A standalone enhancement for Construct 2, an intuitive game engine with block-based code. Some of us would simply rather type our code, and have auto-completion.
The end goal of this project is to allow anyone to do just that. Import a Construct 2 project, access or create an event sheet, and type out as much code as we would like.

## Modules

- `restruct_studio` will be the IDE and the frontend for users. I haven't started it yet, but it should be GUI-based, intuitive, and platform-accessible. I would like to have it run mainly in the browser with WASM so that anyone can use it without having to install or build the entire suite.
- `restruct_lang` will contain all the logic to "fill in the blanks" between my language and the intermediate representation provided by `restruct_serialization`.
- `restruct_serialization` is the first module under Restruct-Studio. It will parse any existing event sheet XML into intermediate representation for use by the rest of the suite, and it will also be able to convert that representation back into XML. I would like to be able to get the output to be as close to the input as possible, so I will be implementing some tests that make sure conversion is 1-to-1 and the output XML is equal to the input XML.

## Links
[tosh](https://github.com/tjvr/tosh2) is the main inspiration for this project. It is a text-based language and editor for the language Scratch, also block-based.\
[Construct 2](https://www.construct.net/en/construct-2/download) is the game engine I am looking to ~~re-construct~~ implement. While block-based code is perfectly fine for people of all ages and skill levels, I don't think it would hurt to have a second option.\
[regex101](https://regex101.com/) and other regex "IDE's" are very helpful for writing code highlighting patterns, especially for those who aren't great with regex to begin with.