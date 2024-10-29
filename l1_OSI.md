# Open Systems Interconnection Reference Model

Model
OSI Protocol suite it is NOT 

There are unique protocol's at every later.
All people seem to need data a processing

# OSI Layer 1 - Physical Symbols we send through the network
Signaling,Cabling,  connectors
If you have a physical layer problem 
Fix your cabling, punch-downs,etc 

Cables, Fitter and the signal itself
fix your cables check loopback tests, test/replace cables, swap adapter cards.

# Data Link layer

DFrame, MAC address, Extended Uniqu idendfifier (EUD-48, EUI- 64), Switch
The basic "language"
the foundation of communication at the data link layer 

Data Link Control (DLC) protocols 
MAC (Media Access Control) address on Ethernet 

The Switching layer. 
OSI Layer 2 Device ***Switch*** 

Layer 3 switch if routing functionality.

# The Network Layer
Ip Address, Router, Packet
The routing layer 
Internet Protocol (IP)
fragments frames to traverse different networks 
192.168.0.1 - 10.22.253.7 

"Anything about routing

##### Networking Devices, 

***Router***
***OSI Layer 3 Device***
***Routes Traffic Between IP Subnets***

*Firewalls, Most can be layer 3* 
- NGFW vs. Traditional 
- Encrypt traffic - VPN 
- Network Address Translation (NAT)*
- Dynamic Routing

*IDS / IPS* 
- Work with firewalls, 
- Watch Network Traffic
- Intrusions
- Exploits against operation systems. 
- Buffer overflows cross-site scripting
- Detection - Alarm pr alert
- Prevention


*Load Balancers* 
- *Multiple Servers*
- *Invisible in end to end*
- *large Scale Implementations*
- *webserver farms , Database farms*
- *Fault Tolerance*
- *Server outages have no effect*
- *Very fast convergence*
- Configurable Loads
- TCP Offload - Protocol Overhead 
- SSL offload
- Encrypting/ Decryption
- Caching Fast Response
- Prioritization - QOS
- Application Centric - Exclusive routing



*Proxies* 
*Run and return on the users behalf Keep the user safe*
*Useful for caching information, access control, URL filtering, content scanning.*

NAS vs. SAN 
Network  Attached Storage (NAS)
- Connect a shared storage device across the network
- File-level access 

*Storage Area Network* (SAN)
- Looks and feels like a local storage device
- Block Level access Change just the blocks that have been modified very efficient. [

***BOTH USE ALOT OF BANDWITH NAS & SAN***
Access Point (AP)

# Transport Layer 



The ability to transport information from one device to the other 
From one side of the network to the other
Control Protocols, tunneling protocols.

# Presentation Layer
Character encoding.
Application encryption (SSL/TLS).
Often combines with Application Layer.
Control, protocols tunneling protocols.

# Application Layer 

The Layer We See, the layer we interact with 
Https, FTP , DNS, POP3

Real world OSI model. 
Your eyes are the communication?

---

# Wireshark

You can use wireshark to ge different information in the frame and you can see a hexadecimal break down oft he data itself, 
there are 5 lines of data in th eframe, 
each protocols are used, 

1. 2005 bytes on the wire can be the first layer.
2. Ethernet Can be the second.
3. Internet protocol, src, etc can be the third.
4. Transmission control Protocol could be the fourth.
5. Secure socket layer can be the fifth.

![Screenshot of the layers in wireshark](./Screenshots/Screenshot\%202024-10-27\%20at\%204.38.09 AM.png)



