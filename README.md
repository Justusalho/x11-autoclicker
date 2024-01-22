# x11-autoclicker
A simple but effective autoclicker for x11 to be run from the command line

## Installation

To install to _~/.local/bin/_ run:  
``$ cargo install --git "https://github.com/Justusalho/x11-autoclicker.git" --root ${HOME}/.local/``  

You can also pull the files with  
``$ cargo pull https://github.com/Justusalho/x11-autoclicker.git``  
and then compile using cargo:  
``$ cd x11-autoclicker && cargo build --release``  

## Usage

Usage: autoclicker [OPTIONS]

Options:  

**-x**, --mouse-x             Cursor x position. (default: Current position)  
**-y**, --mouse-y             Cursor y position. (default: Current position)  
**-r**, --click-rate          Clicks/second. Must be 0-200. (Set a value between 0 and 1 to have gaps >1s) (default: 5)  
**-b**, --button              Mouse button to click. (default: left)  
**-c**, --click-limit         A limit on the number of total clicks.  
**-t**, --time-limit          A time limit on the clicker.  
**-w**, --wait-period         Time to wait before starting the clicker.  
**-p**, --prevent-movement    Always move the cursor back to given coordinates before clicking. (Needs either a limit or the flag --force)  
**-f**, --force               Force run even with potentially bad options.  
**-v**, --verbose             Print the location of each click.  
**-h**, --help                Show this help message.  
