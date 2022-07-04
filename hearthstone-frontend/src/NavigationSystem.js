import React, { useState } from 'react';
import Nav from 'react-bootstrap/Nav';

import Clients from './clients/Clients.js';
import Domains from './domains/Domains.js';
import Groups from './groups/Groups.js';

export function NavigationSystem(props) {
    const [mainNav, setMainNav] = useState("domains");

    let content;
    if (mainNav === "domains") {
        content = <Domains />;
    } else if (mainNav === "groups") {
        content = <Groups />;
    } else {
        content = <Clients />;
    }

    return (
        <>
            <Nav className="justify-content-center navbar navbar-expand-lg bg-light" activeKey={mainNav} onSelect={e => setMainNav(e)}>
                <Nav.Item>
                    <Nav.Link eventKey="domains">Domains</Nav.Link>
                </Nav.Item>
                <Nav.Item>
                    <Nav.Link eventKey="groups">Groups</Nav.Link>
                </Nav.Item>
                <Nav.Item>
                    <Nav.Link eventKey="clients">Clients</Nav.Link>
                </Nav.Item>
            </Nav>
            <div className="container-fluid">
                {content}
            </div>
        </>
    );
}

export default NavigationSystem;