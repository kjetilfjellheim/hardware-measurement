# hardware-measurement

## Description
The target of this application is to make possible to control and and get measurements from several instruments. This application will 
primarily be for Linux users, but might work on other systems. I will only be creating it for instruments I have and use.

## Goals (Initial goals)
- [x] print measurement from Uni-T 161D multimeter to terminal (USB, HID)
- [x] Control Peaktech 4055mv (USB)
- [ ] print measurements from Sigilent RSDS 1204X-E oscilloscope (USB, SCPI)
- [ ] print measurements from Brymen 869 multimeter (USB, HID)
- [ ] print measurements from Sigilent RSDS 1204X-E oscilloscope (LAN, SCPI)
- [ ] collect measurements from multiple instruments
- [ ] create command sets to define when to get measurements
- [ ] save measurements to file

## Example commands Uni-T 161D
sudo ./target/debug/hardware-measurement --device=unit161d --hid=/dev/hidraw6 --commands=Measure
sudo ./target/debug/hardware-measurement --device=unit161d --hid=/dev/hidraw6 --commands=MinMax

## Example commands Peaktech 4055mv
sudo ./target/debug/hardware-measurement --device=peaktech4055mv --commands="Apply:Sin, 10kHz, 3, 0.4" --usb=17224:21815
sudo ./target/debug/hardware-measurement --device=peaktech4055mv --commands="Reset" --usb=17224:21815
sudo ./target/debug/hardware-measurement --device=peaktech4055mv --commands="Raw:Apply:Sin, 10kHz, 3, 0.4" --usb=17224:21815
