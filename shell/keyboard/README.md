# Keyboard

The **On-Screen Keyboard** is a **layer shell**-based application, developed using **MCTK**, a GUI library in Rust.

The keyboard enhances the typing experience with word suggestions in the Normal layout, powered by a ranked dictionary, and it dynamically adjusts the touch area for probable next characters. These features ensure an efficient and tailored typing experience across a variety of use cases.

## Features

### Layouts supported

The keyboard will support multiple predefined layouts tailored for various input scenarios:

| **ID** | **Layout**    | **Description**                                                                            | **Status**     |
|--------|---------------|--------------------------------------------------------------------------------------------|----------------|
| 1      | **Normal**    | Default layout for general typing.                                                         | Done           |
| 2      | **Alpha**     | Allows only alphabetic characters.                                                         | InProgress     |
| 3      | **Digits**    | Allows only numeric digits.                                                                | InProgress     |
| 4      | **Number**    | Input a number, including decimal separator and sign.                                      | InProgress     |
| 5      | **Phone**     | Input a phone number.                                                                      | InProgress     |
| 6      | **Url**       | Input a URL.                                                                               | InProgress     |
| 7      | **Email**     | Input an email address.                                                                    | InProgress     |
| 8      | **Name**      | Input a person's name.                                                                     | InProgress     |
| 9      | **Password**  | Input a password (combine with sensitive data or password hints).                          | InProgress     |
| 10     | **Date**      | Input a date.                                                                              | InProgress     |
| 11     | **Time**      | Input a time.                                                                              | InProgress     |
| 12     | **Datetime**  | Input both date and time.                                                                  | InProgress     |
| 13     | **Terminal**  | Input for a terminal environment.                                                          | InProgress     |                                                       |


Each layout adjusts the key arrangements and input handling for its specific purpose, ensuring an optimal user experience.

### Word Suggestions

The **Normal Layout** includes a **word suggestion feature** to enhance typing speed and accuracy. 

- A **dictionary of words with ranks** is used to prioritize word suggestions.  
  Example dictionary snippet:

  | Word | Rank |
  |------|------|
  | you  | 1    |
  | I    | 2    |
  | to   | 3    |
  | the  | 4    |

- **How it works**:
  - As the user types, the keyboard suggests words based on the current input.
  - Words with higher ranks (lower rank number) appear first in the suggestion bar.
  - For instance, typing "y" will suggest "you" as it is ranked highest in the dictionary.

- **Enhanced Text Area**:
  - The size of the keys for possible next characters dynamically increases.
  - For example, if the user types "y," keys for "o" and "u" will expand since they form the word "you."

This feature ensures faster typing and reduces errors by providing relevant suggestions and highlighting the next probable input.


## Local Setup

To set up the Keyboard locally, follow these steps:

### Prerequisites

