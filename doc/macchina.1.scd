MACCHINA(1)

# NAME

macchina - A system information frontend, with an (unhealthy) emphasis
on performance.

# SYNOPSIS

*macchina* [FLAGS]

# FLAGS

*-d, --doctor*
	Checks the system for failures.

*-e, --export-config*
	Prints a template configuration file to stdout.

*-c, --config*=_FILE_
	Specify a custom path to a configuration file.

	This is helpful in cases where macchina fails to parse the configuration
	file from the default path, i.e. *$XDG_CONFIG_HOME/macchina/macchina.toml*.

*-l, --list-themes*
	Lists all available themes.

	Themes are *TOML* files which must be placed in
	*$XDG_CONFIG_HOME/macchina/themes*.  Whatever this flag
	returns to the console can be used with the *--theme* flag.

*-t, --theme*=_THEME_
	Specify the name of the theme to use, i.e. its basename and
	without the ".toml" extension.

	You should verify whether macchina was able to find your theme
	with *--list-themes*.

*-i, --interface*=_IF_NAME_
	Specify the network interface for the LocalIP readout, e.g. "wlan0", "eth0".

*-s, --current-shell*
	Toggles between the current shell, i.e. the parent of the terminal emulator, or the default one.

*-C, --logical-cores*
	Toggles between logical and physical cores for the Processor readout.

*-K, --long-kernel*
	Lengthens kernel output.

*-S, --long-shell*
	Lengthens shell output.

*-U, --long-uptime*
	Lengthens uptime output.

*-m, --memory-percentage*
	Show memory usage in percentage

*-p, --disk-space-percentage*
	Show disk space usage in percentage

*-h, --help*
	Prints help information.

*-v, --version*
	Prints version information.

*-o, --show*
	Display only the specified readouts.

	Please note that the order these are listed in will be the order that they are
	displayed in.

	Possible values are (case-sensitive):
	- Host
	- Machine
	- Kernel
	- Distribution
	- OperatingSystem
	- DesktopEnvironment
	- WindowManager
	- Packages
	- Shell
	- Terminal
	- LocalIP
	- Backlight
	- Resolution
	- Uptime
	- Processor
	- ProcessorLoad
	- Memory
	- Battery
	- GPU
	- DiskSpace

*--ascii-artists*
	Lists the original artists of the ASCII art used by macchina.

# SEE ALSO

macchina(7)

# AUTHORS

Written and maintained by the Macchina-CLI team:
- Taha Aziz Ben Ali <ba.tahaaziz@gmail.com>
- Uttarayan Mondal <email@uttarayan.me>
- Marvin Haschker <marvin@haschker.me>

# RESOURCES

This project is hosted at *https://github.com/Macchina-CLI/macchina*.
