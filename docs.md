# Documentation

### console-rs is simple to use. All is in name.

Function name | Arguments | Returns | What does it do?
------------------------------- | ------------- | ------------ | ------------
clear | `std::io::Stdout` | `crossterm::Result<(), std::error::Error>` | Clears the whole console.
write | `T (all things implements std::fmt::Display)` | `crossterm::Result<(), std::error::Error>` | Write something in the terminal without line break.
writeln | `T (all things implements std::fmt::Display)` | `crossterm::Result<(), std::error::Error>` | Write something in the terminal with line break.
write_colored | `T (all things implements std::fmt::Display), termcolor::Color, boolean` | `crossterm::Result<(), std::error::Error>` | Write something in the terminal colored with you color. And, if you add true in the `inline` field, it gonna add a line break.