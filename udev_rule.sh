#!/bin/bash

# create home for rule
touch /lib/udev/allow_write_access_to_brightness.sh

# populate rule
echo chmod u+w /sys/class/backlight/intel_backlight/brightness >>/lib/udev/allow_write_access_to_brightness.sh

# allow execution
chmod +x /lib/udev/allow_write_access_to_brightness.sh

# add rule file to udev
touch /etc/udev/rules.d/allow_write_access_to_brightness.rules

# populate
echo ACTION=="add" RUN+="/lib/udev/allow_write_access_to_brightness.sh" >> /etc/udev/rules.d/allow_write_access_to_brightness.rules
