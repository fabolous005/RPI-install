# RPI-install
RaspberryPi install for Plug and Play smokeping environment in local office and homeoffice


MODIFY ALL VARIABLES (COMMENTED) BEFORE EXECUTING THE rpi-install.sh SCRIPT

rpi-install.sh will determine what and if a job has to start.

in case the RPI is in the office it should automaticly connect (if configured) and recheck if connected.
in case the RPI is at the users home the script will start the file-listener, from the file-listener a web-server and a rest-api will be started.
The web-server gives the user the ability to enter their ssid and psk for local wifi, which will be send to the rest-api.
When the file-listener detects a change, it will try to render a network-manager config with the provided credentials.

The script (that should be ran as a cron-job) will try to connect to newly configured network-manager networks.
If this happens successfully the script will finally start the smokeping.

In case of an error the user will have the ability to enter a new ssid and psk and the script will try to use the new config.
