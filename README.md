# why is rust so hard 
all i do is dig through some obscure documentation only to realize that that specific crate is built on top of another crate and they just stack on top of one another so I end up having 500 chrome tabs of just crates.io
genuinely just crates.io github some rust book looking websites that are the wiki pages of those crates and more crates.io EVERYTHING IS CRATES.IO its not even that i hate it its just that why are there so many tabs open
on my computer its cluttering up my computer and i'm always confused. Okay for the actual code part i don't really understand why everything is like this like conceptually i understand why i have usb serial wrapped in a 
static mutex refcell option serial port lifetime static usb bus. but why why I would have never actually come up with this and all i really ever do is use the weird free cs closure thingy that is magical that i like don't
really understand but apparently thread safety or something but i'm just trying to get hello world out of my atsamd21g18a-au why does it have to be so difficult. also before trying to use defmt i literally used a macro like
the one you can see in the actual real repo but macros look like actual demon speak like what purpose could ```($($arg:tt)*) => {{``` possibly mean like i haven't even gone to like try to format the out put i mean it looks 
nice that theres colors for debug or info or trace or error or warn but to like actually make it pretty so i can draw like graphs of sensor data is still so far away like i don't know why my barometer thinks i'm underground
it literally just copied the c code over to rust why is it -53562 meters underground i don't understand. i know i already mentioned this but genuinely what could 
```pub static USB_SERIAL: Mutex<RefCell<Option<SerialPort<'static, UsbBus>>>> = Mutex::new(RefCell::new(None));``` possibly be hinting at like sure it makes sense but when i saw this for the first time i actually had a heart
attack. anyway thats my journey so far! yayy i'm having so much fun!
