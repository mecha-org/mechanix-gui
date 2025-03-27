# XDG Desktop portals

XDG Desktop portals, are portals for sandboxed applications to interact with the system resources. Some examples of sandboxed applications: Browser, non native applications, etc.

So XDG Desktop portals also have a frontend and a backend, Frontend being the portal itself. While its the backend where desktop-specific implementation of each interfaces are written.

## Configuration

First of all create a `mechanix.portal` file at `/usr/share/xdg-desktop-portal/portals/`

Then add this content to that file.

```bash
DBusName=org.mechanix.services
Interfaces=org.freedesktop.impl.portal.FileChooser;org.freedesktop.impl.portal.Access;org.freedesktop.impl.portal.Settings;org.freedesktop.impl.portal.Wallpaper;org.freedesktop.impl.portal.Notification;
UseIn=gnome;sway;xfce;

```

This file is for registering our mechanix portal, and describing what all Interfaces mechanix portal has. As this config file describes, it has interfaces: FileChooser, Access, Settings, Wallpaper, Notification. And we are also specifying that this portal should be used in gnome, sway or xfce.

Now the `org.freedesktop.impl.portal.FileChooser` is the default interface where all of the calls for FileChooser will be called. We’re writing this interface to this config file because, we need our portal to override the FileChooser portal that was there already in the OS.

Now lets create `portals.conf`

```bash
[preferred]
org.freedesktop.impl.portal.FileChooser=mechanix;
org.freedesktop.impl.portal.Settings=mechanix;
org.freedesktop.impl.portal.Wallpaper=mechanix;
org.freedesktop.impl.portal.Notification=mechanix;
org.freedesktop.impl.portal.Access=mechanix;
default=gnome;gtk;

```

Now this file is used to set any preferences over the portals. As I said earlier, we want our portal to override the default portals for FileChooser, Settings, Wallpaper, Notification, Access. So this file does that.

To check whether our config file is configured as we expected, we can run this command.

```bash
 /usr/libexec/xdg-desktop-portal -v
```

## Production Configuration

If we are ready to push our portal to prod, we could do these steps to start our portal on startup.

First of all compile the code,

```bash
cargo build --release
```

then move the executable `xdg-desktop-portal-mechanix` to `/usr/libexec/`

