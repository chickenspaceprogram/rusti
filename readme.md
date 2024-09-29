# rusti-tokenizer - A tokenizer for TI-BASIC programs

rusti is currently incomplete and in active development. As such, many of the features promised in this readme are not yet implemented.

## Summary

This library tokenizes a TI-BASIC program (inputted as a string of text), and returns the tokenized program, as well as the length of the tokenized program and the oldest calculator and OS version that will support the program.

It does not create headers for the tokenized program or a complete variable entry for the tokenized data. Those will need to be created separately by the user. 

This library is intended to be used as part of a larger program to compile and decompile variables on TI-8x calculators.

## Acknowledgements/Notes

For information on how to format the headers and actually send the data this program creates to a calculator, check out Merthsoft's [link guide](https://merthsoft.com/linkguide/ti83+/), specifically the pages on [packet formats](https://merthsoft.com/linkguide/ti83+/packet.html) and [file formats](https://merthsoft.com/linkguide/ti83+/fformat.html). 

This library only creates the "variable data" part of the file format listed on the [file formats](https://merthsoft.com/linkguide/ti83+/fformat.html) page. It can tokenize programs, Y-Vars, and strings.

This library uses the [TI-Toolkit Token Sheets](https://github.com/TI-Toolkit/tokens).