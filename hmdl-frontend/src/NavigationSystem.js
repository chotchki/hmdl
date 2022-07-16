import React, { useState } from 'react';
import useAxios from 'axios-hooks';
import { useInterval } from 'usehooks-ts';

import Alert from 'react-bootstrap/Alert';
import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';
import NavDropdown from 'react-bootstrap/NavDropdown';
import Spinner from 'react-bootstrap/Spinner';

import Clients from './clients/Clients.js';
import ClientGroups from './client_groups/ClientGroups.js';
import Domains from './domains/Domains.js';
import DomainGroups from './domain_groups/DomainGroups.js';

export function NavigationSystem(props) {
    const [{ data, errorHealth, loadingHealth }, executeHealth] = useAxios({ url: "/api/health", method: "GET" }, { manual: true });

    const [isLoading, setLoading] = useState(true);
    const [count, setCount] = useState(0);

    const [mainNav, setMainNav] = useState("loading");

    useInterval(
        () => {
            if (count > 30) {
                setLoading(false);
                setMainNav("error");
            } else if (!loadingHealth && !errorHealth && data === "Ok") {
                setLoading(false);
                setMainNav("domains"); //Eventually this will be login
            } else if (!loadingHealth) {
                setCount(count + 1);
                executeHealth();
            }
        },
        // Run until we have success or failure
        isLoading ? 1000 : null,
    );

    let content;
    if (mainNav === "loading") {
        return (
            <Alert variant='info'>
                <h1>Backend Server is loading</h1>
                <p>Please wait...</p>
                <Spinner animation="border" role="status">
                    <span className="visually-hidden">Loading...</span>
                </Spinner>
            </Alert>
        );
    }

    if (mainNav === "error") {
        return (
            <Alert variant='danger'>
                <h1>The server is not coming up in a reasonable time!</h1>
                <p>Go yell at the admin.</p>
            </Alert>
        );
    }

    if (mainNav === "domains") {
        content = <Domains />;
    } else if (mainNav === "domain-groups") {
        content = <DomainGroups />;
    } else if (mainNav === "clients") {
        content = <Clients />;
    } else {
        content = <ClientGroups />;
    }

    return (
        <>
            <Navbar bg="primary" variant="dark" activeKey={mainNav} defaultActiveKey="domains">
                <Container>
                    <Navbar.Brand onClick={e => setMainNav("domains")}>HMDL</Navbar.Brand>
                    <Navbar.Toggle aria-controls="basic-navbar-nav" />
                    <Navbar.Collapse id="basic-navbar-nav">
                        <Nav className="me-auto">
                            <NavDropdown title="Domains">
                                <NavDropdown.Item eventKey="domains" onClick={e => setMainNav("domains")}>Uncategorized Domains</NavDropdown.Item>
                                <NavDropdown.Item eventKey="domain-groups" onClick={e => setMainNav("domain-groups")}>Domain Groups</NavDropdown.Item>
                            </NavDropdown>
                            <NavDropdown title="Clients">
                                <NavDropdown.Item eventKey="clients" onClick={e => setMainNav("clients")}>Uncategorized Clients</NavDropdown.Item>
                                <NavDropdown.Item eventKey="client-groups" onClick={e => setMainNav("client-groups")}>Client Groups</NavDropdown.Item>
                            </NavDropdown>
                        </Nav>
                    </Navbar.Collapse>
                </Container>
            </Navbar>
            <Container>
                {content}
            </Container>
        </>
    );
}

export default NavigationSystem;