Ensure you have **Rust** installed on your system. Follow the installation guide for your platform at [Rust Installation Guide](https://www.rust-lang.org/tools/install).

### Installation Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/mecha-org/mechanix-gui.git
   ```

2. Navigate into the keyboard directory:
   ```bash
   cd mechanix-gui/shell/keyboard
   ```

3. Copy and Update settings.yml:
   - Before running the Keyboard, you need to set up the settings.yml file in the Keyboard directory.
   - The settings.yml file is used for configuring various settings and assets for the Keyboard. You can modify this file to customize the keyboard’s appearance and behavior.
   - Copy the settings.yml.example file to settings.yml in the same directory.

   ```bash
   cp settings.yml.example settings.yml
   ```
   - After copying, open `settings.yml` and update the paths accordingly (such as font paths, icon paths, and other resources).
   
   <br />
   ### Example Path Update:
    If the path in `settings.yml` is:
    ```plaintext
    /usr/share/mechanix/shell/keyboard/assets/icons/key_shift_icon.png
    ```
    You should change it to something like:
    ```plaintext
    ./src/assets/icons/key_shift_icon.png
    ```
   - This is necessary to ensure that the paths are relative to your project's directory structure.

4. Run the app:
   ```bash
   cargo run
   ```

## Customizing Layout  

### Overview  
A layout file defines the structure, appearance, and functionality of the On-Screen Keyboard. Each layout is tailored to a specific input type, such as general typing, email addresses, URLs, PINs, or terminal commands.  

With a layout file, you can:  
- **Define Key Placement:** Arrange keys for different contexts (e.g., alphabetic, numeric, symbolic) within the layout.  
- **Set Key Actions:** Configure key behavior, like switching views, entering symbols, or triggering special functions.  
- **Customize Button Styles:** Adjust key sizes, shapes, and icons for an intuitive and accessible interface.  
- **Create Specialized Layouts:** Design unique keyboard layouts for specific tasks, such as email, URL input, or PIN entry.  

Layout files are stored in `src/assets/layouts`, with subdirectories for specialized layouts like `email`, `url`, `pin`, and more.  

### Example

Here's an example of a default alphabetic layout file (`src/assets/layouts/us.yml`):

```yml
---
outlines:
  default: { width: 44, height: 50 }
  altline: { width: 44, height: 50 }
  change-view: { width: 44, height: 50 }
  change-view-2: { width: 44, height: 50 }
  wide: { width: 88, height: 50 }
  spaceline: { width: 260, height: 50 }
  special: { width: 44, height: 50 }
  large: { width: 44, height: 50 }

views:
  base:
    - "q w e r t y u i o p"
    - "a s d f g h j k l"
    - "Shift_L   z x c v b n m  BackSpace"
    - "show_numbers ,         space       . Return"
  upper:
    - "Q W E R T Y U I O P"
    - "A S D F G H J K L"
    - "Shift_L   Z X C V B N M  BackSpace"
    - "show_numbers ,         space       . Return"
  numbers:
    - "1 2 3 4 5 6 7 8 9 0"
    - "! @ # $ % ^ & * ( )"
    - "show_symbols   - ' '' : ; , ?  BackSpace"
    - "show_letters ,         space       . Return"
  symbols:
    - "↑ ↓ ← → e e \\ | { }"
    - "+ ` ~ = / _ < > [ ]"
    - "show_numbers_from_symbols   - ' '' : ; , ?  BackSpace"
    - "show_letters ,         space       . Return"

buttons:
  Shift_L:
    action:
      locking:
        lock_view: "upper"
        unlock_view: "base"
    outline: "change-view"
    icon: "key-shift"
  BackSpace:
    outline: "altline"
    icon: "edit-clear-symbolic"
    action: "erase"
  preferences:
    action: "show_prefs"
    outline: "special"
    icon: "keyboard-mode-symbolic"
  show_numbers:
    action:
      set_view: "numbers"
    outline: "change-view-2"
    label: "1#!"
  show_numbers_from_symbols:
    action:
      set_view: "numbers"
    outline: "change-view"
    label: "2/2"
  show_letters:
    action:
      set_view: "base"
    outline: "change-view-2"
    label: "ABC"
  show_symbols:
    action:
      set_view: "symbols"
    outline: "change-view"
    label: "1/2"
  .:
    outline: "default"
  space:
    outline: "spaceline"
    label: " "
    text: " "
  Return:
    outline: "wide"
    # icon: "key-enter"
    label: "Return"
    action: "enter"
```

## Key Sections  

### 1. Outlines  
Defines the button outline sizes for different buttons across all views. These outlines set the width and height of buttons, allowing for flexible UI adjustments.  

- **default**: Standard button outline.  
- **altline**: Outline for alternative buttons (e.g., Backspace).  
- **change-view**, **change-view-2**: Used for buttons that switch between different views (e.g., Numbers, Letters).  
- **wide**: Wide buttons (e.g., Return).  
- **spaceline**: Outline for the spacebar.  
- **special**: Special buttons like preferences.  
- **large**: Used for larger buttons.  

### 2. Views  
Defines different keyboard views that the user can switch between. Each view represents a different layout or set of keys.  

- **base**: The standard alphabetic layout with a few special keys (Shift, Backspace, Preferences, etc.).  
- **upper**: Uppercase layout for typing capital letters.  
- **numbers**: Numeric layout for numbers and common symbols.  
- **symbols**: Layout for additional symbols and arrow keys.  

### 3. Buttons  
Each button on the keyboard has specific actions associated with it. This section outlines the button definitions, including actions, icons, and the layout outline.  

- **Shift_L**: Locks the keyboard view to uppercase letters and unlocks it back to the base view. Icon: `key-shift`.  
- **BackSpace**: Erases the last character typed. Icon: `edit-clear-symbolic`.  
- **preferences**: Opens the preferences window. Icon: `keyboard-mode-symbolic`.  
- **show_numbers**: Switches to the numbers view. Label: `"1#!"`.  
- **show_numbers_from_symbols**: Switches to the numbers view from the symbols view. Label: `"2/2"`.  
- **show_letters**: Switches back to the base (alphabetic) view. Label: `"ABC"`.  
- **show_symbols**: Switches to the symbols view. Label: `"1/2"`.  
- **space**: The spacebar, with a wide outline and label `" "`.  
- **Return**: The Return (Enter) key, with a wide outline and icon `key-enter`.  

### Button Actions  
Each button in the layout is mapped to an action that determines its functionality.  

- **locking**: Locks or unlocks views.  
- **set_view**: Switches between views (e.g., `"numbers"`, `"symbols"`).  
- **show_prefs**: Opens the preferences window.  
- **erase**: Deletes the last character typed.  

### Iconography  
Each button can have an icon to visually represent its action, which is specified using standard icon names (e.g., `"key-shift"`, `"edit-clear-symbolic"`, etc.).  