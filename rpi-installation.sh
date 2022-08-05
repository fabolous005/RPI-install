#!/bin/bash

# Script for RPi-installation
	
# Enter your known  {} 
# the syntax for bash arrays is ARR=('example1' 'example2')
KNOWN_NETWORKS=('')



show_help(){
	#TODO: use cat <<EOF
	#TODO: write help function
}



determine_todo(){
	#Check for Internet connection
	if timeout 10 true >/dev/tcp/8.8.8.8/53; then #Check if connected in this example google-dns
		connection = $(nmcli connection show --active | grep -Eo '^[^ ]+' | grep -Ev '^(NAME$| {} )') 
		# In case of the {} add your known network
		#^^This will grep for all nearby  {}  except your known leave the "NAME$" << because this will be the first line of the output of nmcli
        	if [[ " ${KNOWN_NETWORKS[*]} " =~ " $connection " ]]; then #check if network is known
			LOCATION="office" #set location
		else 
			# TODO: Make this more secure
			LOCATION="home" #set location
		fi
	else #if not connected
		if [ "$(nmcli -f GENERAL.STATE con show {} | grep activated | wc -w)" -eq 0 ]; then # insert your wlan-ssid for {}
            		nmcli connection up {}  # insert your wlan-ssid for {}
        	fi
		#TODO: change to r-c wlan
        	if [ "$(nmcli -f GENERAL.STATE con show {} | grep activated | wc -w)" -eq 0 ]; then  # insert your wlan-ssid for {}
                	nmcli connection up {}   # insert your wlan-ssid for {}
        	fi

		if timeout 10 true >/dev/tcp/8.8.8.8/53; then #Check if connected
			LOCATION="office"
		else
			LOCATION="home"
			#TODO:
			#call function for starting file-listener, webserver and rest-api
		fi
	fi
}



checking_running(){
	#TODO: finish checks
	ps aux | grep #file-listener \| smokeping \| web-server \| rest-api
}



checking_conditions(){
	echo "Installing necessary tools"
	apt install network-manager net-tools nmcli smokeping
	echo "Checking if the hostname is set"
	if ($HOSTNAME !=~ {}) # enter your wanted hostname for the RPI
	then
        	echo "are you shure that the hostname is correct?"
	fi

	echo "Checking if the user is  {} "
	if ($SUDO_USER =~ ^ {} $) # enter your wanted user for the RPI
	then
        	echo "please run this script with sudo but not as root"
	fi
}


configure_network(){
	#TODO: let this get done either by the rust-lib or by this bash-script
	cat >> /etc/NetworkManager/system-connections<< EOF
	 network ={
	  ssid="r-c"
	  [auth]
	  id_str="r-c wlan"
	}
EOF
}



case $1 in

	"--help" | "help")
	show_help
	;;

esac

echo "determine what to do"
determine_todo
echo "checking if services are already running"
checking_running
echo "checking conditions"
checking_conditions
