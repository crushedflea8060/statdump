# Statdump - a linux larping tool and hardware larping tool
This tool is intended for the less hardcore larpers out there who only care about their hardware and not their OS! Unfortunately, Linux runs on basically anything, so nobody really cares about their hardware, but still! With the simple run of `statdump` you can see your CPU, Disk. GPU, and RAM information almost immediately.
## Dependencies and Disclaimers
 - Of course, this project requires Rust to be installed.
 - To get GPU Info, you will need the `pciutils` package. Otherwise, you won't be able to use the GPU Part.
 - Again, This is not intended for Windows. I may create a version for windows, but this was intended as a personal project. Reading system files is a lot easier on Linux than it is on Windows.
## Uses
 - This is an easy tool to allow you to get system information without the use of `sudo` or complex tools. It shows more hardware information than `neofetch` and less OS info.
