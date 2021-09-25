# Simple FOF synthesizer
Written in rust.
This is for me to experiment with this synthesis technique because it's a quite cool one

### FOF?
French abbreviation for "Fonction d'onde formantique" (Formant wavefunction synthesis)
It consists of sine pulses with an attack and decay envelope, pulsed at the base frequency
The sine itself resets per pulse and is at the frequency of the formant.
Decay factor is exp(-bandwidth * pi / sample rate)

Sources: 
 - https://ccrma.stanford.edu/~serafin/320/lab3/FOF_synthesis.html

### What will this be?
A simple FOF synthesizer, although it won't do it as originally.
Instead, it will probably use some form of bandpass filter to achieve the same result as FOF.

### Goals
 - Don't require multiple FOF pulses to be computed at the same time
 - Be fast
 - try and fit within [-1, 1] sample amplitude (or lower than that)
 - Also do noise (although this will get closer to a regular bandpass filter than FOF)

### Licence
The unlicense, do what you want with this