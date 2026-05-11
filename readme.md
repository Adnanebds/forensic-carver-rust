# Rust File Carver data recovery (learning project)

This is a project I built to learn how data recovery actually works. I used it to pull a deleted JPEG of Luffy (u can choose any image) out of a 1GB virtual disk on my windows machine lol (.vhd). Note its a learning project not a production ready tool

## What it does
Most programs load a whole file into RAM, but that's impossible if the file is 100GB. This tool uses a **BufReader** to look at the disk in small 4KB chunks. It acts like a "State Machine" to find the "DNA" of a JPEG. It looks for a signature start and end of a jpeg format file. Its very simple.

## The Logic 
The program scans for two specific byte sequences:
1.  **Start:** `0xFF 0xD8` (The beginning of a JPEG)
2.  **End:** `0xFF 0xD9` (The end of a JPEG)

### How I handled the "Middle"
I used a `is_carving` flag (a state) to remember if I'm currently inside a file. 
* **If** I find the start -> Start writing.
* **Else if** I find the end -> Stop writing.
* **Else if** I'm already carving -> Keep writing the whole buffer.


## What I learned
* How to use `std::io::BufReader` for high-performance I/O.
* Forensics basics: Deleting a file doesn't erase the bytes, it just hides them.

## How to test this quickly?
* U can make a virtual disk vhd in disk management 
* add jpg image to it and then delete it!
* the resolved.jpg eventually built by the program should show that image! (this only works for 1 image, so lets say u have two images in disk and u delete both i wouldnt know which will be shown to be fair)

## Current Limitations 
* If a JPEG header is split exactly between two 4096-byte reads, it might miss it.
* Only recovers one file (it'll overwrite `resolved.jpg` if it finds another).
