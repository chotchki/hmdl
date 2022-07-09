# hearthstone-server
Home Web Server


Device connects to network
    Asks for an ip via DHCP

    DHCP looks up the MAC and see if its already named and associated with a group
        (name, mac, group, IP (autogenerated))

        If its not associated, it prompts in the admin interface to be configured

    DHCP provides an unused IP address, gateway and DNS server

Device requests a DNS entry
    DNS server receives request 

    The request is logged into a known DNS table in reverse domain notation if not known.
        known_table (reverse domain syntax, Needs_Review, Kids_Block, Adults_Block, IOT_Block, Servers_Block)

        Known or not is determined by taking the domain, reversing it and seeing if the domain or parts of the domain appears in the table.

        If known, the request checks if the requestor group has the domain blocked. If so a CNAME is returned pointing to "blocked.blocked". (For now so I can tell its happening).

        If not known, we attempt to figure out if it should be auto blocked. Either way its logged and the "needs_review" flag is set.

    If everything passes through this step, the forwarder is used to actually look up the record and return it to the client.



Admin Interface
    Device Mapping
        Unsetup
            ----- Entries -----
            Mac + Form for Name/Group/IP (optional)
        Kids Group
            ----- Entries -----
            Name, Mac, IP + Delete Button
        Adults Group

        IOT

        Servers

    DNS Review
        Needs Review
            ---- Entries -----
            Name, Kids_Block, Adults_Block, IOT_Block, Servers_Block + Save Review

        Known + filter box



Task List (motivation is to put the DHCP as late as possible)
- [x] Edit Group
- [x] Delete Group
- [x] Provide a way to associate domain with a group
- [ ] Manage what a client is and what group its assigned to
- [ ] Find a way to setup a client assignment without DHCP
- [ ] Download the html of a domain
- [ ] Feed the domains of a group and NOT of group into a ML model
- [ ] Setup User Registration
- [ ] Setup First User / Second User
- [ ] Setup User Authentication
- [ ] Block Access to the application if you aren't authenticated
- [ ] Need to fix how the restful urls are structured (single vs plural)


<ListGroup.Item>No Domains</ListGroup.Item>