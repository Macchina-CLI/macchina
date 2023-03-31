MACCHINA(7)

# NAME
macchina - theming.

# SYNOPSIS
*$XDG_CONFIG_HOME/macchina/themes*, *~/.config/macchina/themes*.

# DESCRIPTION
Themes are your interface to customizing all visual aspects of macchina. The
following manpage covers everything you need to know to design your own theme.

# GENERAL OPTIONS
General options should be set at the top of the theme's configuration file, as
they do not belong to any particular section.

## spacing
Defines the amount of spacing to leave between
the separator and the content besides it, e.g.:
	
	spacing = 1
	
## padding
Defines the amount of padding to leave between
the content and its surroundings, e.g.:
	
	padding = 0

## hide_ascii
Disables the rendering of ASCII, whether it be
built-in or custom, e.g.:
	
	hide_ascii = false
	
## prefer_small_ascii
For built-in ASCII, always use smaller variants, e.g.:
	
	prefer_small_ascii = true
	
## separator
Defines the glyph to use for the separator, e.g.:
	
	separator = "-->"
	
## key_color
Defines the color of the keys.

Accepts hexadecimal values:

	color = "#00FF00"

Indexed values:

	color = "046"

Predefined color names, where casing is insensitive:

	color = "Green"
	
## separator_color
Defines the color of the separator.
	
Accepts hexadecimal values:

	color = "#00FF00"

Indexed values:

	color = "046"

Predefined color names, where casing is insensitive:

	color = "Green"
	
# PALETTE SECTION
This section, noted *[palette]*, offers a visual component that displays and
represents the active colorscheme of your terminal emulator.

## type
Defines the color set to use for the palette, with possible values of "Dark",
"Light", "Full" (case-sensitive).

## glyph
Defines the glyph to use for the palette, e.g.:

	glyph = "() "

You should append a space to leave some room between the glyphs.

## visible
Defines whether to show or hide the palette, e.g.:
	
	visible = true

# BAR SECTION

This section, noted *[bar]*, replaces data that ranges from 0-100% with bars.

## glyph
Defines the glyph to use for all bars, e.g.:
	
	glyph = "o"

## symbol_open
Defines the character to use for opening delimiters. Be sure
	to surround the value with single quotes and not double quotes, e.g.:

	symbol_open = '('

## symbol_close
Defines the character to use for closing delimiters. Be sure
to surround the value with single quotes and not double quotes, e.g.:

	symbol_close = ')'

## visible
Defines whether to show or hide the bars, e.g.:

	visible = true

## hide_delimiters
Defines whether to show or hide the bars delimiters, i.e.
the characters that surround the bars themselves, e.g.:

	hide_delimiters = false

# BOX SECTION

The section, noted *[box]*, offers a box component which is rendered to surround
your system information.

## title
Defines the title of the box, e.g.:

	title = "Hydrogen"

## border
Defines the type of border to use for the box, with possible values of "plain",
"thick", "rounded" or "double".

## visible
Defines whether to show or hide the box, e.g.:

	visible = true

# BOX.INNER_MARGIN SECTION

## x 
Defines the horizontal margin to leave between
the content and the box, e.g.:
	
	x = 2

## y
Defines the vertical margin to leave between the content and the box, e.g.:
	
	y = 1

# CUSTOM_ASCII SECTION
This section, noted *[custom_ascii]*, allows you to specify your own ASCII art.
ANSI escape sequences are supported.

## color
Defines the color of the ASCII.
	
Accepts hexadecimal values:

	color = "#00FF00"

Indexed values:

	color = "046"

Predefined color names (case-insensitive):

	color = "Green"

## path
Defines the path to a file on your filesystem
which contains the ASCII art you want to display, e.g.:
	
	path = "~/ascii/arch_linux"

# RANDOMIZE SECTION
This section, noted *[randomize]*, is used to randomize color selection.

## key_color
Defines whether to randomize the color of the keys, e.g.:

	key_color = true

## separator_color
Defines whether to randomize the color of the separator, e.g.:

	separator_color = true

## pool
Defines the pool of colors from which to pick a random color, with possible
values of "hexadecimal", "indexed" or "base" (case-insensitive).
	
- If "hexadecimal" is specified, you'll get a random color ranging
from #000000 to #FFFFFF
	
- If "indexed" is specified, you'll get a random color ranging from 0 to 255
	
- If "base" is specified, you'll see a random color from the following set of
colors: "black", "white", "red", "green", "blue", "yellow", "magenta" and
"cyan".

# KEYS SECTION
This section, noted *[keys]*, allows you to modify the text of each key.

For example, the "Processor" readout, which by default shows up as "CPU" in
macchina's output, can be renamed to whatever you like by setting the "cpu"
option.

## host
Defines the text of the Host readout, e.g.:

	host = "Host"

## kernel
Defines the text of the Kernel readout, e.g.:

	kernel = "Kernel"

## os
Defines the text of the OperatingSystem readout, e.g.:

	os = "OS"

## machine
Defines the text of the Machine readout, e.g.:

	machine= "Machine"

## de
Defines the text of the DesktopEnvironment readout, e.g.:

	de = "DE"

## wm
Defines the text of the WindowManager readout, e.g.:

	wm = "WM"

## distro
Defines the text of the Distribution readout, e.g.:
	
	distro = "Distro"

## terminal
Defines the text of the Terminal readout, e.g.:

	terminal = "Term"

## shell
Defines the text of the Shell readout, e.g.:

	shell = "Shell"

## packages
Defines the text of the Packages readout, e.g.:

	packages = "Packages"

## uptime
Defines the text of the Uptime readout, e.g.:

	uptime = "Uptime"

## local_ip
Defines the text of the LocalIP readout, e.g.:

	local_ip = "Local IP"

## memory
Defines the text of the Memory readout, e.g.:

	memory = "Memory"

## battery
Defines the text of the Battery readout, e.g.:
	
	battery = "Battery"

## backlight
Defines the text of the Backlight readout, e.g.:

	backlight = "Brightness"

## resolution
Defines the text of the Resolution readout, e.g.:

	resolution = "Resolution"

## cpu
Defines the text of the Processor readout, e.g.:
	
	cpu = "CPU"

## cpu_load
Defines the text of the ProcessorLoad readout, e.g.:

	cpu_load = "CPU %"

## gpu
Defines the text of the GPU readout(s), e.g.:

	gpu = "GPU"

## disk_space
Defines the text of the disk space readout, e.g.:

	disk_space = "Disk Space"

# SEE ALSO
macchina(1)
