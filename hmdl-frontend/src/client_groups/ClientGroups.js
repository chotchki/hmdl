import React from 'react';
import useAxios from 'axios-hooks';

import Accordion from 'react-bootstrap/Accordion';
import Alert from 'react-bootstrap/Alert';
import Spinner from 'react-bootstrap/Spinner';

import AddClientGroup from './AddClientGroup.js';
import ClientGroupRow from './ClientGroupRow.js';
import Container from 'react-bootstrap/esm/Container.js';


export function ClientGroups() {
    const [{ data, error, loading }, executeGet] = useAxios("/api/client-groups", "GET");

    if (error) {
        return (
            <Alert key="danger" variant="danger">
                Error: {error.message}
            </Alert>
        );
    } else if (loading) {
        return (
            <Spinner animation="border" role="status">
                <span className="visually-hidden">Loading...</span>
            </Spinner>
        );
    } else if (data.length === 0) {
        return (
            <>
                <Container>
                    <h1>Existing Groups</h1>
                    <p>No groups exist, setup a new one below.</p>
                </Container>
                <Container>
                    <h1>Add New Group</h1>
                    <AddClientGroup refresh={executeGet} />
                </Container>
            </>
        );
    }
    return (
        <>
            <Container>
                <h1>Existing Groups</h1>
                <Accordion>
                    {data.map(group => (
                        <ClientGroupRow key={group} group={group} refresh={executeGet} />
                    ))}
                </Accordion>
            </Container>
            <Container>
                <h1>Add New Group</h1>
                <AddClientGroup refresh={executeGet} />
            </Container>
        </>
    );
}

export default ClientGroups;