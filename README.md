# Spongebab
If you somehow ended up here looking for a useful tool, turn back now.  If you're like me, completely new to rust, and just want to see some really simple Rust code that does something, I guess you might find something here, TBD.

### WHY?

I've heard so many good things about Rust, I decided it was time to take a test-drive.  My goals were simple: "learn" the language as quickly as possble, and attempt to write anything that does something.  In the process, I hoped to see how long it actually takes to learn enough Rust to do more than "Hello World."  It's all very.... scientific...

### HOW?

I took roughly a weekend to get through https://doc.rust-lang.org/rust-by-example/, and that was with no coding at all, just reading through it, sometimes quickly and sometimes slowly.  After that, the race was on to write something before everything I read fell out of my head.  It took roughly another weekend to write the abomination found in this repo.  A significant amount of time was lost getting used to how module importing works, and a little time was lost until I realized that `ructc` and `crate build` were putting my binaries in different places.


### Conclusion

Is Rust "easy" to learn?  I.e., is it a language that can be picked up quickly?  I was surprised to find it much more approachable than I expected.  What looked like gibberish when I started seemed far more intuitive by the time I reached the end of the examples, though I'd say some of the examples were a bit poor in that they didn't clearly illustrate the concepts.  All in all, I'd say _possibly_ easy to learn the basics but hard to master, which was better than I expected.

In the end, the module system was friendlier than it initially appeared, the messages from the compiler were immensely helpful, available crates (like clap) were nice, and there really wasn't anything too unfamiliar in the language aside from some of the syntax, though I was able to get away with lifetime elision everywhere since this is just a handful of LOC.



## Q & A:

##### Q: So, what is this?

A: Spongebab is an ARP sponge, but it's a really bad one.  ARP sponges are something one might use to reduce noise in their network due to devices ARP'ing for things that aren't there.  Most ARP sponges have things like thresholds and limits so that they only "sponge" up requests for devices that likely don't exist.  Spongebab does not do that.  It has only one mission in life: respond to ARP requests immediately, aggressively, and unceasingly.

##### Q: Is this something I should run in my network?
A: NO.  A real ARP sponge will help reduce unwanted network traffic.  Spongebab will magnify it for literally no good reason.

##### Q: Can you put this in a docker?
A: No.  Well, _you_ can if _you_ want, but just don't.

##### Q: This code is just awful.
A: Yes.


