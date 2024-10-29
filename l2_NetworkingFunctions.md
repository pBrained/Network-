
# Networking Functions
- Trafic management 
- Protocol Support
- Remote access - Securely 

### Content Delivery Network
Geographically distributed caching servers

### Virtual Private network (VPN)
- Usually use a concentrator / head-end to be the central connection point for all users, this is usually a purpose built appliance to do live high speed encryption and decryption of network data in real time so you can have many people being able to send secure data across the network decrypt all of the data send it to the inside of the network and then revers that process to get the response to the user. 

- Use a purpose built appliance for very high throughput. 
- Smaller may not need concentrator
- Software install may be necessary

# (QoS) Quality of Service 
- Traffic shaping, packet shaping
- Set important Applications
- Firewall router or switch configurations you can set your on priority of applications or add applications. 

# Time to live
- How long should data be available
- Not all protocols are self regulating. 
- Number of iterations or number of hops to stop then (drop) 
- Many differnet uses, 
- Clear a cache or drop a packet stuck in a loop

# Routing Loops

Router A thinks its going to A and Router B things its going to A
- One IP Address mistake in static route
- Use a TTL field inside of the packet 
- IP Protocol has TTL For this exact reason. 

- Linux / MacOS TTL is 64*
- Windows TTL is 128
- The router decreases the ttl by one

(Internet Protocol) 

```bash
Time to live: 58
```

## DNS (Domain Name System)
- TTL IS DIFFERENT
- TTL IS NOW ABOUT SECONDS 
- Resolve a address from a fully qualifies domain name www.x.com - 120.23.23.45
- A device stashes a lookup for a certain amount of time. 