Then create a .service file at `/etc/systemd/system` , in this case `xdg-desktop-portal-mechanix.service` and add this content to it. For more info check [here](https://www.shellhacks.com/systemd-service-file-example/)

```bash
[Unit]
Description=Mechanix File Chooser Portal
After=network.target

[Service]
ExecStart=/usr/libexec/xdg-desktop-portal-mechanix
Restart=always

[Install]
WantedBy=default.target
```

and then create a service file at `/usr/share/dbus-1/service/` called `org.mechanix.services.service` whose contents are:

```bash
[D-Bus Service]
Name=org.mechanix.services.FileChooser
Exec=/usr/libexec/xdg-desktop-portal-mechanix
SystemdService=xdg-desktop-portal-mechanix.service
```

also if we want to stop/restart our portal we could run this command:

```bash
systemctl --user stop xdg-desktop-portal-mechanix.service
```

or

```bash
systemctl --user restart xdg-desktop-portal-mechanix.service
```

## Development

The current implementation used [zbus](https://docs.rs/zbus/latest/zbus/) crate to develop custom backends for portals.

`src/interfaces` is where all the methods of interfaces are developed.

```bash
.
└── src
    ├── connections.rs
    ├── gui
    │   ├── file_chooser_handler.rs
    │   ├── mod.rs
    │   └── xdg_portal_handler.rs
    ├── interfaces
    │   ├── access.rs
    │   ├── file_chooser.rs
    │   ├── mod.rs
    │   ├── notification.rs
    │   ├── settings.rs
    │   └── wallpaper.rs
    └── main.rs
```

[`connection.rs`](http://connection.rs) : This file contains interface implementation for `Request` ,`Session` deserialize for `Response` , its better to not edit this file anymore.

The portal has 3 types of Responses, `Success`, `Cancelled` ,`Other` each of them can be signified as 0,1,2  respectively, each of the int value will be signalled as a reponse to the request. More info on this [here](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html).

When a portal Request is called, a Request Object is created at an object path, this object handle will be of the format : **`/**org**/**freedesktop**/**portal**/**desktop**/**request**/**SENDER**/**TOKEN`
where SENDER is the callers unique name, with the initial ':' removed and all '.' replaced by '_', and TOKEN is a unique token that the caller provided with the handle_token key in the options vardict.

for eg: /org/freedesktop/portal/desktop/request/1_169/ashpd_xUb2m99YBD

the `/gui` contains `xdg_desktop_portal.rs` where code for an app without any gui is written, and messages are handled accordingly.
For eg: When a FIieChooser.OpenFile is called, we should send message about rendering gui for the filepicker, right now this directory is not integrated into the program.

`/interfaces` contains custom backend impls of interfaces:

### [Access](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Access.html)

This backend can be used by portal implementations that need to ask a direct access question, such as “May xyz use the microphone?”

This portal can be tested via this command:

```bash
gdbus call --session --dest org.mechanix.services --object-path /org/freedesktop/portal/desktop --method org.freedesktop.impl.portal.Access.AccessDialog "/test/handle"     "test_app"     "test_id"    "title: <Test Title>" "subtitle" "body': <'Test Body'>"    "{}"

```

It has method AccessDialog

```bash
AccessDialog (
  IN handle o,
  IN app_id s,
  IN parent_window s,
  IN title s,
  IN subtitle s,
  IN body s,
  IN options a{sv},
  OUT response u,
  OUT results a{sv}
)
```

Supported keys in the `options` include:

- `modal` (`b`)

    Whether to make the dialog modal. Defaults to true.

- `deny_label` (`s`)

    Label for the Deny button.

- `grant_label` (`s`)

    Label for the Grant button.

- `icon` (`s`)

    Icon name for an icon to show in the dialog. This should be a symbolic icon name.

- `choices` (`a(ssa(ss)s)`)

    List of serialized choices. See [org.freedesktop.portal.FileChooser.OpenFile](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html#org-freedesktop-portal-filechooser-openfile) for details.


The following results get returned via the `results` vardict:

- `choices` (`a(ss)`)

    An array of pairs of strings, corresponding to the passed-in choices. See [org.freedesktop.portal.FileChooser.OpenFile](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html#org-freedesktop-portal-filechooser-openfile) for details.


**handle** Object path to export the Request object at

**app_id** App id of the application

**parent_window** Identifier for the application window, see [Window Identifiers](https://flatpak.github.io/xdg-desktop-portal/docs/window-identifiers.html)

**title** Title for the dialog

**subtitle** Subtitle for the dialog

**body** Body text, may be “”

**options**

Vardict with optional further information

**response** Numeric response. The values allowed match the values allowed for [org.freedesktop.portal.Request::Response](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html#org-freedesktop-portal-request-response) signal.

**results** Vardict with the results of the call



### [FileChooser](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html)

The FileChooser portal allows sandboxed applications to ask the user for access to files outside the sandbox. The portal backend will present the user with a file chooser dialog.

Eg: Browser asking to upload a file/saving a file
Testing: Ashpd, File Upload/Save a file in browser (make sure to use GTK_USE_PORTAL=1 as by default firefox/other browsers built with GTK uses in built filechooser instead of the one given by the portal)

It has methods: OpenFile, SaveFile, SaveFiles

```bash
OpenFile (
  IN handle o,
  IN app_id s,
  IN parent_window s,
  IN title s,
  IN options a{sv},
  OUT response u,
  OUT results a{sv}
)
```

Presents a file chooser dialog to the user to open one or more files.

Supported keys in the `options` vardict include:

- `accept_label` (`s`)

    The label for the accept button. Mnemonic underlines are allowed.

- `modal` (`b`)

    Whether to make the dialog modal. Default is yes.

- `multiple` (`b`)

    Whether to allow selection of multiple files. Default is no.

- `directory` (`b`)

    Whether to select for folders instead of files. Default is to select files.

- `filters` (`a(sa(us))`)

    A list of serialized file filters. See [org.freedesktop.portal.FileChooser.OpenFile](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html#org-freedesktop-portal-filechooser-openfile) for details.

- `current_filter` (`(sa(us))`)

    Request that this filter be set by default at dialog creation. See [org.freedesktop.portal.FileChooser.OpenFile](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html#org-freedesktop-portal-filechooser-openfile) for details.

- `choices` (`a(ssa(ss)s)`)

    A list of serialized combo boxes. See [org.freedesktop.portal.FileChooser.OpenFile](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html#org-freedesktop-portal-filechooser-openfile) for details.

- `current_folder` (`ay`)

    A suggested folder to open the files from. See [org.freedesktop.portal.FileChooser.OpenFile](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html#org-freedesktop-portal-filechooser-openfile) for details.


The following results get returned via the `results` vardict:

- `uris` (`as`)

    An array of strings containing the uris of the selected files. All URIs must have the `file://` scheme.

- `choices` (`a(ss)`)

    An array of pairs of strings, corresponding to the passed-in choices. See [org.freedesktop.portal.FileChooser.OpenFile](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html#org-freedesktop-portal-filechooser-openfile) for details.

- `current_filter` (`(sa(us))`)

    The filter that was selected. See [org.freedesktop.portal.FileChooser.OpenFile](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html#org-freedesktop-portal-filechooser-openfile) for details.

- `writable` (`b`)

    Whether the file is opened with write access. Default is `false`.


**handle** Object path for the [Request](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.Request.html#org-freedesktop-impl-portal-request) object representing this call
**app_id** App id of the application
**parent_window** Identifier for the application window, see [Window Identifiers](https://flatpak.github.io/xdg-desktop-portal/docs/window-identifiers.html)
**title** Title for the file chooser dialog
**options** Vardict with optional further information
**response**Numeric response**results**Vardict with the results of the call

### [Notification](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Notification.html)

This notification interface lets sandboxed applications send and withdraw notifications. Even though there is already a Notification portal, this is an xdg desktop portal which means that, this is specially made for sandboxed applications like browsers, flatpak apps,etc

This has methods: AddNotification, RemoveNotification

This portal can be tested via [ASHPD](https://github.com/kvvarun-mecha/ashpd-test/)

```bash
AddNotification (
  IN app_id s,
  IN id s,
  IN notification a{sv}
)
```

The format of the serialized notification is a vardict, with the following supported keys, all of which are optional:

- `title` (`s`)

    User-visible string to display as the title.

    This should be a short string, if it doesn’t fit the UI it may be truncated to fit on a single line.

- `body` (`s`)

    User-visible string to display as the body.

    This can be a long string but if it doesn’t fit the UI it may be wrapped or/and truncated.

- `markup-body` (`s`)

    The same as `body` but with support for markup formatting. The markup is XML-based and supports a small subset of HTML `<b>...</b>`, `<i>...</i>` and `<a href="...">...</a>`.

    Any markup not supported, e.g. new lines, will be removed from the string. In the future, the set of supported markup may be extended.

    This can be a long string but if it doesn’t fit the UI it may be wrapped or/and truncated.

- `icon` (`v`)

    A serialized icon to add to the notification. The icon must pass [icon validation](https://flatpak.github.io/xdg-desktop-portal/docs/icons.html) in order to be used. The format for serialized icon is a tuple (sv) with the following supported keys:

    - `themed` (`as`)

        A themed icon containing an array of strings with the icon names.

        This is the same format as a serialized [GThemedIcon](https://docs.gtk.org/gio/class.ThemedIcon.html) at the moment, but this interoperability may change in the future.

    - `bytes` (`ay`)

        Since version 2, this is deprecated and should not be used. Please use the themed or file-descriptor option to set an icon.

        This is the same format as a serialized [GBytesIcon](https://docs.gtk.org/gio/class.BytesIcon.html) at the moment, but this interoperability may change in the future.

    - `file-descriptor` (`h`)

        A file descriptor to an image file in png, jpeg or svg form. The file-descriptor used needs to be sealable, currently this is only possible for file descriptors created with `memfd_create()` with the `MFD_ALLOW_SEALING` flag set.


    For historical reasons, it is also possible to send a simple string for themed icons with a single icon name.

    There may be further restrictions on the supported kinds of icons.

- `sound` (`v`)

    A serialized sound to add to the notification. Supported sound formats are ogg/opus, ogg/vorbis and wav/pcm.

    The format for serialized sound is a tuple (sv) with the following supported keys:

    - `file-descriptor` (`h`)

        A file descriptor to a sound file. The file-descriptor used needs to be sealable, currently this is only possible for file descriptors created with `memfd_create()` with the `MFD_ALLOW_SEALING` flag set.


    To play the default sound the string `default` can be passed. To play no sound at all the string `silent` can be passed. If this property isn’t specified the notification server can decide whether to play a sound.

    There may be further restrictions on the supported kinds of sounds.

- `priority` (`s`)

    The priority for the notification. Supported values:

    - `low`
    - `normal`
    - `high`
    - `urgent`
- `default-action` (`s`)

    Name of an action that is exported by the application. This action will be activated when the user clicks on the notification.

- `default-action-target` (`v`)

    Target parameter to send along when activating the default action.

- `buttons` (`aa{sv}`)

    Array of serialized buttons to add to the notification. The format for serialized buttons is a vardict with the following supported keys:

    - `label` (`s`)

        User-visible label for the button. Mandatory, if no purpose is specified. It is strongly recommended to always provide sensible label. Buttons without a `label` are ignored by the server when it doesn’t understand the `purpose` or is needed to display the button.

    - `action` (`s`)

        Name of an action that is exported by the application. The action will be activated when the user clicks on the button. Mandatory.

    - `target` (`v`)

        Target parameter to send along when activating the action.

    - `purpose` (`s`)

        The `purpose` of the button. This information may be used by the notification server to treat the button specially.

        Depending on the `purpose` other fields of the button may be ignored. If the server doesn’t understand the `purpose` it will be ignored and the button will be shown as a normal button.

        Most standardized hints are defined as part of a `category`. Additional purposes may be defined by notification servers using `x-vendor` prefix e.g. `x-gnome.class.specific`

        The following purposes are defined outside of a `category`:

        - `system.custom-alert`:

            Not a button in a strict sense. This action may be called, depending on system policies, automatically by the notification server whenever the notification is shown.

            This allows apps to use custom methods for notifying the user, for example, to play audio from a special source like a streaming service or a radio station.

            No `label` should be given when this purpose is used, so that the server can ignore the button if it doesn’t understand the purpose.

- `display-hint` (`as`)

    An array of ways to display the notification. If none are set, or the notification server has its own policy, it is free to decide how and where to display the notification.

    - `transient`

        The notification is displayed only as a banner and won’t be kept by the server in a tray.

        It’s a programmer error to specify `tray` at the same time.

    - `tray`

        No banner for the notification will be displayed and the notification is placed in the tray.

        It’s a programmer error to specify `transient` at the same time.

    - `persistent`

        Make the notification persistent in the notification tray. The user can’t dismiss it using the usual close button or gesture.

        Apps are only allowed to display persistent notifications as long as they have a window. Once the last window of an app is closed the persistent notification will be removed.

    - `hide-on-lockscreen`

        Don’t show the notification on the lockscreen.

    - `hide-content-on-lockscreen`

        All content of the notification will be hidden on the lockscreen.

    - `show-as-new`

        If a notification with the same `id` of the app exists already replace the previous notification, by removing the old notification (including animation, etc) and adding a new notification.

        If this hint isn’t specified the notification’s content is updated without any flickering.

- `category` (`s`)

    The `category` describes the content of a notification. A notification server may use this information to display the notification specially. Some categories also include button purposes that can be set for a button so that the notification can know the purpose of the button.

    Additional categories and button purposes may be defined by notification servers using `x-vendor` prefix e.g. `x-gnome.class.specific`

    The following categories are standarized so far:

    - `im.received`

        Intended for instant messaging apps displaying notifications for new messages.

        This category has the following button purposes:

        - `im.reply-with-text`:

            Inline replies for instant messaging. The user-provided text will be added to the response.

            The user response (`s`) will be placed as the second value in the parameter array of exported actions. For non-exported actions it will be placed as the third value in the parameter array of [org.freedesktop.portal.Notification::ActionInvoked](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Notification.html#org-freedesktop-portal-notification-actioninvoked).

    - `alarm.ringing`

        Intended for alarm clock apps

    - `call.incoming`

        Intended for call apps to notify the user about an incoming call.

        This category has the following button purposes:

        - `call.accept`:

            Accept the incoming call.

        - `call.decline`:

            Decline the incoming call.

    - `call.ongoing`

        Intended for call apps while a call is ongoing.

        This type has the following button purposes:

        - `call.hang-up`:

            Hang up the ongoing call.

        - `call.enable-speakerphone`:

            Enable the speakerphone for the ongoing call.

        - `call.disable-speakerphone`:

            Disable the speakerphone for the ongoing call.

    - `call.unanswered`

        Intended to be used by call apps when a call was missed.

    - `weather.warning.extreme`

        Intended to be used to display an extreme weather warning.

    - `cellbroadcast.danger.extreme`

        Intended to be used to display extreme danger warnings broadcasted by the cell network.

    - `cellbroadcast.danger.severe`

        Intended to be used to display severe danger warnings broadcasted by the cell network.

    - `cellbroadcast.amber-alert`

        Intended to be used to display amber alerts broadcasted by the cell network.

    - `cellbroadcast.test`

        Intended to be used to display tests broadcasted by the cell network.

    - `os.battery.low`

        Intended to be used to indicate that the system is low on battery.

    - `browser.web-notification`

        Intended to be used by browsers to mark notifications send by websites via the [Notifications API](https://developer.mozilla.org/en-US/docs/Web/API/Notifications_API).


**id** Application-provided ID for this notification

**notification** Vardict with the serialized notification

### Settings

This interface provides read-only access to a small number of standardized host settings required for toolkits similar to XSettings. It is not for general purpose settings.

For eg: Choosing Default mode in browser themes section, (Light/Dark/Default) will call this portal.

This portal can be tested via [ASHPD](https://github.com/kvvarun-mecha/ashpd-test/), also by selecting Default settings in Browser themes. (In light/dark mode option).
Methods: ReadAll, ReadOne

```bash
ReadAll (
  IN namespaces as,
  OUT value a{sa{sv}}
)

ReadOne (
  IN namespace s,
  IN key s,
  OUT value v
)

```

If `namespaces` is an empty array or contains an empty string it matches all. Globbing is supported but only for trailing sections, e.g. “org.example.*”.

**namespaces** List of namespaces to filter results by, supports simple globbing explained below.

**value** Dictionary of namespaces to its keys and values.

**key** The key to get.

**value** The value `key` is set to.

### Wallpaper

This simple interface lets sandboxed applications set the user’s desktop background picture. For eg: installing a new wallpaper app, setting wallpaper via that app.

This portal can be tested via [ASHPD](https://github.com/kvvarun-mecha/ashpd-test/), also by installing a new wallpaper app like [Nostalgia]((https://gitlab.gnome.org/bertob/nostalgia))
 And using this command:
 ```
 gdbus call --session --dest org.mechanix.services --object-path /org/freedesktop/portal/desktop --method org.freedesktop.impl.portal.Wallpaper.SetWallpaperFile "/test/handle" "test_app" 10  "{'show-preview': <true>, 'set-on': <'both'>}"
Methods: SetWallpaperURI, SetWallpaperFile
```

```bash
SetWallpaperURI (
  IN parent_window s,
  IN uri s,
  IN options a{sv},
  OUT handle o
)

SetWallpaperFile (
  IN parent_window s,
  IN fd h,
  IN options a{sv},
  OUT handle o
)
```

Supported keys in the `options` vardict include:

- `show-preview` (`b`)

    Whether to show a preview of the picture. Note that the portal may decide to show a preview even if this option is not set.

- `set-on` (`s`)

    Where to set the wallpaper. Possible values are `background`, `lockscreen`, or `both`.


**parent_window** Identifier for the application window, see [Window Identifiers](https://flatpak.github.io/xdg-desktop-portal/docs/window-identifiers.html)

**uri** The picture file uri

**options** Options that influence the behavior of the portal

**handle** Object path for the [Request](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html#org-freedesktop-portal-request) object representing this call

Inspiration for most of the code is from: [Cosmic](https://github.com/pop-os/xdg-desktop-portal-cosmic/), [Luminous](https://github.com/waycrate/xdg-desktop-portal-luminous/), [Ashpd](https://github.com/bilelmoussaoui/ashpd/)

## Running, Testing, Debugging

For running the program, we could just  `cargo run` but make sure that there are no services running on the same address, ie the xdg-desktop-portal-mechanix.service is not running.

We can make sure our portal is up and running, by these commands:

```bash
busctl --user introspect org.mechanix.services /org/freedesktop/portal/desktop
```

Then for testing the program, we can use a library called [ashpd](https://github.com/bilelmoussaoui/ashpd/). ASHPD can be considered as a client side application that calls the portal, The code for tests are available [here](https://github.com/kvvarun-mecha/ashpd-tets).

also we could run this command:

```bash
gdbus call --session --dest org.mechanix.services --object-path /org/freedesktop/portal/desktop --method org.freedesktop.impl.portal.Access.AccessDialog "/test/handle"     "test_app"     "test_id"    "title: <Test Title>" "subtitle" "body': <'Test Body'>"    "{}"
```

this is a demo call of Access portal, with relevant params passed, if you want a gui version of this, you could install [dspy](https://apps.gnome.org/Dspy/).

So for debugging/logging you could use these commands:

```bash
 dbus-monitor --session "interface=org.freedesktop.portal.Access”

```

or simply `dbus-monitor` to monitor all dbus logs.

we could also use

```bash
journalctl --user -u xdg-desktop-portal-mechanix
```

or simply `journalctl -xe`  or this.

```bash
G_MESSAGES_DEBUG=all /usr/libexec/xdg-desktop-portal -r
```

### Now what all work is done, what all are pending.

What is done,

All the portals written, makes sure control flows to our portals itself, whenever each of the portal is called our code is run.

What is pending,

Access: GUI for asking for access, giving access.

FileChooser: GUI for picking a file/saving a file

Notifications: Send notifications received to the notification display.

Settings: We need a store to hold data that is to be returned, like accent-color, color-scheme, contrast, appearance.

Wallpaper: We need to write logic for changing the wallpaper, when a request is given.
