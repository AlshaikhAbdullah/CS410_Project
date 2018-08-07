## A Guessing game 


## Installation :

The **gtk** crate expects **GTK+**, **GLib** and **Cairo** development files to be installed on your system.

**For Debian**
```
sudo apt-get install libgtk-3-dev
```
**For fedora**
```
sudo dnf install gtk3-devel glib2-devel
```
To compile and run
```
cargo run 
```
##  About this game :

 - This is just a simple gtk rs based guessing game .  The Game start
   and Randomize a I32 variable **X**.  So when user type an integer >
   **X**  then event is fired and an text box setTetx() is changed to **TOO big**
 - Then if user input is less then **X**  then changes value of text field to Too small .

So using basic equality rules , User guess **the actual value ** using uper limit and lower limit of the variable 

## How did it went  ?
Firstly , I didn't go as planned . I had alot of problems with libraries that wasn't build passing . So I had to stick to **rand**  crate . 

Firstly I was going to use Glade prototyping my application then I changed my mind , go for hardcoding each label , button in the rust code . 

## Review about gtk rs

 - Since gtk rs dosn't uses any strict rules about ownership and Borrowing . The lib handles these situation itself aslo I didn't have to write a 100 lines of gtk code. So , I was happy that majority of the trait impl I had to do using Rust . 



