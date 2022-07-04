import React, { useState } from 'react';
import useAxios from 'axios-hooks';
import { useInterval } from 'usehooks-ts';

import Alert from 'react-bootstrap/Alert';
import Nav from 'react-bootstrap/Nav';
import Spinner from 'react-bootstrap/Spinner';

import Clients from './clients/Clients.js';
import Domains from './domains/Domains.js';
import Groups from './groups/Groups.js';

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