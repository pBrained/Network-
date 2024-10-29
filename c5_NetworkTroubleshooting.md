# Explain the troubleshooting methodology.
1. Identify the problem
- Gather information
- Question users
- Identify symptoms
- Determine if anything has
changed
- Duplicate the problem, if
possible
- Approach multiple problems
individually
2. Establish a theory of probable
cause
- Question the obvious
- Consider multiple approaches
- - Top-to-bottom/bottom-to-top OSI model
- - Divide and conquer
3. Test the theory to determine the cause
- If theory is confirmed, determine next steps to resolve problem
- If theory is not confirmed, establish a new theory or escalate
4. Establish a plan of action to resolve the problem and identify potential effects
5. Implement the solution or escalate as necessary Verify full system functionality and implement preventive measures if applicable
6. Document findings, actions, outcomes, and lessons learned throughout the process

## Given a scenario, troubleshoot common cabling and physical interface issues.
Cable issues
- Incorrect cable
- - Single mode vs multimode
- - Category 5/6/7/8
- - Shielded twisted pair (STP) vs. vs. unshielded twisted pair (UTP)
- Signal degradation
- - Crosstalk
- - Interference
- - Attenuation
- Improper termination
- Transmitter (TX)/Receiver (RX)
transposed

Interface issues
- Increasing interface counters
- - Cyclic redundancy check (CRC)
- - Runts
- - Giants
- - Drops
- Port status
- - Error disabled
- - Administratively down
- - Suspended

Hardware issues
- Power over Ethernet (PoE)
- - Power budget exceeded
- - Incorrect standard
- Transceivers
- - Mismatch
- - Signal strength

## Given a scenario troubleshoot issues with common network services. 
Switching issues
- STP
- - Network loops
- - Root Bridge Selectiono
- - Port roles
- - Port states
- Incorrect VLAN assignment
- ACLs

Route selection
- Routing table 
- Default routes

Address pool exhaustion

Incorrect default gateway

Incorrect IP address 
- Duplicate IP Address

Incorrect subnet mask

## Given a scenario, troubleshoot common performance issues.

Congestion/contention

Bottlenecking

Bandwidth

- Throughput capacity

Latency

Packet loss

Jitter

Wireless

- Interference
- -  Channel overlap
- Signal degradation or loss
- Insufficient wireless coverage
- Client disassociation issues
- Roaming misconfiguration

## Given a scenario, use the appropriate tool or protocol to solve networking issues.

Software tools
- Protocol analyzer
- Command line
- - ping
- - traceroute/tracert
- - nslookup
- - tcpdump
- - dig
- - netstat
- - ip/ifconfig/ipconfig
- - arp
- Nmap
- Link Layer Discovery Protocol (LLDP)/Cisco Discovery Protocol
(CDP)
- Speed tester

Hardware tools
- Toner
- Cable tester
- Taps
- Wi-Fi analyzer
- Visual fault locator

Basic networking device
commands
- show mac-address-table
- show route
- show interface
- show config
- show arp
- show vlan
- show power
