
Inspired by the Christina Hunger's work to use an AAC button board to teach her dog to talk. (https://hungerforwords.com) This software plays audio clips based on buttons pressed. 

The commonly used buttons by Christina and others are prone to breaking, have poor sound quality, poor volume, and require battery changes. This software and associated hardware setup allows a more compact and portable design that is ideal for cats and smaller dogs. 

The software runs as a simple service in the background on a Raspberry Pi Desktop. The Raspberry Pi interfaces with the buttons using two (or more) USB "Zero-Delay" arcade button controllers for a total of 24+ buttons on one sound board. Additional button controllers, and buttons, are possible but have not be tested. 

![Button Board Prototype](images/board-front.jpg?raw=true "Button Board Prototype")

Installation:

The below install script assumes you are installing on a fresh Raspberry Pi Desktop OS with nothing else. It will enable filesharing for the `pi` user over WiFi. The audio is sent out the headphone jack.

   1. Download software to the `pi` user home folder.
   2. Change to RustButtonBoard directory: `cd /home/pi/RustbuttonBoard`
   3. Run `sudo ./scripts/install.sh`
   4. Add your sound files for the words to: `/home/pi/button-board/vocabulary`
   5. Edit button mapping file to assign your audio files to the buttons: `/home/pi/button-board/words.json`
   6. `sudo reboot` your raspberry pi.
   7. Success! Press your buttons and you should get audio on the headphone out port on your Raspberry Pi.


Reference Hardware:

The selected hardware below was chosen to not require any soldering or wire splicing. All of these parts connect together using preinstalled connectors so the board can be constructed without any specialized equipment. 

        Easyget LED Arcade Controller DIY Kit $37.99 (Two are required)
        https://www.amazon.com/dp/B00WAYVR9M
        https://www.amazon.com/dp/B00WAZK5K8

        CanaKit Raspberry Pi 4 4GB Starter PRO Kit - 4GB RAM $99.99	
        https://www.amazon.com/dp/B07V5JTMV9

        Kinter MA170+ 2-Channel Mini Speaker Amplifier $17.88
        https://www.amazon.com/dp/B07C1Q1FPT

        Poly-Planar MA3013G 3 inch Flush-Mount Speakers $45.71
        https://www.amazon.com/dp/B00197V836

        12V to 5V DC USB Buck Converter $12.99
        https://www.amazon.com/dp/B07SS6XTZY
        
        Audio Ground-Loop Isolator
        https://www.amazon.com/dp/B09B9TS6ZW

        USB Type A to Type C Cable $11.99
        https://www.amazon.com/dp/B01I4ZOIQY

        2 Way Barrel Plug DC Power Splitter Cable 5.5mm x 2.1mm $5.99
        https://www.amazon.com/dp/B01M7N1GOH

        Rankie 3.5mm to Male RCA Adapter Audio Stereo Cable $5.98
        https://www.amazon.com/dp/B071R4R5B8

        Double Sided Tape, Heavy Duty $6.99	
        https://www.amazon.com/dp/B0852XL3CC

        White Nylon Self-Adhesive Twist Lock Cable Ties $13.99
        https://www.amazon.com/dp/B014U91B22

![Button Board Prototype Wiring](images/board-back.jpg?raw=true "Button Board Prototype Wiring")

Holes in the board are cut using the layout template in the `templates` folder of this repository. 
