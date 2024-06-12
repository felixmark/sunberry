# Install all apt packages from apt-packages.txt
xargs sudo apt-get -y install < apt-packages.txt

# Create sunberry folder in etc and set the owner to sunshine
sudo mkdir -p /etc/sunberry
sudo chown sunshine /etc/sunberry
sudo chgrp sunshine /etc/sunberry

# Other stuff
sudo cp motd /etc/motd
